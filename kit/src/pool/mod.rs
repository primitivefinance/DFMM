use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::Bytes;

use self::bindings::dfmm::DFMM;
use super::*;
use crate::bindings::arbiter_token::ArbiterToken;

pub mod constant_sum;
pub mod geometric_mean;
pub mod log_normal;

pub trait PoolType {
    type Parameters;
    type StrategyContract;
    type SolverContract;
    type AllocationData;

    #[allow(async_fn_in_trait)]
    async fn swap_data(&self, pool_id: eU256, swap_x_in: bool, amount_in: eU256) -> Result<Bytes>;
    /// Change Parameters
    #[allow(async_fn_in_trait)]
    async fn update_data(&self, new_data: Self::Parameters) -> Result<Bytes>;
    /// Change Allocation Date
    #[allow(async_fn_in_trait)]
    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_date: Self::AllocationData,
    ) -> Result<Bytes>;
}

pub enum AllocateOrDeallocate {
    Allocate,
    Deallocate,
}

pub struct Pool<P: PoolType> {
    pub id: eU256,
    pub dfmm: DFMM<ArbiterMiddleware>,
    pub instance: P,
    pub token_x: ArbiterToken<ArbiterMiddleware>,
    pub token_y: ArbiterToken<ArbiterMiddleware>,
}

impl<P: PoolType> Pool<P> {
    pub async fn swap(
        &self,
        amount_in: eU256,
        token_in: &ArbiterToken<ArbiterMiddleware>,
    ) -> Result<()> {
        let swap_x_in = token_in.address() == self.token_x.address();
        let data = self
            .instance
            .swap_data(self.id, swap_x_in, amount_in)
            .await?;
        self.dfmm.swap(self.id, data).send().await?.await?;
        Ok(())
    }
}
