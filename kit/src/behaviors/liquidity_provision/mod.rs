use std::fmt::Debug;
use std::marker::PhantomData;

use ethers::types::I256;
use serde::de::DeserializeOwned;

use super::*;

pub mod initial;

#[derive(Debug, Deserialize, Serialize)]
pub struct LiquidityProvision<A, E>
where
    A: AllocationType<E>,
    E: Debug + Send + Sync,
{
    pub allocation_type: A,
    // #[serde(skip)]
    // pub pool: Option<Pool<P>>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    phantom: PhantomData<E>,
}

pub trait AllocationType<E>: Send + Sync + Debug + Serialize + 'static {
    fn change_allocation_amount(self, event: E) -> Option<Allocation>;
}

pub struct Allocation {
    pub amount_x: Option<I256>,
    pub amount_y: Option<I256>,
    pub price: Option<eU256>,
}

#[async_trait::async_trait]
impl<A, E> Behavior<E> for LiquidityProvision<A, E>
where
    A: AllocationType<E> + DeserializeOwned,
    E: Debug + Send + Sync + 'static,
{
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<E>>> {
        self.client = Some(client);
        self.messager = Some(messager);
        Ok(None)
    }
}
