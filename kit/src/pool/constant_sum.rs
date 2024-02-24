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
    type Parameters = ConstantSumParameters;
    type StrategyContract = ConstantSum<ArbiterMiddleware>;
    type SolverContract = ConstantSumSolver<ArbiterMiddleware>;

    async fn swap_data(&self, pool_id: eU256, swap_x_in: bool, amount_in: eU256) -> Result<Bytes> {
        let (valid, _, data) = self
            .solver_contract
            .simulate_swap(pool_id, swap_x_in, amount_in)
            .call()
            .await?;
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
        is_allocate: bool,
        amount_x: eU256,
        amount_y: eU256,
    ) -> Result<Bytes> {
        let (valid, data) = self
            .solver_contract
            .simulate_allocate_or_deallocate(pool_id, is_allocate, amount_x, amount_y)
            .call()
            .await?;
        if valid {
            Ok(data)
        } else {
            anyhow::bail!("allocation was invalid!")
        }
    }
}
