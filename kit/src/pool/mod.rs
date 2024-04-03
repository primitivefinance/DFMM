use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::Bytes;

use self::bindings::dfmm::DFMM;
use super::*;
use crate::bindings::arbiter_token::ArbiterToken;

pub mod constant_sum;
pub mod geometric_mean;
pub mod log_normal;

pub trait PoolType {
    type UpdateParameters;
    type StrategyContract;
    type SolverContract;
    type AllocationData;

    #[allow(async_fn_in_trait)]
    async fn swap_data(&self, pool_id: eU256, swap: InputToken, amount_in: eU256) -> Result<Bytes>;
    /// Change Parameters
    #[allow(async_fn_in_trait)]
    async fn update_data(&self, new_data: Self::UpdateParameters) -> Result<Bytes>;
    /// Change Allocation Date
    #[allow(async_fn_in_trait)]
    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_date: Self::AllocationData,
    ) -> Result<Bytes>;
}

pub enum InputToken {
    TokenX,
    TokenY,
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
    pub async fn swap(&self, amount_in: eU256, token_in: InputToken) -> Result<()> {
        let data = match token_in {
            InputToken::TokenX => {
                self.instance
                    .swap_data(self.id, token_in, amount_in)
                    .await?
            }
            InputToken::TokenY => {
                self.instance
                    .swap_data(self.id, token_in, amount_in)
                    .await?
            }
        };
        self.dfmm
            .swap(self.id, self.dfmm.address(), data)
            .send()
            .await?
            .await?;
        Ok(())
    }
}
