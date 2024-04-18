use super::*;
use crate::{
    behaviors::{creator::PoolCreation, deploy::DeploymentData},
    bindings::erc20::ERC20,
};
use futures_util::StreamExt;

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
    pub params: Vec<P::Parameters>,
}

#[derive(Debug, Clone)]
pub struct Processing<P: PoolType> {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
    pub pool_params: Vec<P::Parameters>,
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

#[async_trait::async_trait]
impl<P> Behavior<Message> for Update<Config<P>>
where
    P: PoolType + Send + Sync + for<'de>Deserialize<'de> + 'static,
    P::Parameters: Send + Sync +'static,
    P::StrategyContract: Send + Sync + 'static,
    P::SolverContract: Send + Sync + 'static,
{
    type Processor = Update<Processing<P>>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        // Configuration from deployed contracts

        debug!("starting the updator");
        let deployment_data = messager.clone().get_next::<DeploymentData>().await?.data;
        debug!("got message {:?}", deployment_data);
        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());
        let mut init_event_stream = stream_event(dfmm.init_filter());

        let init_event = init_event_stream.next().await.unwrap();
        debug!("got init event {:?}", init_event);

        // this might be failing here
        // let msg = messager.get_next::<MessageTypes<P>>().await?.data;

        let pool_creatrion = while let MessageTypes::Create(pool_creation) = messager.get_next::<MessageTypes<P>>().await?.data { 
            // debug!("got message {:?}", msg);
        };

        let instance = P::create_instance(strategy_contract, solver_contract, pool_creation.params);
        let lp_token = ERC20::new(init_event.lp_token, client.clone());
        // Get the intended tokens for the pool and do approvals.
        let mut tokens: Vec<ArbiterToken<ArbiterMiddleware>> = Vec::new();
        for _ in self.data.token_list.drain(..) {
            let token = ArbiterToken::new(
                messager.get_next::<eAddress>().await.unwrap().data,
                client.clone(),
            );
            tokens.push(token);
        }

        let pool = Pool::<P> {
            id: pool_creation.id,
            dfmm,
            instance,
            tokens,
            liquidity_token: lp_token,
        };

        let process = Self::Processor {
            token_admin: self.token_admin.clone(),
            data: Processing {
                messager,
                client,
                pool,
                pool_params: self.data.params.clone(),
            },
        };
        let stream = process.data.messager.clone().stream()?;
        Ok(Some((process, stream)))
    }
}

#[async_trait::async_trait]
impl<P> Processor<Message> for Update<Processing<P>>
where
    P: PoolType + Send + Sync,
{
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let msg: UpdatoorQuerry = serde_json::from_str(&event.data).unwrap_or(UpdatoorQuerry::NoOp);

        match msg {
            UpdatoorQuerry::UpdateMeDaddy => {
                let params = self.data.pool_params.pop().unwrap();
                self.data.pool.update(params.clone()).await?;
                let _ = self
                    .data
                    .messager
                    .send(To::Agent(event.from), params)
                    .await?;
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
