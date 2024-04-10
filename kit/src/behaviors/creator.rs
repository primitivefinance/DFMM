use super::*;
use crate::pool::{Pool, PoolType};
use arbiter_engine::machine::{Behavior, Configuration, Processing, Processor, State};

// Idea: Let's make a behavior that has two states:
// State 1. This is for configuration and it should have everything be `Serialize`/`Deserialize` so that it can be read in from a config.
// State 2. This is the "built" version of the behavior that may now own client, messager, or contracts (etc.) and other things that had to be gotten from running the `startup` method.

// Example:
// Let's make a "pool_creator" type of behavior that will take some configuration for a pool and work to attempt to deploy that pool.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolCreator<S: State> {
    pub data: S::Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolConfig<P: PoolType> {
    pub params: P::PoolParameters,
    pub initial_allocation_data: P::InitializationData,
    pub token_list: Vec<eAddress>,
}

pub struct PoolProcessor<P: PoolType> {
    pub pool: Pool<P>,
}

#[async_trait::async_trait]
impl<P, E> Behavior<E> for PoolCreator<Configuration<PoolConfig<P>>>
where
    P: PoolType + Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    type Processor = PoolCreator<Processing<PoolProcessor<P>>>;
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<E>)>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl<P, E> Processor<E> for PoolCreator<Processing<PoolProcessor<P>>>
where
    P: PoolType + Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    async fn process(&mut self, _event: E) -> Result<ControlFlow> {
        Ok(ControlFlow::Halt)
    }
}
