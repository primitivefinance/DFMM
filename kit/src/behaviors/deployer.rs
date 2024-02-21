use arbiter_core::middleware::ArbiterMiddleware;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {}

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        Ok(None)
    }
}
