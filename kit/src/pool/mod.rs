use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::machine::State;
use ethers::types::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::bindings::dfmm::DFMM;
use super::*;
use crate::bindings::arbiter_token::ArbiterToken;

pub mod constant_sum;
pub mod geometric_mean;
pub mod log_normal;

pub trait PoolConfigurer:
    std::fmt::Debug + Clone + Serialize + for<'de> Deserialize<'de> + 'static
{
    type Parameters: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;
}

pub trait PoolType {
    type Parameters: PoolConfigurer;
    type StrategyContract: Send + Sync + 'static;
    type SolverContract: Send + Sync + 'static;
    type AllocationData: Send + Sync + 'static;

    #[allow(async_fn_in_trait)]
    async fn swap_data(&self, pool_id: eU256, swap: InputToken, amount_in: eU256) -> Result<Bytes>;
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

pub enum InputToken {
    TokenX,
    TokenY,
}

pub enum AllocateOrDeallocate {
    Allocate,
    Deallocate,
}

#[derive(Clone, Debug)]
pub struct Pool<P: PoolType> {
    pub id: eU256,
    pub dfmm: DFMM<ArbiterMiddleware>,
    pub instance: P::StrategyContract,
    pub tokens: Vec<ArbiterToken<ArbiterMiddleware>>,
}

impl<P: PoolType> Pool<P> {
    pub async fn swap(&self, amount_in: eU256, token_in: InputToken) -> Result<()> {
        // let data = match token_in {
        //     InputToken::TokenX => {
        //         self.instance
        //             .swap_data(self.id, token_in, amount_in)
        //             .await?
        //     }
        //     InputToken::TokenY => {
        //         self.instance
        //             .swap_data(self.id, token_in, amount_in)
        //             .await?
        //     }
        // };
        // self.dfmm.swap(self.id, data).send().await?.await?;
        // Ok(())
        todo!()
    }
}

// async fn swap_data(
//     &self,
//     pool_id: eU256,
//     swap: Self::AllocationData,
//     amount_in: eU256,
// ) -> Result<Bytes>;
