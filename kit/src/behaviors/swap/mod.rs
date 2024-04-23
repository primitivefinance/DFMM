use self::{bindings::erc20::ERC20, pool::InputToken};
use super::*;
use crate::behaviors::token::Response;

pub trait SwapType<E> {
    fn compute_swap_amount(event: E) -> (eU256, InputToken);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Swap<S, T, E>
where
    S: State,
    T: SwapType<E>,
{
    // to get tokens on start up
    pub token_admin: String,
    pub update: String,
    pub data: S::Data,
    pub swap_type: T,
    pub _phantom: PhantomData<E>,
}

// TODO: This needs to be configurable in some way to make the `SwapType` become
// transparent and useful.
// Should also get some data necessary for mint amounts and what not.
#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Config<P: PoolType> {
    phantom: PhantomData<P>,
}

impl<P: PoolType> Default for Config<P> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, State)]
pub struct Processing<P: PoolType> {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
}

#[async_trait::async_trait]
impl<P, T, E> Behavior<()> for Swap<Config<P>, T, E>
where
    P: PoolType + Send,
    T: SwapType<E> + Send,
    E: Send,
{
    // type Processor = Swap<Processing<P>, T, E>;
    type Processor = ();
    async fn startup(
        &mut self,
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
                    To::Agent(self.token_admin.clone()),
                    TokenAdminQuery::MintRequest(MintRequest {
                        token: name,
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

        // build pool for processor and stream
        let _pool = Pool::<P> {
            id: pool_creation.id,
            dfmm,
            instance: P::create_instance(strategy_contract, solver_contract, pool_creation.params),
            tokens,
            liquidity_token: ERC20::new(pool_creation.liquidity_token, client.clone()),
        };
        // TODO: We need to come back around and adjust this.
        // match self.swap_type.get_stream(messager.clone()) {
        //     Some(stream) => {
        //         let process = Self::Processor {
        //             token_admin: self.token_admin.clone(),
        //             update: self.update.clone(),
        //             data: Processing {
        //                 messager,
        //                 client,
        //                 pool,
        //             },
        //             swap_type: self.swap_type.clone(),
        //             _phantom: PhantomData::<E>,
        //         };
        //         Ok(Some((process, stream)))
        //     }
        //     None => Ok(None),
        // }
        Ok(())
    }
}

#[async_trait::async_trait]
impl<P, T, E> Processor<E> for Swap<Processing<P>, T, E>
where
    P: PoolType + Send + Sync,
    T: SwapType<E> + Send,
    E: Send + 'static,
{
    async fn get_stream(&mut self) -> Result<Option<EventStream<E>>> {
        todo!("We have not implemented the 'get_stream' method yet for the 'Swap' behavior.")
    }
    async fn process(&mut self, event: E) -> Result<ControlFlow> {
        let (swap_amount, input) = T::compute_swap_amount(event);
        self.data.pool.swap(swap_amount, input).await?;
        Ok(ControlFlow::Continue)
    }
}
