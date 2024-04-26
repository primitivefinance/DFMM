use std::collections::VecDeque;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update<S: State, P: PoolType> {
    pub token_admin: String,
    pub params: VecDeque<P::Parameters>,
    pub data: S::Data,
}
// TODO: One could add in an `UpdateType` as we have with `Allocate` and `Swap`.
// This `Update` only acts with messages for now.

// TODO: This could be set up in another way if more state dependent data is
// needed! Right now it is just a place holder.
#[derive(Debug, Serialize, Deserialize, State)]
pub struct Config;

#[async_trait::async_trait]
impl<P> Behavior<Message> for Update<Config, P>
where
    P: PoolType + Send + Sync,
{
    type Processor = Update<PoolProcessing<P>, P>;
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
            token_admin: self.token_admin,
            params: self.params,
            data: PoolProcessing {
                messager,
                client,
                pool,
            },
        };
        Ok(processor)
    }
}

#[async_trait::async_trait]
impl<P> Processor<Message> for Update<PoolProcessing<P>, P>
where
    P: PoolType + Send + Sync,
{
    async fn get_stream(&mut self) -> Result<Option<EventStream<Message>>> {
        Ok(Some(self.data.messager.stream()?))
    }
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        match serde_json::from_str(&event.data) {
            Ok(UpdateRequest::ApplyUpdate) => {
                let params = self.params.pop_front().unwrap();
                self.data.pool.update(params.clone()).await?;
                let _ = self
                    .data
                    .messager
                    .send(To::Agent(event.from), MessageTypes::Update::<P>(params))
                    .await?;
                info!("Successfully updated!");
            }
            Err(e) => {
                debug!(
                    "Received message in `Update::process` that agent {:#?} cannot process: {}",
                    self.data.messager.id.clone(),
                    e
                );
            }
        }
        Ok(ControlFlow::Continue)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UpdateRequest {
    ApplyUpdate,
}
