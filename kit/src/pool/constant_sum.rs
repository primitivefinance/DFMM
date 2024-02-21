use bindings::{constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver};
use ethers::abi::Tokenize;

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
        let data = self
            .solver_contract
            .simulate_swap(pool_id, swap_x_in, amount_in)
            .call()
            .await?;
        Ok(Bytes::from(ethers::abi::encode(&data.into_tokens())))
    }
}
