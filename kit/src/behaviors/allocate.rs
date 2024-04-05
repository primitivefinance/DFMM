use arbiter_engine::machine::ControlFlow;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Allocate {}

#[async_trait::async_trait]
impl Behavior<()> for Allocate {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        Ok(None)
    }
}
