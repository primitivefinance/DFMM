use bindings::{geometric_mean::GeometricMean, geometric_mean_solver::GeometricMeanSolver};

use super::*;

pub struct GeometricMeanPool {
    pub strategy_contract: GeometricMean<ArbiterMiddleware>,
    pub solver_contract: GeometricMeanSolver<ArbiterMiddleware>,
    pub parameters: GeometricMeanParameters,
}

pub struct GeometricMeanParameters {
    pub price: eU256,
    pub swap_fee: eU256,
}

impl PoolType for GeometricMeanPool {
    type Parameters = GeometricMeanParameters;
    type StrategyContract = GeometricMean<ArbiterMiddleware>;
    type SolverContract = GeometricMeanSolver<ArbiterMiddleware>;
    type AllocationData = (AllocateOrDeallocate, eU256, eU256);

    async fn swap_data(&self, pool_id: eU256, swap: InputToken, amount_in: eU256) -> Result<Bytes> {
        todo!()
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
