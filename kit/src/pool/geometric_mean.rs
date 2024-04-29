use bindings::{geometric_mean::GeometricMean, geometric_mean_solver::GeometricMeanSolver};
use ethers::types::Address;

use super::*;

pub struct GeometricMeanPool {
    pub strategy_contract: GeometricMean<ArbiterMiddleware>,
    pub solver_contract: GeometricMeanSolver<ArbiterMiddleware>,
    pub parameters: GeometricMeanParameters,
}

pub struct GeometricMeanParameters {
    pub swap_fee: eU256,
    pub update_parameters: UpdateParameters,
}

pub enum UpdateParameters {
    SwapFee(eU256),
    TargetTimestamp(eU256),
    TargetWeightX(eU256),
    TargetController(Address),
}

pub enum GeometricMeanAllocationData {
    GivenX(eU256),
    GivenY(eU256),
}

impl PoolType for GeometricMeanPool {
    type UpdateParameters = GeometricMeanParameters;
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

    async fn update_data(&self, new_data: Self::UpdateParameters) -> Result<Bytes> {
        let data = match new_data.update_parameters {
            UpdateParameters::SwapFee(fee) => {
                self.solver_contract.prepare_fee_update(fee).call().await?
            }
            UpdateParameters::TargetTimestamp(timestamp) => {
                self.solver_contract
                    .prepare_weight_x_update(timestamp, timestamp)
                    .call()
                    .await?
            }
            UpdateParameters::TargetWeightX(weight_x) => {
                self.solver_contract
                    .prepare_weight_x_update(weight_x, weight_x)
                    .call()
                    .await?
            }
            UpdateParameters::TargetController(controller) => {
                self.solver_contract
                    .prepare_controller_update(controller)
                    .call()
                    .await?
            }
        };
        Ok(data)
    }

    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        let data = match allocation_data {
            GeometricMeanAllocationData::GivenX(amount_x) => {
                self.solver_contract
                    .prepare_allocation_deltas_given_delta_x(pool_id, amount_x)
                    .call()
                    .await?
            }
            GeometricMeanAllocationData::GivenY(amount_y) => {
                self.solver_contract
                    .prepare_allocation_deltas_given_delta_y(pool_id, amount_y)
                    .call()
                    .await?
            }
        };
        Ok(data)
    }
}
