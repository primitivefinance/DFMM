// Notes:
// Idea is that we want to be able to configure behaviors that depend on the `PoolType` generic from the config.toml.
// What this means is that `PoolType` itself has to be `Deserialize`able and this is kinda tough to work with.
// ---->>> The reason why is because we can't `Deserialize` contract objects themselves because that's just not possible.

use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::bindings::dfmm::DFMM;
use super::*;
use crate::bindings::arbiter_token::ArbiterToken;

pub mod constant_sum;
// pub mod geometric_mean;
// pub mod log_normal;
// pub mod n_token_geometric_mean;

// Notes:
// `InitData` is something that all pools need  in order to be created. This consists of:
// 1. The parameters of the pool which, for example, are like the `mean` and `width` of the `LogNormal` pool. (Strategy specific since other pools might have different params like `ConstantSum` has `price`)
// 2. Initial allocation data, which consists of, for example, a `price` and an amount of `token_x` for the `LogNormal` pool. (Strategy specific since other pools like `ConstantSum` may not have the same needs)
// 3. Base configuration which ALL pools share as part of their parameterization which is the `swap_fee`, `controller` and the `controller_fee`. Every type of strategy needs these.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitData<P: PoolType> {
    pub params: P::PoolParameters,
    pub initial_allocation_data: P::InitialAllocationData,
    pub base_config: BaseParameters,
}

// Notes:
// These are the things that all strategies need to have to be initialized (and potentially updated).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseParameters {
    pub swap_fee: eU256,
    pub controller: eAddress,
    pub controller_fee: eU256,
}

// TODO: I don't think we need this anymore!
// Notes:
// This trait provides the interface for people to construct pools from a `Configuration` state since all of this should be `Serialize` and `Deserialize`.
// This stuff ultimately will be what's used to deploy a `Pool<P: PoolType>` which will hold onto actual instances of contracts (whereas this just holds config data).
pub trait PoolConfigurer: Clone + std::fmt::Debug + 'static {
    type PoolParameters: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;
    type InitialAllocationData: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;
}

// Notes:
// Everything from the above now gets collapsed (or inherited) as the associated `Parameters` type of the `PoolType`. E
// All the other types will be specific to each pool/strategy type since those will be specific contracts
pub trait PoolType: Sized + Clone + std::fmt::Debug + 'static {
    // type Parameters: PoolConfigurer;
    type PoolParameters: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;
    type InitialAllocationData: Clone
        + std::fmt::Debug
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static;
    type StrategyContract;
    type SolverContract;
    type AllocationData: Send + Sync + 'static;

    #[allow(async_fn_in_trait)]
    async fn create_pool(
        &self,
        dfmm: DFMM<ArbiterMiddleware>,
        init_data: InitData<Self>,
    ) -> Result<Pool<Self>> {
        todo!()
        // TODO: There is a blanket implementation that we can do here.
        // we might be able to use the solver to encode this.
        //  THIS IS FROM THE BINDINGS FOR THE DFMM CONTRACT, WE NEED THE INPUT TO `dfmm.init(_)` to be of this type.
        // pub struct InitParams {
        //     pub name: ::std::string::String,
        //     pub symbol: ::std::string::String,
        //     pub strategy: ::ethers::core::types::Address,
        //     pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        //     pub data: ::ethers::core::types::Bytes,
        //     pub fee_collector: ::ethers::core::types::Address,
        //     pub controller_fee: ::ethers::core::types::U256,
        // }
    }

    #[allow(async_fn_in_trait)]
    async fn swap_data(&self, pool_id: eU256, swap: InputToken, amount_in: eU256) -> Result<Bytes>;
    /// Change Parameters
    #[allow(async_fn_in_trait)]
    async fn update_data(&self, new_data: Self::PoolParameters) -> Result<Bytes>;
    /// Change Allocation Date
    #[allow(async_fn_in_trait)]
    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes>;
}

pub enum UpdateParameters<P: PoolType> {
    PoolParameters(P::PoolParameters),
    Controller(eAddress),
    Fee(eU256),
}

// Notes:
// This is used in the `swap_data` function of the poolType trait to determine which token to swap in.
pub enum InputToken {
    TokenX,
    TokenY,
}

// Notes:
// This is used in the `change_allocation_data` function of the Pool to determine whether to allocate or deallocate.
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
    pub async fn update(&self, new_data: P::PoolParameters) -> Result<()> {
        let data = self.instance.update_data(new_data).await?;
        self.dfmm.update(self.id, data).send().await?.await?;
        Ok(())
    }
}
