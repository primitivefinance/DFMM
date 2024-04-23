use self::{bindings::erc20::ERC20, pool::InputToken};
use super::*;
use crate::behaviors::token::Response;

pub trait SwapType<E>: Debug + Serialize + Clone {
    fn compute_swap_amount(event: E) -> (eU256, InputToken);
    // TODO: Put this on the processor in arbiter engine so that startups just
    // return a proccess
    fn get_stream(
        &self,
        _messager: Messager,
    ) -> Option<Pin<Box<dyn Stream<Item = E> + Send + Sync>>> {
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Swap<S: State, T: SwapType<E>, E> {
    // to get tokens on start up
    pub token_admin: String,
    pub update: String,
    pub data: S::Data,
    pub swap_type: T,
    pub _phantom: PhantomData<E>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config<P: PoolType> {
    pub base_config: BaseConfig,
    pub params: P::Parameters,
    pub allocation_data: P::AllocationData,
    pub token_list: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Processing<P: PoolType> {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
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
struct SwapTodo<P: PoolType> {
    deployment_data: Option<DeploymentData>,
    #[allow(clippy::type_complexity)]
    pool_creation: Option<(
        PoolId,         // Pool ID
        TokenList,      // Token List
        LiquidityToken, // Liquidity Token
        <P as PoolType>::Parameters,
        <P as PoolType>::AllocationData,
    )>,
}

#[async_trait::async_trait]
impl<P, T, E> Behavior<E> for Swap<Config<P>, T, E>
where
    P: PoolType + Send + Sync + 'static,
    P::StrategyContract: Send,
    P::SolverContract: Send,
    T: SwapType<E> + Send + Sync + 'static + for<'a> Deserialize<'a>,
    E: Debug + Send + Sync + 'static,
{
    type Processor = Swap<Processing<P>, T, E>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<E>)>> {
        // Make a "TODO" list.
        // This is the data I need to recieve to do my job
        let mut todo: SwapTodo<P> = SwapTodo {
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

        // Get the intended tokens for the pool and do approvals.
        let mut tokens: Vec<ArbiterToken<ArbiterMiddleware>> = Vec::new();
        for tkn in self.data.token_list.drain(..) {
            messager
                .send(
                    To::Agent(self.token_admin.clone()),
                    TokenAdminQuery::AddressOf(tkn.clone()),
                )
                .await
                .unwrap();
            let token = ArbiterToken::new(
                messager.get_next::<eAddress>().await.unwrap().data,
                client.clone(),
            );
            messager
                .send(
                    To::Agent(self.token_admin.clone()),
                    TokenAdminQuery::MintRequest(MintRequest {
                        token: tkn,
                        mint_to: client.address(),
                        mint_amount: 100_000_000_000,
                    }),
                )
                .await
                .unwrap();
            assert_eq!(
                messager.get_next::<Response>().await.unwrap().data,
                Response::Success
            );
            token
                .approve(dfmm.address(), MAX)
                .send()
                .await
                .unwrap()
                .await
                .unwrap();

            tokens.push(token);
        }

        let lp_address = todo.pool_creation.clone().unwrap().2;
        let lp_token = ERC20::new(lp_address, client.clone());
        let instance = P::create_instance(
            strategy_contract,
            solver_contract,
            todo.pool_creation.clone().unwrap().3,
        );

        // build pool for processor and stream
        let pool = Pool::<P> {
            id: todo.pool_creation.clone().unwrap().0,
            dfmm,
            instance,
            tokens,
            liquidity_token: lp_token,
        };

        match self.swap_type.get_stream(messager.clone()) {
            Some(stream) => {
                let process = Self::Processor {
                    token_admin: self.token_admin.clone(),
                    update: self.update.clone(),
                    data: Processing {
                        messager,
                        client,
                        pool,
                    },
                    swap_type: self.swap_type.clone(),
                    _phantom: PhantomData::<E>,
                };
                Ok(Some((process, stream)))
            }
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl<P, T, E> Processor<E> for Swap<Processing<P>, T, E>
where
    P: PoolType + Send + Sync,
    T: SwapType<E> + Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    async fn process(&mut self, event: E) -> Result<ControlFlow> {
        let (swap_amount, input) = T::compute_swap_amount(event);
        self.data.pool.swap(swap_amount, input).await?;
        Ok(ControlFlow::Continue)
    }
}
