use std::collections::VecDeque;

use bindings::idfmm::IDFMM;
use tracing::warn;

use super::*;
use crate::bindings::erc20::ERC20;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update<S: State> {
    pub token_admin: String,
    pub data: S::Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config<P: PoolType> {
    pub base_config: BaseConfig,
    pub allocation_data: P::AllocationData,
    pub token_list: Vec<String>,
    pub params: VecDeque<P::Parameters>,
}

#[derive(Debug, Clone)]
pub struct Processing<P: PoolType> {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
    pub pool_params: VecDeque<P::Parameters>,
}

impl<P: PoolType> State for Config<P> {
    type Data = Self;
}

impl<P> State for Processing<P>
where
    P: PoolType,
{
    type Data = Self;
}

#[derive(Debug)]
struct Todo<P: PoolType> {
    deployment_data: Option<DeploymentData>,
    pool_creation: Option<(
        eU256,         // Pool ID
        Vec<eAddress>, // Token List
        eAddress,      // Liquidity Token
        <P as PoolType>::Parameters,
        <P as PoolType>::AllocationData,
    )>,
}

#[async_trait::async_trait]
impl<P> Behavior<Message> for Update<Config<P>>
where
    P: PoolType + Send + Sync + 'static,
    P::Parameters: Send + Sync + 'static,
    P::StrategyContract: Send + Sync + 'static,
    P::SolverContract: Send + Sync + 'static,
{
    type Processor = Update<Processing<P>>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        // Make a "TODO" list.
        // This is the data I need to recieve to do my job
        let mut todo: Todo<P> = Todo {
            deployment_data: None,
            pool_creation: None,
        };

        // Loop through the messager until we check off the boxes for this TODO list.
        debug!("Updater is looping through their TODO list.");
        loop {
            if let Ok(msg) = messager.get_next::<MessageTypes<P>>().await {
                match msg.data {
                    MessageTypes::Deploy(deploy_data) => {
                        debug!("Updater: Got deployment data: {:?}", deploy_data);
                        todo.deployment_data = Some(deploy_data);
                        if todo.pool_creation.is_some() {
                            debug!("Updater: Got all the data.\n{:#?}", todo);
                            break;
                        }
                    }
                    MessageTypes::Create(pool_creation) => {
                        debug!("Updater: Got pool creation data: {:?}", pool_creation);
                        todo.pool_creation = Some(pool_creation);
                        if todo.deployment_data.is_some() {
                            debug!("Updater: Got all the data.\n{:#?}", todo);
                            break;
                        }
                    }
                    _ => continue,
                }
            } else {
                debug!("Updater got some other message variant it could ignore.");
                continue;
            }
        }
        debug!("Updater has checked off their TODO list.");

        let (strategy_contract, solver_contract) =
            P::get_contracts(todo.deployment_data.as_ref().unwrap(), client.clone());
        let dfmm = DFMM::new(todo.deployment_data.unwrap().dfmm, client.clone());
        debug!("Got DFMM and the strategy contracts.");

        let pool = Pool::<P> {
            id: todo.pool_creation.clone().unwrap().0,
            dfmm,
            instance: P::create_instance(
                strategy_contract,
                solver_contract,
                todo.pool_creation.clone().unwrap().3.clone(),
            ),
            tokens: todo
                .pool_creation
                .clone()
                .unwrap()
                .1
                .into_iter()
                .map(|t| ArbiterToken::new(t, client.clone()))
                .collect(),
            liquidity_token: ERC20::new(todo.pool_creation.as_ref().unwrap().2, client.clone()),
        };

        debug!("Updater has built the pool.");

        let process = Self::Processor {
            token_admin: self.token_admin.clone(),
            data: Processing {
                messager: messager.clone(),
                client,
                pool,
                pool_params: self.data.params.clone(),
            },
        };
        let stream = messager.stream()?;
        Ok(Some((process, stream)))
    }
}

#[async_trait::async_trait]
impl<P> Processor<Message> for Update<Processing<P>>
where
    P: PoolType + Send + Sync,
{
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        warn!("Process: Got event: {:?}", event);
        let msg: UpdatoorQuerry = serde_json::from_str(&event.data).unwrap_or(UpdatoorQuerry::NoOp);

        warn!("Process: deserialized update querry: {:?}", msg);

        match msg {
            UpdatoorQuerry::UpdateMeDaddy => {
                let params = self.data.pool_params.pop_front().unwrap();
                self.data.pool.update(params.clone()).await?;
                let _ = self
                    .data
                    .messager
                    .send(To::Agent(event.from), MessageTypes::Update::<P>(params))
                    .await?;
                info!("Successfully updated!");
            }

            UpdatoorQuerry::NoOp => {
                debug!("NoOp");
            }
        }

        Ok(ControlFlow::Continue)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UpdatoorQuerry {
    NoOp,
    UpdateMeDaddy,
}
