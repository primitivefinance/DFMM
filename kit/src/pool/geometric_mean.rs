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
    AllocateGivenX(eU256),
    AllocateGivenY(eU256),
    DeallocateGivenX(eU256),
    DeallocateGivenY(eU256),
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
        let (valid, _, _, data) = match input_token {
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
        let (next_x, next_y, next_l) = match allocation_data {
            GeometricMeanAllocationData::AllocateGivenX(amount_x) => {
                self.solver_contract
                    .allocate_given_x(pool_id, amount_x)
                    .call()
                    .await?
            }
            GeometricMeanAllocationData::AllocateGivenY(amount_y) => {
                self.solver_contract
                    .allocate_given_y(pool_id, amount_y)
                    .call()
                    .await?
            }
            GeometricMeanAllocationData::DeallocateGivenX(amount_x) => {
                self.solver_contract
                    .deallocate_given_x(pool_id, amount_x)
                    .call()
                    .await?
            }
            GeometricMeanAllocationData::DeallocateGivenY(amount_y) => {
                self.solver_contract
                    .deallocate_given_y(pool_id, amount_y)
                    .call()
                    .await?
            }
        };
        // Byte hell to convert the eU256's to bytes
        let token_1 = ethers::abi::Token::Uint(next_x);
        let token_2 = ethers::abi::Token::Uint(next_y);
        let token_3 = ethers::abi::Token::Uint(next_l);
        let bytes: Bytes = ethers::abi::encode(&[token_1, token_2, token_3])
            .iter()
            .collect();
        Ok(bytes)
    }
}
