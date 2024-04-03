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

pub enum ConstantSumAllocationData {
    AmountX(eU256),
    AmountY(eU256),
}
impl PoolType for ConstantSumPool {
    type Parameters = ConstantSumParameters;
    type StrategyContract = ConstantSum<ArbiterMiddleware>;
    type SolverContract = ConstantSumSolver<ArbiterMiddleware>;
    type AllocationData = ConstantSumAllocationData;

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
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        match allocation_data {
            ConstantSumAllocationData::AmountX(amount_x) => {
                let data = self
                    .solver_contract
                    .prepare_allocation_delta_given_delta_x(pool_id, amount_x)
                    .call()
                    .await?;
                Ok(data)
            }
            ConstantSumAllocationData::AmountY(amount_y) => {
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
