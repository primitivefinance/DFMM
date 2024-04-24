use std::collections::VecDeque;

use tracing::warn;

use super::*;
use crate::bindings::erc20::ERC20;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update<S: State> {
    pub token_admin: String,
    pub data: S::Data,
}

#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Config<P: PoolType> {
    pub base_config: BaseConfig,
    pub allocation_data: P::AllocationData,
    pub token_list: Vec<String>,
    pub params: VecDeque<P::Parameters>,
}

#[derive(Debug, Clone, State)]
pub struct Processing<P: PoolType> {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
    pub pool_params: VecDeque<P::Parameters>,
}

#[async_trait::async_trait]
impl<P> Behavior<Message> for Update<Config<P>>
where
    P: PoolType + Send + Sync,
{
    type Processor = Update<Processing<P>>;
    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Self::Processor> {
        let completed_todo = GetPoolTodo::<P>::complete(&mut messager).await;
        let (deployment_data, pool_creation) = (
            completed_todo.deployment_data.unwrap(),
            completed_todo.pool_creation.unwrap(),
        );

        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());

        let pool = Pool::<P> {
            id: pool_creation.id,
            dfmm,
            instance: P::create_instance(strategy_contract, solver_contract, pool_creation.params),
            tokens: pool_creation
                .tokens
                .into_iter()
                .map(|t| ArbiterToken::new(t, client.clone()))
                .collect(),
            liquidity_token: ERC20::new(pool_creation.liquidity_token, client.clone()),
        };

        let processor = Self::Processor {
            token_admin: self.token_admin.clone(),
            data: Processing {
                messager,
                client,
                pool,
                pool_params: self.data.params.clone(),
            },
        };
        Ok(processor)
    }
}

#[async_trait::async_trait]
impl<P> Processor<Message> for Update<Processing<P>>
where
    P: PoolType + Send + Sync,
{
    async fn get_stream(&mut self) -> Result<Option<EventStream<Message>>> {
        Ok(Some(self.data.messager.stream()?))
    }
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        match serde_json::from_str(&event.data) {
            Ok(UpdateRequest::ApplyUpdate) => {
                let params = self.data.pool_params.pop_front().unwrap();
                self.data.pool.update(params.clone()).await?;
                let _ = self
                    .data
                    .messager
                    .send(To::Agent(event.from), MessageTypes::Update::<P>(params))
                    .await?;
                info!("Successfully updated!");
            }
            Err(e) => {
                warn!("Failed to parse message: {}", e);
            }
        }
        Ok(ControlFlow::Continue)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UpdateRequest {
    ApplyUpdate,
}
