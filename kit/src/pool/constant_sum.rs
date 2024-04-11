use std::sync::Arc;

use bindings::{
    constant_sum::ConstantSum,
    constant_sum_solver::{ConstantSumParams, ConstantSumSolver},
    shared_types::InitParams,
};
use ethers::etherscan::Client;
use tracing::{debug, info};

use self::behaviors::deployer::DeploymentData;
use super::*;

#[derive(Clone, Debug)]
pub struct ConstantSumPool {
    pub strategy_contract: ConstantSum<ArbiterMiddleware>,
    pub solver_contract: ConstantSumSolver<ArbiterMiddleware>,
    pub parameters: ConstantSumParams,
}

// Configuration for the pool
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstantSumInitData {
    pub name: String,
    pub symbol: String,
    pub reserve_x: eU256,
    pub reserve_y: eU256,
    pub token_x_name: String,
    pub token_y_name: String,
    pub params: ConstantSumParams,
}

pub enum ConstantSumAllocationData {
    GivenX(eU256),
    GivenY(eU256),
}

// TODO: It's worth thinking about what this is since we have our own "Pool"
// struct pub struct Pool {
//     pub strategy: ::ethers::core::types::Address,
//     pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
//     pub reserves: ::std::vec::Vec<::ethers::core::types::U256>,
//     pub total_liquidity: ::ethers::core::types::U256,
//     pub liquidity_token: ::ethers::core::types::Address,
//     pub fee_collector: ::ethers::core::types::Address,
//     pub controller_fee: ::ethers::core::types::U256,
// }

#[async_trait::async_trait]
impl PoolType for ConstantSumPool {
    type PoolParameters = ConstantSumParams;
    type InitializationData = ConstantSumInitData;
    type StrategyContract = ConstantSum<ArbiterMiddleware>;
    type SolverContract = ConstantSumSolver<ArbiterMiddleware>;
    type AllocationData = ConstantSumAllocationData;

    fn get_contracts(
        deployment: &DeploymentData,
        client: Arc<ArbiterMiddleware>,
    ) -> (Self::StrategyContract, Self::SolverContract) {
        (
            ConstantSum::new(deployment.constant_sum, client.clone()),
            ConstantSumSolver::new(deployment.constant_sum_solver, client),
        )
    }

    async fn create_pool(
        init_data: Self::InitializationData,
        token_list: Vec<ArbiterToken<ArbiterMiddleware>>,
        strategy_contract: Self::StrategyContract,
        solver_contract: Self::SolverContract,
        dfmm: DFMM<ArbiterMiddleware>,
    ) -> Result<Pool<Self>> {
        let init_bytes = solver_contract
            .get_initial_pool_data(
                init_data.reserve_x,
                init_data.reserve_y,
                init_data.params.clone(),
            )
            .call()
            .await?;
        debug!("Got init bytes {}", init_bytes);

        let tokens: Vec<eAddress> = token_list.iter().map(|tok| tok.address()).collect();
        assert!(tokens.len() == 2);
        assert!(tokens[0] != tokens[1]);
        let init_params = InitParams {
            name: init_data.name,
            symbol: init_data.symbol,
            strategy: strategy_contract.address(),
            tokens,
            data: init_bytes,
            fee_collector: eAddress::zero(),
            controller_fee: eU256::zero(),
        };
        // let (id, _reserves, _total_liquidity) =
        // dfmm.init(init_params.clone()).send().await?;
        let thing = dfmm.init(init_params.clone()).send().await?.await?.unwrap();
        let thing1 = thing.status.unwrap();
        debug!("tx succeeded with status {}", thing1);
        // debug!("got pool id {}", id);

        let instance = ConstantSumPool {
            strategy_contract,
            solver_contract,
            parameters: init_data.params,
        };

        Ok(Pool {
            id: eU256::one(),
            dfmm,
            instance,
            token_x: token_list[0].clone(),
            token_y: token_list[1].clone(),
        })
    }

    async fn swap_data(
        &self,
        pool_id: eU256,
        input_token: InputToken,
        amount_in: eU256,
    ) -> Result<Bytes> {
        let (valid, _, data) = match input_token {
            InputToken::TokenX => {
                self.solver_contract
                    .simulate_swap(pool_id, true, amount_in)
                    .call()
                    .await?
            }
            InputToken::TokenY => {
                self.solver_contract
                    .simulate_swap(pool_id, false, amount_in)
                    .call()
                    .await?
            }
        };
        if valid {
            Ok(data)
        } else {
            anyhow::bail!("swap was invalid!")
        }
    }

    async fn update_data(&self, parameters: Self::PoolParameters) -> Result<Bytes> {
        let price_update_data = self
            .solver_contract
            .prepare_price_update(parameters.price)
            .call()
            .await?;
        Ok(price_update_data)
    }

    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        match allocation_data {
            ConstantSumAllocationData::GivenX(amount_x) => {
                let data = self
                    .solver_contract
                    .prepare_allocation_delta_given_delta_x(pool_id, amount_x)
                    .call()
                    .await?;
                Ok(data)
            }
            ConstantSumAllocationData::GivenY(amount_y) => {
                let data = self
                    .solver_contract
                    .prepare_allocation_delta_given_delta_y(amount_y)
                    .call()
                    .await?;
                Ok(data)
            }
        }
    }
}
