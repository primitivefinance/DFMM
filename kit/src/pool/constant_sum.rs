use bindings::{constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver};

use self::bindings::constant_sum_solver::ConstantSumParams;
use super::*;

#[derive(Clone, Debug)]
pub struct ConstantSumPool {
    pub strategy_contract: ConstantSum<ArbiterMiddleware>,
    pub solver_contract: ConstantSumSolver<ArbiterMiddleware>,
    pub parameters: ConstantSumParams,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ConstantSumAllocationData {
    pub reserve_x: eU256,
    pub reserve_y: eU256,
}

#[async_trait::async_trait]
impl PoolType for ConstantSumPool {
    type Parameters = ConstantSumParams;
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

    async fn get_init_data(
        params: Self::Parameters,
        allocation_data: &Self::AllocationData,
        solver_contract: &Self::SolverContract,
    ) -> Result<Bytes> {
        let init_bytes = solver_contract
            .get_initial_pool_data(allocation_data.reserve_x, allocation_data.reserve_y, params)
            .call()
            .await?;
        Ok(init_bytes)
    }

    fn set_controller(mut params: Self::Parameters, controller: eAddress) -> Self::Parameters {
        params.controller = controller;
        params
    }

    fn get_strategy_address(strategy_contract: &Self::StrategyContract) -> eAddress {
        strategy_contract.address()
    }

    fn create_instance(
        strategy_contract: Self::StrategyContract,
        solver_contract: Self::SolverContract,
        parameters: Self::Parameters,
    ) -> Self {
        Self {
            strategy_contract,
            solver_contract,
            parameters,
        }
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

    async fn update_data(&self, parameters: Self::Parameters) -> Result<Bytes> {
        let price_update_data = self
            .solver_contract
            .prepare_price_update(parameters.price)
            .call()
            .await?;
        Ok(price_update_data)
    }

    async fn change_allocation_data(
        &self,
        _pool_id: eU256,
        _allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        todo!()
        // match allocation_data {
        //     ConstantSumAllocationData::GivenX(amount_x) => {
        //         let data = self
        //             .solver_contract
        //             .prepare_allocation_delta_given_delta_x(pool_id,
        // amount_x)             .call()
        //             .await?;
        //         Ok(data)
        //     }
        //     ConstantSumAllocationData::GivenY(amount_y) => {
        //         let data = self
        //             .solver_contract
        //             .prepare_allocation_delta_given_delta_y(amount_y)
        //             .call()
        //             .await?;
        //         Ok(data)
        //     }
        // }
    }
}
