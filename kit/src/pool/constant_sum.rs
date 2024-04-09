use arbiter_engine::machine::{Configuration, Processing};
use bindings::{constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver};

use super::*;

pub struct ConstantSumPool<S: State> {
    pub data: S::Data,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstantSumPoolConfig {
    pub price: eU256,
    pub swap_fee: eU256,
}

#[derive(Clone, Debug)]
pub struct ConstantSumPoolDeployed {
    pub strategy_contract: ConstantSum<ArbiterMiddleware>,
    pub solver_contract: ConstantSumSolver<ArbiterMiddleware>,
    pub parameters: ConstantSumPoolConfig,
}

impl PoolConfigurer for ConstantSumPoolConfig {
    type Parameters = ConstantSumPoolConfig;
}

impl PoolType for ConstantSumPool<Processing<ConstantSumPoolDeployed>> {
    type Parameters = ConstantSumPoolConfig;
    type StrategyContract = ConstantSum<ArbiterMiddleware>;
    type SolverContract = ConstantSumSolver<ArbiterMiddleware>;
    type AllocationData = (AllocateOrDeallocate, eU256, eU256);

    async fn swap_data(
        &self,
        pool_id: eU256,
        input_token: InputToken,
        amount_in: eU256,
    ) -> Result<Bytes> {
        // let (valid, _, data) = match input_token {
        //     InputToken::TokenX => {
        //         self.solver_contract
        //             .simulate_swap(pool_id, true, amount_in)
        //             .call()
        //             .await?
        //     }
        //     InputToken::TokenY => {
        //         self.solver_contract
        //             .simulate_swap(pool_id, false, amount_in)
        //             .call()
        //             .await?
        //     }
        // };
        // if valid {
        //     Ok(data)
        // } else {
        //     anyhow::bail!("swap was invalid!")
        // }
        todo!()
    }

    async fn update_data(&self, parameters: Self::Parameters) -> Result<Bytes> {
        // let price_update_data = self
        //     .solver_contract
        //     .prepare_price_update(parameters.price)
        //     .call()
        //     .await?;
        // Ok(price_update_data)
        todo!()
    }

    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        // let (amount_x, amount_y, allocate) = match allocation_data.0 {
        //     AllocateOrDeallocate::Allocate => (allocation_data.1, allocation_data.2, true),
        //     AllocateOrDeallocate::Deallocate => (allocation_data.1, 0.into(), false),
        // };
        // let (valid, data) = self
        //     .solver_contract
        //     .simulate_allocate_or_deallocate(pool_id, allocate, amount_x, amount_y)
        //     .call()
        //     .await?;
        // if valid {
        //     Ok(data)
        // } else {
        //     anyhow::bail!("allocation was invalid!")
        // }
        todo!()
    }
}
