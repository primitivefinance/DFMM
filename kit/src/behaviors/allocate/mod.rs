use self::pool::AllocateOrDeallocate;
use super::*;

pub trait AllocateType<P, E>
where
    P: PoolType,
    E: Send + 'static,
{
    fn change_allocation_amount(
        &mut self,
        event: E,
    ) -> Option<(AllocateOrDeallocate, P::AllocationData)>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Allocate<S, A, P, E>
where
    S: State,
    A: AllocateType<P, E>,
    P: PoolType,
    E: Send + 'static,
{
    pub data: S::Data,
    pub allocate_type: A,
    _phantom_e: PhantomData<E>,
    _phantom_p: PhantomData<P>,
}

// TODO: This is actually the exact same as the `swap::Config` so... maybe they
// can be combined.
#[derive(Debug, Serialize, Deserialize, State)]
pub struct Config<P: PoolType> {
    pub token_admin: String,
    pub _phantom: PhantomData<P>,
}

// TODO: This start up is also very much the same as the `Swap` start up. More
// than likely, `Update`, `Swap`, and `Allocate` can all be combined into one
// type of behavior like a `Interact` behavior that just specializes to do
// different things.
#[async_trait::async_trait]
impl<A, P, E> Behavior<E> for Allocate<Config<P>, A, P, E>
where
    A: AllocateType<P, E> + Send,
    P: PoolType + Send + Sync,
    E: Send + 'static,
{
    type Processor = Allocate<PoolProcessing<P>, A, P, E>;
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
        // like `Swap` and `Allocate`.
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
            allocate_type: self.allocate_type,
            _phantom_e: PhantomData,
            _phantom_p: PhantomData,
        };

        Ok(processor)
    }
}

#[async_trait::async_trait]
impl<A, P, E> Processor<E> for Allocate<PoolProcessing<P>, A, P, E>
where
    A: AllocateType<P, E> + Send,
    P: PoolType + Send + Sync,
    E: Send + 'static,
{
    default async fn get_stream(&mut self) -> Result<Option<EventStream<E>>> {
        Ok(None)
    }
    default async fn process(&mut self, event: E) -> Result<ControlFlow> {
        if let Some((allocate_or_deallocate, allocation_data)) =
            self.allocate_type.change_allocation_amount(event)
        {
            self.data
                .pool
                .change_allocation(allocate_or_deallocate, allocation_data)
                .await?;
        }

        Ok(ControlFlow::Continue)
    }
}
