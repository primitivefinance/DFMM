use bindings::{log_normal::LogNormal, log_normal_solver::LogNormalSolver};

use super::*;
pub struct LogNormalPool {
    pub strategy_contract: LogNormal<ArbiterMiddleware>,
    pub solver_contract: LogNormalSolver<ArbiterMiddleware>,
    pub parameters: LogNormalParameters,
}

pub struct LogNormalParameters {
    pub price: eU256,
    pub swap_fee: eU256,
}

pub struct LogNormalAllocationData {
    pub allocate: bool,
    pub amount: eU256,
}

impl PoolType for LogNormalPool {
    type Parameters = LogNormalParameters;
    type StrategyContract = LogNormal<ArbiterMiddleware>;
    type SolverContract = LogNormalSolver<ArbiterMiddleware>;
    type AllocationData = LogNormalAllocationData;

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

    async fn update_data(&self, new_data: Self::Parameters) -> Result<Bytes> {
        todo!()
    }

    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_date: Self::AllocationData,
    ) -> Result<Bytes> {
        todo!()
    }
}
