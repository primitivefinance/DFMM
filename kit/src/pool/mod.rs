use std::fmt::Debug;

use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::Bytes;

use self::bindings::dfmm::DFMM;
use super::*;
use crate::bindings::arbiter_token::ArbiterToken;

pub mod constant_sum;
pub mod geometric_mean;
pub mod log_normal;

pub trait PoolType: Debug + Send + Sync {
    type Parameters;
    type StrategyContract;
    type SolverContract;
    type AllocationData;

    #[allow(async_fn_in_trait)]
    async fn swap_data(&self, pool_id: eU256, swap: Token, amount_in: eU256) -> Result<Bytes>;
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

pub enum Token {
    TokenX,
    TokenY,
}

pub enum AllocateOrDeallocate {
    Allocate,
    Deallocate,
}

#[derive(Debug)]
pub struct Pool<P: PoolType> {
    pub id: eU256,
    pub instance: P,
    pub dfmm: DFMM<ArbiterMiddleware>,
    pub token_x: ArbiterToken<ArbiterMiddleware>,
    pub token_y: ArbiterToken<ArbiterMiddleware>,
}

impl<P: PoolType> Pool<P> {
    pub async fn swap(&self, amount_in: eU256, token_in: Token) -> Result<()> {
        let data = match token_in {
            Token::TokenX => {
                self.instance
                    .swap_data(self.id, token_in, amount_in)
                    .await?
            }
            Token::TokenY => {
                self.instance
                    .swap_data(self.id, token_in, amount_in)
                    .await?
            }
        };
        self.dfmm.swap(self.id, data).send().await?.await?;
        Ok(())
    }
}

// async fn swap_data(
//     &self,
//     pool_id: eU256,
//     swap: Self::AllocationData,
//     amount_in: eU256,
// ) -> Result<Bytes>;
