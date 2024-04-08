use std::marker::PhantomData;
use arbiter_engine::machine::ControlFlow;

use self::pool::PoolType;

use super::*;

trait AllocationType<E> {
    fn change_allocation_amount(&self, event: E) -> Option<eU256>;
}

// not sure what to specify for E right now but this is the general idea so far
struct ChangeAllocation<P: PoolType, A: AllocationType<E>> {
    pool: P,
    client: Arc<ArbiterMiddleware>,
    allocation_data: P::AllocationData,
    allocation_type: A,
    phantom: PhantomData<E>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitialAllocation<P: PoolType> {
    pub initial_x: eU256,
    pub initial_price: eU256,
    pub initial_parameters: P::AllocationData,
    pub tokens: (ArbiterToken<ArbiterMiddleware>, ArbiterToken<ArbiterMiddleware>),


}
#[allow(unused_variables)]
#[async_trait::async_trait]
impl<P: PoolType> Behavior<()> for InitialAllocation<P> {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        Ok(None)
    }
}
