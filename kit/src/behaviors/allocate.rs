use super::*;

pub trait AllocateType<E>: Debug + Serialize + Clone
where
    E: Send + 'static,
{
    // TODO: This should probably be how we do it, but this generic `P` gets
    // annoying fn change_allocation_amount(&mut self, event: E) ->
    // Option<P::AllocationData>;
    fn change_allocation_amount(&mut self, event: E) -> Option<Vec<eI256>>;
    fn get_stream(&self) -> Pin<Box<dyn Stream<Item = E> + Send + Sync>>;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Config<P: PoolType> {
    pub allocation_data: P::AllocationData,
}

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

impl<P: PoolType> State for Config<P> {
    type Data = Self;
}

impl<P, E> State for Processing<P, E>
where
    P: PoolType,
    E: Send + 'static,
{
    type Data = Self;
}

#[allow(unused_variables)]
#[async_trait::async_trait]
impl<A, P, E> Behavior<E> for Allocate<A, E, Config<P>>
where
    A: AllocateType<E> + Debug + Send + Sync + 'static + for<'a> Deserialize<'a>,
    P: PoolType + Debug + Send + Sync + 'static,
    E: Debug + Send + Sync + 'static,
{
    type Processor = Allocate<A, E, Processing<P, E>>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<E>)>> {
        todo!();
    }
}

#[async_trait::async_trait]
impl<A, P, E> Processor<E> for Allocate<A, E, Processing<P, E>>
where
    A: AllocateType<E> + Debug + Send + Sync + 'static,
    P: PoolType + Debug + Send + Sync + 'static,
    E: Debug + Send + Sync + 'static,
{
    async fn process(&mut self, _event: E) -> Result<ControlFlow> {
        Ok(ControlFlow::Halt)
    }
}
