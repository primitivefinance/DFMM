use super::*;
use crate::pool::InputToken;

// TODO: This could depend on the `PoolType` as the `AllocateType` does.
pub trait SwapType<E> {
    fn compute_swap_amount(&self, event: E) -> Option<(eU256, InputToken)>;
}

// TODO: This may not want to ever have a phantom data in it, but this is
// working for now.
#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Swap<S, T: SwapType<E>, E>
where
    S: State,
{
    pub data: S::Data,
    pub swap_type: T,
    pub _phantom: PhantomData<E>,
}

// Should also get some data necessary for mint amounts and what not.
#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Config<P>
where
    P: PoolType,
{
    pub token_admin: String,
    pub _phantom: PhantomData<P>,
}

// TODO: This start up is also very much the same as the `Allocate` start up.
// More than likely, `Update`, `Swap`, and `Allocate` can all be combined into
// one type of behavior like a `Interact` behavior that just specializes to do
// different things.
#[async_trait::async_trait]
impl<P, T, E> Behavior<E> for Swap<Config<P>, T, E>
where
    P: PoolType + Send + Sync,
    T: SwapType<E> + Send + Clone,
    E: Send + 'static,
{
    type Processor = Swap<PoolProcessing<P>, T, E>;
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

        // TODO: This sort of approval and token loop is also repeated in other places
        // like `Allocate` and `Create`.
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
            data: PoolProcessing {
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

/// This is the default implementation for any processor that takes in some
/// event E and will work for the `Swap` struct.
#[async_trait::async_trait]
impl<P, T, E> Processor<E> for Swap<PoolProcessing<P>, T, E>
where
    P: PoolType + Send + Sync,
    T: SwapType<E> + Send + Clone,
    E: Send + 'static,
{
    default async fn get_stream(&mut self) -> Result<Option<EventStream<E>>> {
        Ok(None)
    }

    default async fn process(&mut self, event: E) -> Result<ControlFlow> {
        if let Some((swap_amount, input)) = self.swap_type.compute_swap_amount(event) {
            debug!("Found the swap amounts: {:?}", swap_amount);
            self.data.pool.swap(swap_amount, input).await?;
        }

        Ok(ControlFlow::Continue)
    }
}

/// With the `specialization` feature in Rust, we can take the above trait and
/// use it as a default when we don't want to have some specific kind of stream
/// come with it. Sadly, we still have to copy the `process` method along with
/// the `get_stream`, ideally, in the future, all you have to do is just
/// implement your own `get_stream` given whatever event `E` you want to
/// produce, and this will be a very straightforward specialization that allows you to easily extend `Swap` for whatever swap type you want. See this tracking RFC for trait `specialization` https://github.com/rust-lang/rust/issues/31844
///
/// Just to be clear, this now will allow you to work with any `Swap` and
/// `SwapType` as long as it streams `Message`s. If you need to stream something
/// else, just copy this specialization and use whatever stream item you'd like!
#[async_trait::async_trait]
impl<P, T> Processor<Message> for Swap<PoolProcessing<P>, T, Message>
where
    P: PoolType + Send + Sync,
    T: SwapType<Message> + Send + Clone,
{
    // This is the specialized trait for the specific type E = Message. Cool!
    async fn get_stream(&mut self) -> Result<Option<EventStream<Message>>> {
        Ok(Some(self.data.messager.stream()?))
    }

    // Would be nice to nice to not have to rewrite this every time see above... RIP
    default async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        if let Some((swap_amount, input)) = self.swap_type.compute_swap_amount(event) {
            self.data.pool.swap(swap_amount, input).await?;
        }

        Ok(ControlFlow::Continue)
    }
}
