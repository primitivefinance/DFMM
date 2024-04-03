use bindings::{n_token_geometric_mean::NTokenGeometricMean, n_token_geometric_mean_solver::NTokenGeometricMeanSolver};
use ethers::types::Address;

use super::*;

pub struct GeometricMeanPool {
    pub strategy_contract: NTokenGeometricMean<ArbiterMiddleware>,
    pub solver_contract: NTokenGeometricMeanSolver<ArbiterMiddleware>,
    pub parameters: NTokenGeometricMeanParameters,
}

pub struct NTokenGeometricMeanParameters {
    pub swap_fee: eU256,
    pub update_parameters: UpdateParameters,
}

pub enum UpdateParameters {
    SwapFee(eU256),
    TargetWeights(Vec<eU256>, eU256),
    TargetController(Address),
}

pub enum GeometricMeanAllocationData {
    AllocateGivenT(eU256, eU256),
    DeallocateGivenT(eU256, eU256),
}

impl PoolType for GeometricMeanPool {
    type UpdateParameters = NTokenGeometricMeanParameters;
    type StrategyContract = NTokenGeometricMean<ArbiterMiddleware>;
    type SolverContract = NTokenGeometricMeanSolver<ArbiterMiddleware>;
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
            UpdateParameters::TargetWeights(weights, target_timestamp) => {
                self.solver_contract
                    .prepare_weights_update(weights, target_timestamp)
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
            GeometricMeanAllocationData::AllocateGivenT(index, amount_t) => {
                self.solver_contract
                    .get_allocation_deltas_given_delta_t(pool_id, index, amount_t)
                    .call()
                    .await?
            }
            GeometricMeanAllocationData::DeallocateGivenT(index, amount_t) => {
                self.solver_contract
                    .get_deallocation_deltas_given_delta_t(pool_id, index, amount_t)
                    .call()
                    .await?
            }
        };
        let mut deltas = Vec::new();
        for i in data.0.iter() {
            deltas.push(ethers::abi::Token::Uint(*i));
        }
        todo!("return the encoded deltas")
    }
}
