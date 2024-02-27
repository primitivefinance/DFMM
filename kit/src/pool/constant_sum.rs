use bindings::{constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver};

use super::*;

pub struct ConstantSumPool {
    pub strategy_contract: ConstantSum<ArbiterMiddleware>,
    pub solver_contract: ConstantSumSolver<ArbiterMiddleware>,
    pub parameters: ConstantSumParameters,
}

pub struct ConstantSumParameters {
    pub price: eU256,
    pub swap_fee: eU256,
}

impl PoolType for ConstantSumPool {
    type UpdateParameters = ConstantSumParameters;
    type StrategyContract = ConstantSum<ArbiterMiddleware>;
    type SolverContract = ConstantSumSolver<ArbiterMiddleware>;
    type AllocationData = (AllocateOrDeallocate, eU256, eU256);

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

    async fn update_data(&self, parameters: Self::UpdateParameters) -> Result<Bytes> {
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
        let (valid, data) = match allocation_data.0 {
            AllocateOrDeallocate::Allocate => {
                self.solver_contract
                    .simulate_allocate_or_deallocate(
                        pool_id,
                        true,
                        allocation_data.1,
                        allocation_data.2,
                    )
                    .call()
                    .await?
            }
            AllocateOrDeallocate::Deallocate => {
                self.solver_contract
                    .simulate_allocate_or_deallocate(
                        pool_id,
                        false,
                        allocation_data.2,
                        allocation_data.1,
                    )
                    .call()
                    .await?
            }
        };
        if valid {
            Ok(data)
        } else {
            anyhow::bail!("allocation was invalid!")
        }
    }
}
