use std::marker::PhantomData;

use arbiter_core::events::stream_event;
use arbiter_engine::machine::{Configuration, ControlFlow, Processing, Processor, State};

use self::{
    pool::PoolType,
    token_admin::{MintRequest, TokenAdminQuery},
};
use super::*;

// Notes:
// * The point of this function is to have the event piped to it from the
//   behavior/processor and this just dictates how, based on some event, we want
//   to change the allocation we have (or will have) in a pool.
trait AllocationType<P, E>
where
    P: PoolType,
{
    fn change_allocation_amount(&self, event: E) -> Option<P::AllocationData>;
}

// Notes:
// * The idea here is that the `ChangeAllocation` is generic over everything it
//   needs to be generic over as far as how we'd want to interact with pools
//   (i.e., `PoolType`).
// * It is also generic over the `AllocationType` so that we can implement the
//   `Behavior`/`Processor` on this `ChangeAllocation` struct and have
//   boilerplate here for how ANY `AllocationType` interacts with ANY
//   `PoolType`.
// * In effect, a user ONLY has to implement the `AllocationType` trait
//   themselves and decide how they want to do the allocations -- E.g.,
//   `InitialAllocation` just chooses to allocate an amount to create a new
//   pool. -- E.g., `RandomAllocation` would take in an event `E` like
//   `BlockUpdate`, and based on this, randomly decides to allocate a random
//   amount of liquidity.
// How this works (Type state pattern):
// By letting this be generic over `S: State`, we can have two different states
// that this struct exists in which helps us be able to create a version of this
// struct that can be read in from a config.toml (e.g., `S == Configuration`)
// while having a version of the struct that can do processing (e.g., `S ==
// Processing`). Having things this way makes it possible to have a
// parameterized `ChangeAllocation` that can then set itself up in the
// `Behavior::startup()` method so, for example, we don't have to have
// `Option<Arc<ArbiterMiddleware>>` or other fields that CANNOT BE
// DESERIALIZABLE!!! That's the trick.
// Example:
// I can read in from `config.toml` into `ChangeAllocation<A, P, E,
// Configuration>` since this should be Deserializable.
//
// Some more notes:
#[derive(Debug, Serialize, Deserialize)]
struct ChangeAllocation<A, P, E, S>
where
    A: AllocationType<P, E>,
    P: PoolType,
    S: State,
{
    // APES LOL
    _phantom_a: PhantomData<A>,
    _phantom_p: PhantomData<P>,
    _phantom_e: PhantomData<E>,
    pub data: S::Data,
}

// This `ChangeAllocationStructData` will be the `D` in `S == Processing<D>`
pub struct ChangeAllocationStructData<A, P, E>
where
    A: AllocationType<P, E>,
    P: PoolType,
{
    pub client: Arc<ArbiterMiddleware>,
    pub pool: P,
    pub allocation_data: P::AllocationData,
    pub allocation_type: A,
    _phantom: PhantomData<E>,
}

impl<A, P, E, S> AllocationType<P, E> for ChangeAllocation<A, P, E, S>
where
    A: AllocationType<P, E>,
    P: PoolType,
    S: State,
{
    fn change_allocation_amount(&self, event: E) -> Option<P::AllocationData> {
        None
    }
}

#[derive(Debug)]
pub struct InitialAllocation<P: PoolType> {
    /// The initial amount of token X.
    pub initial_x: eU256,
    /// The initial Price
    pub initial_price: eU256,
    /// Initial Parameters
    pub initial_parameters: P::AllocationData,
    /// The tokens to allocate
    pub tokens: (
        ArbiterToken<ArbiterMiddleware>,
        ArbiterToken<ArbiterMiddleware>,
    ),
    /// The tokens to request.
    pub token_data: TokenData,
    /// The agent ID to request tokens to.
    pub request_to: String,
    /// Client to have an address to receive token mint to and check balance
    // #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
    /// The messaging layer for the token requester.
    // #[serde(skip)]
    pub messager: Option<Messager>,
}

#[allow(unused_variables)]
#[async_trait::async_trait]
impl<A, P, E> Behavior<E> for ChangeAllocation<A, P, E, Configuration>
where
    A: AllocationType<P, E> + std::fmt::Debug + Send + Sync + 'static,
    P: PoolType + std::fmt::Debug + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    type Processor = ChangeAllocation<A, P, E, Processing<ChangeAllocationStructData<A, P, E>>>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<E>)>> {
        todo!();
    }
}

#[async_trait::async_trait]
impl<A, P, E> Processor<E>
    for ChangeAllocation<A, P, E, Processing<ChangeAllocationStructData<A, P, E>>>
where
    A: AllocationType<P, E>,
    P: PoolType,
    E: Send + 'static,
{
    async fn process(&mut self, event: E) -> Result<ControlFlow> {
        Ok(ControlFlow::Halt)
    }
}

// #[allow(unused_variables)]
// #[async_trait::async_trait]
// impl<P: PoolType, A: AllocationType<E>, E> Behavior<()> for
// ChangeAllocation<P, A, E> {     type Processor = ();
//     async fn startup(
//         &mut self,
//         client: Arc<ArbiterMiddleware>,
//         messager: Messager,
//     ) -> Result<Option<EventStream<()>>> {
//         todo!();
//         // PSEUDOCODE FOR HOW WE CAN WORK WITH THIS
//         // // get the `event`
//         // let event = ();
//         // let amount = self.change_allocation_amount(event);

//         // let allocation_data = self.pool.change_allocation_data(pool_id,
// amount);         // self.pool.dfmm.update_allocation(allocation_data);

//         // messager
//         //     .send(
//         //         To::Agent(self.request_to.clone()),
//         //         &TokenAdminQuery::AddressOf(self.token_data.name.clone()),
//         //     )
//         //     .await?;
//         // let message = messager.get_next().await.unwrap();
//         // let token_address =
// serde_json::from_str::<eAddress>(&message.data).unwrap();         // let
// token = ArbiterToken::new(token_address, client.clone());         //
// self.token_data.address = Some(token_address);

//         // let mint_data = TokenAdminQuery::MintRequest(MintRequest {
//         //     token: self.token_data.name.clone(),
//         //     mint_to: client.address(),
//         //     mint_amount: 1,
//         // });
//         // messager
//         //     .send(To::Agent(self.request_to.clone()), mint_data)
//         //     .await?;

//         // self.messager = Some(messager.clone());
//         // self.client = Some(client.clone());
//         // let transfer_stream = stream_event(token.transfer_filter());
//         // Ok(Some(transfer_stream))
//     }
// }
