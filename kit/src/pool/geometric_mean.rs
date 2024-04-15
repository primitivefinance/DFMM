use bindings::{geometric_mean::GeometricMean, geometric_mean_solver::GeometricMeanSolver};
use ethers::types::Address;

use self::bindings::geometric_mean_solver;
use super::*;

#[derive(Clone, Debug)]
pub struct GeometricMeanPool {
    pub strategy_contract: GeometricMean<ArbiterMiddleware>,
    pub solver_contract: GeometricMeanSolver<ArbiterMiddleware>,
    pub parameters: GeometricMeanParams,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct GeometricMeanParams {
    pub target_weight_y: eU256,
    pub target_weight_x: eU256,
}

pub enum UpdateParameters {
    SwapFee(eU256),
    TargetController(Address),
    TargetTimestamp(eU256),
    TargetWeightX(eU256),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct GeometricMeanAllocationData {
    pub amount_x: eU256,
    pub price: eU256,
}

#[async_trait::async_trait]
impl PoolType for GeometricMeanPool {
    type Parameters = GeometricMeanParams;
    type StrategyContract = GeometricMean<ArbiterMiddleware>;
    type SolverContract = GeometricMeanSolver<ArbiterMiddleware>;
    type AllocationData = GeometricMeanAllocationData;

    async fn swap_data(
        &self,
        pool_id: eU256,
        input_token: InputToken,
        amount_in: eU256,
    ) -> Result<Bytes> {
        let (valid, _, data) = match input_token {
            InputToken::TokenX => {
                self.solver_contract
                    .simulate_swap(pool_id, eU256::zero(), eU256::one(), amount_in)
                    .call()
                    .await?
            }
            InputToken::TokenY => {
                self.solver_contract
                    .simulate_swap(pool_id, eU256::one(), eU256::zero(), amount_in)
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

    async fn update_data(&self, _new_data: Self::Parameters) -> Result<Bytes> {
        // let data = match new_data.update_parameters {
        //     UpdateParameters::SwapFee(fee) => {
        //         self.solver_contract.prepare_fee_update(fee).call().await?
        //     }
        //     UpdateParameters::TargetTimestamp(timestamp) => {
        //         self.solver_contract
        //             .prepare_weight_x_update(timestamp, timestamp)
        //             .call()
        //             .await?
        //     }
        //     UpdateParameters::TargetWeightX(weight_x) => {
        //         self.solver_contract
        //             .prepare_weight_x_update(weight_x, weight_x)
        //             .call()
        //             .await?
        //     }
        //     UpdateParameters::TargetController(controller) => {
        //         self.solver_contract
        //             .prepare_controller_update(controller)
        //             .call()
        //             .await?
        //     }
        // };
        // Ok(data)
        todo!()
    }

    async fn change_allocation_data(
        &self,
        _pool_id: eU256,
        _allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        todo!()
    }

    fn get_contracts(
        deployment: &DeploymentData,
        client: Arc<ArbiterMiddleware>,
    ) -> (Self::StrategyContract, Self::SolverContract) {
        (
            GeometricMean::new(deployment.geometric_mean, client.clone()),
            GeometricMeanSolver::new(deployment.geometric_mean, client),
        )
    }

    fn get_strategy_address(strategy_contract: &Self::StrategyContract) -> eAddress {
        strategy_contract.address()
    }

    async fn get_init_data(
        base_config: &BaseConfig,
        params: &Self::Parameters,
        allocation_data: &Self::AllocationData,
        solver_contract: &Self::SolverContract,
    ) -> Result<Bytes> {
        let geometric_mean_params = geometric_mean_solver::GeometricMeanParams {
            w_x: params.target_weight_x,
            w_y: params.target_weight_y,
            swap_fee: base_config.swap_fee,
            controller: eAddress::zero(),
        };

        debug!(
            "Encoding g3m init data: amount: {:?}, price: {:?}, g3m params: {:?}",
            allocation_data.amount_x, allocation_data.price, geometric_mean_params
        );
        let init_bytes = solver_contract
            .get_initial_pool_data(
                allocation_data.amount_x,
                allocation_data.price,
                geometric_mean_params,
            )
            .call()
            .await?;
        debug!("Encoded g3m init data: {:?}", init_bytes);
        Ok(init_bytes)
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
}
