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
        // Configuration from deployed contracts

        debug!("Startup: starting the updator");
        let deployment_data = messager.clone().get_next::<DeploymentData>().await?.data;
        debug!("Startup: got message {:?}", deployment_data);
        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());
        let mut init_event_stream = stream_event(dfmm.init_filter());

        let init_event = init_event_stream.next().await.unwrap();
        debug!("Startup: got init event {:?}", init_event);

        let instance = loop {
            // TODO: This is where we use the weird tuple struct to bypass compile issues
            // with the `MessageTypes<P>` enum. See `behaviors/mod.rs` for that.
            if let MessageTypes::Create((_id, params, _allocation_data)) =
                messager.get_next::<MessageTypes<P>>().await?.data
            {
                break P::create_instance(strategy_contract, solver_contract, params);
            } else {
                continue;
            }
        };

        let lp_token = ERC20::new(init_event.lp_token, client.clone());
        // Get the intended tokens for the pool and do approvals.
        let mut tokens: Vec<ArbiterToken<ArbiterMiddleware>> = Vec::new();
        for token in init_event.tokens {
            let token = ArbiterToken::new(token, client.clone());
            tokens.push(token);
        }

        let pool = Pool::<P> {
            id: init_event.pool_id,
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
        warn!("got to the end up the updator startup");
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
        warn!("Process: Got event: {:?}", event);
        let msg: UpdatoorQuerry = serde_json::from_str(&event.data).unwrap_or(UpdatoorQuerry::NoOp);

        warn!("Process: deserialized update querry: {:?}", msg);

        match msg {
            UpdatoorQuerry::UpdateMeDaddy => {
                let params = self.data.pool_params.pop().unwrap();
                self.data.pool.update(params.clone()).await?;
                let _ = self.data.messager.send(To::All, params).await?;
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
