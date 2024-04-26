use super::*;

pub trait AllocateType<E>
where
    E: Send + 'static,
{
    // TODO: This should probably be how we do it, but this generic `P` gets
    // annoying fn change_allocation_amount(&mut self, event: E) ->
    // Option<P::AllocationData>;
    fn change_allocation_amount(&mut self, event: E) -> Option<Vec<eI256>>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Allocate<A, E, S>
where
    A: AllocateType<E>,
    E: Send + 'static,
    S: State,
{
    pub data: S::Data,
    pub allocate_type: A,
    _phantom_a: PhantomData<A>,
    _phantom_e: PhantomData<E>,
}

#[derive(Debug, Serialize, Deserialize, State)]
pub struct Config<P: PoolType> {
    pub allocation_data: P::AllocationData,
}

#[derive(State)]
pub struct Processing<P, E>
where
    P: PoolType,
    E: Send + 'static,
{
    pub pool: Pool<P>,
    pub client: Arc<ArbiterMiddleware>,
    pub messager: Messager,
    _phantom: PhantomData<E>,
}

#[allow(unused_variables)]
#[async_trait::async_trait]
impl<A, P, E> Behavior<E> for Allocate<A, E, Config<P>>
where
    A: AllocateType<E> + Send,
    P: PoolType + Send,
    E: Send + 'static,
{
    type Processor = Allocate<A, E, Processing<P, E>>;
    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Self::Processor> {
        todo!();
    }
}

#[async_trait::async_trait]
impl<A, P, E> Processor<E> for Allocate<A, E, Processing<P, E>>
where
    A: AllocateType<E> + Send,
    P: PoolType + Send,
    E: Send + 'static,
{
    async fn get_stream(&mut self) -> Result<Option<EventStream<E>>> {
        todo!("We have not implemented the 'get_stream' method yet for the 'Allocate' behavior.");
    }
    async fn process(&mut self, _event: E) -> Result<ControlFlow> {
        Ok(ControlFlow::Halt)
    }
}
