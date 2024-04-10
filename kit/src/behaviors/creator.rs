
use super::*;
use arbiter_engine::machine::{State, Configuration, Processing, Behavior, Processor};
use crate::pool::{PoolType, Pool, PoolConfigurer};

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
pub struct PoolConfig<P: PoolConfigurer> {
    pub params: P::PoolParameters,
    pub token_list: Vec<eAddress>,
    pub initial_allocation: Vec<eU256>,
}

pub struct PoolProcessor<P: PoolType> {
    pub pool: Pool<P>
}


#[async_trait::async_trait]
impl<T> Behavior<()> for PoolCreator<Configuration<PoolConfig<T>>>
where
T: PoolConfigurer,
// P: PoolType,
// E: Send + Sync + 'static,
{
    type Processor = ();
    async fn startup(&mut self, client: Arc<ArbiterMiddleware>, messager: Messager) -> Result<Option<(Self::Processor, EventStream<()>)>> {
        todo!()
    }
}

// #[async_trait::async_trait]
// impl<P,E> Processor<E> for PoolCreator<Processing<PoolProcessor<P>>>
// where
// P: PoolType,
// E: Send + Sync + 'static,
// {
//     async fn process(&mut self, event: E) {
//         todo!()
//     }
// }