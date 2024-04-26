use std::sync::Arc;

use arbiter_core::middleware::ArbiterMiddleware;
use ethers::{abi::AbiDecode, types::Bytes};
use serde::{Deserialize, Serialize};
use tracing::warn;

use self::{
    behaviors::deploy::DeploymentData,
    bindings::{
        arbiter_token::ArbiterToken, dfmm::DFMM, erc20::ERC20, i_strategy::IStrategy, shared_types,
        shared_types::InitParams,
    },
};
use super::*;
use crate::bindings::dfmm;

pub mod constant_sum;
// pub mod geometric_mean;
// pub mod log_normal;
// pub mod n_token_geometric_mean;

// Notes:
// These are the things that all strategies need to have to be initialized (and
// potentially updated).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub name: String,
    pub symbol: String,
    pub swap_fee: eU256,
    pub controller: eAddress,
    pub controller_fee: eU256,
}

// Notes:
// All the other types will be specific to each pool/strategy type since those
// will be specific contracts
#[async_trait::async_trait]
pub trait PoolType: Debug {
    type Parameters: Clone
        + Debug
        + Send
        + for<'de> Deserialize<'de>
        + Serialize
        + ethers::abi::AbiDecode;
    type StrategyContract: Send;
    type SolverContract: Send;
    type AllocationData: Clone + Debug + Send + for<'de> Deserialize<'de> + Serialize;

    async fn swap_data(&self, pool_id: eU256, swap: InputToken, amount_in: eU256) -> Result<Bytes>;
    /// Change Parameters
    async fn update_data(&self, new_data: Self::Parameters) -> Result<Bytes>;
    /// Change Allocation Date
    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes>;

    fn get_contracts(
        deployment: &DeploymentData,
        client: Arc<ArbiterMiddleware>,
    ) -> (Self::StrategyContract, Self::SolverContract);

    fn get_strategy_address(strategy_contract: &Self::StrategyContract) -> eAddress;

    async fn get_init_data(
        params: Self::Parameters,
        allocation_data: &Self::AllocationData,
        solver_contract: &Self::SolverContract,
    ) -> Result<Bytes>;

    fn set_controller(params: Self::Parameters, controller: eAddress) -> Self::Parameters;

    fn create_instance(
        strategy_contract: Self::StrategyContract,
        solver_contract: Self::SolverContract,
        parameters: Self::Parameters,
    ) -> Self;

    async fn get_params(
        strategy_address: eAddress,
        client: Arc<ArbiterMiddleware>,
        pool_id: eU256,
    ) -> Self::Parameters {
        let i_strategy = IStrategy::new(strategy_address, client);
        let bytes = i_strategy.get_pool_params(pool_id).await.unwrap();
        Self::Parameters::decode(bytes).unwrap()
    }
}

pub enum UpdateParameters<P: PoolType> {
    PoolParameters(P::Parameters),
    Controller(eAddress),
    Fee(eU256),
}

// Notes:
// This is used in the `swap_data` function of the poolType trait to determine
// which token to swap in.
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum InputToken {
    TokenX,
    TokenY,
}

// Notes:
// This is used in the `change_allocation_data` function of the Pool to
// determine whether to allocate or deallocate.
pub enum AllocateOrDeallocate {
    Allocate,
    Deallocate,
}

#[derive(Debug, Clone)]
pub struct Pool<P: PoolType> {
    pub id: eU256,
    pub dfmm: DFMM<ArbiterMiddleware>,
    pub instance: P,
    pub tokens: Vec<ArbiterToken<ArbiterMiddleware>>,
    pub liquidity_token: ERC20<ArbiterMiddleware>,
}

impl<P: PoolType> Pool<P> {
    pub async fn new(
        base_config: BaseConfig,
        params: P::Parameters,
        allocation_data: P::AllocationData,
        strategy_contract: P::StrategyContract,
        solver_contract: P::SolverContract,
        dfmm: DFMM<ArbiterMiddleware>,
        tokens: Vec<ArbiterToken<ArbiterMiddleware>>,
    ) -> Result<Self> {
        // This seems a little hacky but might be the way forward untill we get the
        // contract changes settled in the future we should see if we can work
        // with Matt and Clement to remove the controller from the init params
        let params_with_controller = P::set_controller(params, base_config.controller);

        let data = P::get_init_data(
            params_with_controller.clone(),
            &allocation_data,
            &solver_contract,
        )
        .await?;
        debug!("Got init data {:?}", data);
        let init_params = InitParams {
            name: base_config.name,
            symbol: base_config.symbol,
            strategy: P::get_strategy_address(&strategy_contract),
            tokens: tokens.iter().map(|t| t.address()).collect(),
            data,
            fee_collector: eAddress::zero(),
            controller_fee: eU256::zero(),
        };

        let (id, _reserves, _total_liquidity) = dfmm.init(init_params.clone()).call().await?;
        dfmm.init(init_params).send().await?.await?;
        let pool: shared_types::Pool = dfmm.pools(id).call().await?;
        let instance =
            P::create_instance(strategy_contract, solver_contract, params_with_controller);
        let client = dfmm.client();

        let pool = Self {
            id,
            dfmm,
            instance,
            tokens,
            liquidity_token: ERC20::new(pool.liquidity_token, client),
        };
        info!("Pool created!\n {:#?}", pool);
        Ok(pool)
    }

    /// Performs a swap on the pool.
    ///
    /// # Arguments
    ///
    /// * `amount_in` - The amount of tokens to swap in.
    /// * `token_in` - The type of token to swap in (either TokenX or TokenY).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the swap is successful, otherwise returns an error.
    pub async fn swap(&self, amount_in: eU256, token_in: InputToken) -> Result<()> {
        let data = match token_in {
            InputToken::TokenX => {
                self.instance
                    .swap_data(self.id, token_in, amount_in)
                    .await?
            }
            InputToken::TokenY => {
                self.instance
                    .swap_data(self.id, token_in, amount_in)
                    .await?
            }
        };
        self.dfmm
            .swap(self.id, self.dfmm.address(), data)
            .send()
            .await?
            .await?;
        Ok(())
    }

    /// Performs an allocation or deallocation on the pool.
    ///
    /// # Arguments
    ///
    /// * `action` - The type of action to perform (either Allocate or
    ///   Deallocate).
    /// * `allocation_data` - The allocation data to use for the action.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the allocation or deallocation is successful,
    /// otherwise returns an error.
    pub async fn change_allocation(
        &self,
        action: AllocateOrDeallocate,
        allocation_data: P::AllocationData,
    ) -> Result<()> {
        let data = self
            .instance
            .change_allocation_data(self.id, allocation_data)
            .await?;
        match action {
            AllocateOrDeallocate::Allocate => {
                self.dfmm.allocate(self.id, data).send().await?.await?
            }
            AllocateOrDeallocate::Deallocate => {
                self.dfmm.deallocate(self.id, data).send().await?.await?
            }
        };
        Ok(())
    }

    /// Updates the pool with new data.
    ///
    /// # Arguments
    ///
    /// * `new_data` - The new data to update the pool with.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update is successful, otherwise returns an
    /// error.
    pub async fn update(&self, new_data: P::Parameters) -> Result<()> {
        let data = self.instance.update_data(new_data).await?;
        info!("Got update data");
        let tx = self.dfmm.update(self.id, data);
        let tx_result = tx.send().await;
        match tx_result {
            Ok(_) => {}
            Err(er) => match er.as_middleware_error().unwrap() {
                arbiter_core::errors::ArbiterCoreError::ExecutionRevert {
                    gas_used: _,
                    output,
                } => {
                    let error = dfmm::DFMMErrors::decode(output);
                    warn!("Contract reverted with error: {:?}", error);
                }
                _ => todo!(),
            },
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct PoolCreation<P: PoolType> {
    pub id: eU256,
    pub tokens: Vec<eAddress>,
    pub liquidity_token: eAddress,
    #[serde(bound(deserialize = "P::Parameters: Deserialize<'de>"))]
    pub params: P::Parameters,
    #[serde(bound(deserialize = "P::Parameters: Deserialize<'de>"))]
    pub allocation_data: P::AllocationData,
}

#[derive(Debug, Clone, State)]
pub struct PoolProcessing<P>
where
    P: PoolType,
{
    pub pool: Pool<P>,
    pub client: Arc<ArbiterMiddleware>,
    pub messager: Messager,
}
