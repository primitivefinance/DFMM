use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Initialize<P: PoolType> {
    // pub pool: Pool<P>,
    phantom: std::marker::PhantomData<P>,
}

#[async_trait::async_trait]
impl<P: PoolType> Behavior<()> for Initialize<P> {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        Ok(None)
    }
}
