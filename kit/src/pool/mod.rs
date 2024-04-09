use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::Bytes;

use self::bindings::dfmm::DFMM;
use super::*;
use crate::bindings::arbiter_token::ArbiterToken;

pub mod constant_sum;
pub mod geometric_mean;
pub mod log_normal;
pub mod n_token_geometric_mean;

pub trait PoolType {
    type UpdateParameters;
    type StrategyContract;
    type SolverContract;
    type AllocationData: Send + Sync + 'static;

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
        allocation_data: Self::AllocationData,
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
    /// Performs a swap on the pool.
    ///
    /// # Arguments
    ///
    /// * `amount_in` - The amount of tokens to swap in.
    /// * `token_in` - The type of token to swap in (either TokenX or TokenY).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the swap is successful, otherwise returns an error.
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

    /// Performs an allocation or deallocation on the pool.
    ///
    /// # Arguments
    ///
    /// * `action` - The type of action to perform (either Allocate or
    ///   Deallocate).
    /// * `allocation_data` - The allocation data to use for the action.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the allocation or deallocation is successful,
    /// otherwise returns an error.
    pub async fn allocate_or_deallocate(
        &self,
        action: AllocateOrDeallocate,
        allocation_data: P::AllocationData,
    ) -> Result<()> {
        let data = self
            .instance
            .change_allocation_data(self.id, allocation_data)
            .await?;
        match action {
            AllocateOrDeallocate::Allocate => {
                self.dfmm.allocate(self.id, data).send().await?.await?
            }
            AllocateOrDeallocate::Deallocate => {
                self.dfmm.deallocate(self.id, data).send().await?.await?
            }
        };
        Ok(())
    }

    /// Updates the pool with new data.
    ///
    /// # Arguments
    ///
    /// * `new_data` - The new data to update the pool with.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update is successful, otherwise returns an
    /// error.
    pub async fn update(&self, new_data: P::UpdateParameters) -> Result<()> {
        let data = self.instance.update_data(new_data).await?;
        self.dfmm.update(self.id, data).send().await?.await?;
        Ok(())
    }
}
