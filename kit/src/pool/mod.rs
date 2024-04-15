// Notes:
// Idea is that we want to be able to configure behaviors that depend on the
// `PoolType` generic from the config.toml. What this means is that `PoolType`
// itself has to be `Deserialize`able and this is kinda tough to work with.
// ---->>> The reason why is because we can't `Deserialize` contract objects
// themselves because that's just not possible.

use std::sync::Arc;

use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::Bytes;
use serde::{Deserialize, Serialize};

use self::{
    behaviors::deployer::DeploymentData,
    bindings::{erc20::ERC20, shared_types},
};
use super::*;
use crate::bindings::{arbiter_token::ArbiterToken, dfmm::DFMM, shared_types::InitParams};

pub mod constant_sum;
pub mod geometric_mean;
// pub mod log_normal;
// pub mod n_token_geometric_mean;

// Notes:
// `InitData` is something that all pools need  in order to be created. This
// consists of:
// 1. The parameters of the pool which, for example, are like the `mean` and
//    `width` of the `LogNormal` pool. (Strategy specific since other pools
//    might have different params like `ConstantSum` has `price`)
// 2. Initial allocation data, which consists of, for example, a `price` and an
//    amount of `token_x` for the `LogNormal` pool. (Strategy specific since
//    other pools like `ConstantSum` may not have the same needs)
// 3. Base configuration which ALL pools share as part of their parameterization
//    which is the `swap_fee`, `controller` and the `controller_fee`. Every type
//    of strategy needs these.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct InitData<P: PoolType> {
//     pub params: P::PoolParameters,
//     pub initial_allocation_data: P::InitialAllocationData,
//     pub base_config: BaseParameters,
// }

// Notes:
// These are the things that all strategies need to have to be initialized (and
// potentially updated).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub name: String,
    pub symbol: String,
    pub swap_fee: eU256,
    pub controller_fee: eU256,
}

// Notes:
// All the other types will be specific to each pool/strategy type since those
// will be specific contracts
#[async_trait::async_trait]
pub trait PoolType: Clone + std::fmt::Debug + 'static {
    // This trait provides the interface for people to construct pools from a
    // `Configuration` state since all of this should be `Serialize` and
    // `Deserialize`. This stuff ultimately will be what's used to deploy a
    // `Pool<P: PoolType>` which will hold onto actual instances of contracts
    // (whereas this just holds config data).
    type Parameters: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;
    // ~~ These are the contracts that are used to interact with the pool. ~~
    type StrategyContract;
    type SolverContract;
    type AllocationData: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;

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
        base_config: &BaseConfig,
        params: &Self::Parameters,
        allocation_data: &Self::AllocationData,
        solver_contract: &Self::SolverContract,
    ) -> Result<Bytes>;

    fn create_instance(
        strategy_contract: Self::StrategyContract,
        solver_contract: Self::SolverContract,
        parameters: Self::Parameters,
    ) -> Self;
}

pub enum UpdateParameters<P: PoolType> {
    PoolParameters(P::Parameters),
    Controller(eAddress),
    Fee(eU256),
}

// Notes:
// This is used in the `swap_data` function of the poolType trait to determine
// which token to swap in.
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

#[derive(Debug)]
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
        let data =
            P::get_init_data(&base_config, &params, &allocation_data, &solver_contract).await?;
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
        let instance = P::create_instance(strategy_contract, solver_contract, params);
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
    pub async fn allocate_or_deallocate(
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
        self.dfmm.update(self.id, data).send().await?.await?;
        Ok(())
    }
}
