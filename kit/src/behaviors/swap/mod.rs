use self::{bindings::erc20::ERC20, pool::InputToken};
use super::*;
use crate::behaviors::token::Response;

pub trait SwapType<E> {
    fn compute_swap_amount(&self, event: E) -> (eU256, InputToken);
}

#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Swap<S, T: SwapType<E>, E>
where
    S: State,
{
    pub data: S::Data,
    pub swap_type: T,
    pub _phantom: PhantomData<E>,
}

pub trait SwapStream<SwapType, E> where E: Send + 'static {
    fn get_typed_stream(swap_type: SwapType, channel: Messager, client: Arc<ArbiterMiddleware>) -> Result<Option<EventStream<E>>>;
}


#[derive(Deserialize, Clone)]
pub struct SwapOnce {
    pub amount: eU256,
    pub input: InputToken,
}


impl<P, SwapOnce, Message> SwapStream<SwapOnce, Message> for Swap<Processing<P>, SwapOnce, Message>
where
    P: PoolType + Send + Sync,
{
    fn get_typed_stream(swap_type: SwapOnce, channel: Messager, client: Arc<ArbiterMiddleware>) -> Result<Option<EventStream<Message>>> {
        let thing = channel.stream()?;
        Ok(Some(thing))
    }
}

// TODO: This needs to be configurable in some way to make the `SwapType` become
// transparent and useful.
// Should also get some data necessary for mint amounts and what not.
#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Config<P>
where
    P: PoolType,
{
    pub token_admin: String,
    pub _phantom: PhantomData<P>,
}

#[derive(Debug, Clone, State)]
pub struct Processing<P>
where
    P: PoolType,
    // T: SwapType<E>,
    // E: Send + 'static,
{
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
    // pub swap_type: T,
    // _phantom: PhantomData<E>,
}

#[async_trait::async_trait]
impl<P, T, E> Behavior<E> for Swap<Config<P>, T, E>
where
    P: PoolType + Send + Sync,
    T: SwapType<E> + Send + Clone,
    E: Send + 'static,
{
    type Processor = Swap<Processing<P>, T, E>;
    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Self::Processor> {
        // TODO: Here we probably need to filter on the `PoolCreation` so that we get
        // the correct pool.
        let completed_todo = GetPoolTodo::<P>::complete(&mut messager).await;
        let (deployment_data, pool_creation) = (
            completed_todo.deployment_data.unwrap(),
            completed_todo.pool_creation.unwrap(),
        );

        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());

        // Get the intended tokens for the pool and do approvals.
        let mut tokens: Vec<ArbiterToken<ArbiterMiddleware>> = Vec::new();
        for token_address in pool_creation.tokens.into_iter() {
            let token = ArbiterToken::new(token_address, client.clone());
            let name = token.name().call().await?;
            messager
                .send(
                    To::Agent(self.data.token_admin.clone()),
                    TokenAdminQuery::MintRequest(MintRequest {
                        token: name,
                        mint_to: client.address(),
                        mint_amount: parse_ether(100)?,
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

        // build pool for processor and stream
        let pool = Pool::<P> {
            id: pool_creation.id,
            dfmm,
            instance: P::create_instance(strategy_contract, solver_contract, pool_creation.params),
            tokens,
            liquidity_token: ERC20::new(pool_creation.liquidity_token, client.clone()),
        };

        let processor = Self::Processor {
            data: Processing {
                messager,
                client,
                pool,
            },
            swap_type: self.swap_type,
            _phantom: PhantomData,
        };

        Ok(processor)
    }
}

#[async_trait::async_trait]
impl<P, T, E> Processor<E> for Swap<Processing<P>, T, E>
where
    P: PoolType + Send + Sync,
    T: SwapType<E> + Send + Clone,
    E: Send + 'static,
    Swap<Processing<P>, T, E>: SwapStream<T, E>,
{
    async fn get_stream(&mut self) -> Result<Option<EventStream<E>>> {
        behaviors::swap::Swap::<behaviors::swap::Processing<P>, T, E>::get_typed_stream(self.swap_type.clone(), self.data.messager.clone(), self.data.client)
    }
    async fn process(&mut self, event: E) -> Result<ControlFlow> {
        let (swap_amount, input) = self.swap_type.compute_swap_amount(event);
        self.data.pool.swap(swap_amount, input).await?;
        Ok(ControlFlow::Continue)
    }
}
