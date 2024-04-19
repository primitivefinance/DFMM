use self::{
    bindings::erc20::ERC20, creator::PoolCreation, pool::InputToken,
    update::UpdatoorQuerry,
};
use super::*;
use crate::behaviors::token::Response;

pub trait SwapType<E>: Debug + Serialize + Clone {
    fn compute_swap_amount(event: E) -> (eU256, InputToken);
    fn get_stream(&self) -> Pin<Box<dyn Stream<Item = E> + Send + Sync>>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Swap<S: State, T: SwapType<E>, E> {
    // to get tokens on start up
    pub token_admin: String,
    pub update: String,
    pub data: S::Data,
    pub swap_type: T,
    _phantom: PhantomData<E>,
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
        // Receive the `DeploymentData` from the `Deployer` agent and use it to get the
        // contracts.
        let deployment_data = messager.get_next::<DeploymentData>().await?.data;
        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());
        let mut init_event_stream = stream_event(dfmm.init_filter());

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

        // Note: Would be nice to get one of these note both?
        let init_event = init_event_stream.next().await.unwrap();
        let pool_creation = messager.get_next::<PoolCreation<P>>().await?.data;
        let lp_token = ERC20::new(init_event.lp_token, client.clone());
        let instance = P::create_instance(strategy_contract, solver_contract, pool_creation.params);

        // ask for first price udpate
        messager
            .send(
                To::Agent(self.update.to_owned()),
                UpdatoorQuerry::UpdateMeDaddy,
            )
            .await?;

        // build pool for processor and stream
        let pool = Pool::<P> {
            id: pool_creation.id,
            dfmm,
            instance,
            tokens,
            liquidity_token: lp_token,
        };

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

        let stream = self.swap_type.get_stream();
        Ok(Some((process, stream)))
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
