use arbiter_core::events::stream_event;
use futures_util::StreamExt;

use self::{
    bindings::{dfmm::DFMM, erc20::ERC20},
    creator::PoolCreation,
    deployer::DeploymentData,
    pool::{BaseConfig, InputToken, Pool},
};
use super::*;
use crate::behaviors::token_admin::Response;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Swap<S: State, T: SwapType> {
    // to get tokens on start up
    pub token_admin: String,
    pub data: S::Data,
    pub swap_type: T,
}

#[derive(Debug, Clone)]
pub struct SwapProcessing<P: PoolType> {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub pool: Pool<P>,
}

pub trait SwapType: std::fmt::Debug + Serialize + Clone {
    fn compute_swap_amount() -> (eU256, InputToken);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config<P: PoolType> {
    pub base_config: BaseConfig,
    pub params: P::Parameters,
    pub allocation_data: P::AllocationData,
    pub token_list: Vec<String>,
}

impl<P: PoolType> State for Config<P> {
    type Data = Self;
}

impl<P> State for SwapProcessing<P>
where
    P: PoolType,
{
    type Data = Self;
}

#[async_trait::async_trait]
impl<P, T> Behavior<Message> for Swap<Config<P>, T>
where
    P: PoolType + Send + Sync + 'static,
    P::StrategyContract: Send,
    P::SolverContract: Send,
    T: SwapType + Send + Sync + 'static + for<'a> Deserialize<'a>,
{
    type Processor = Swap<SwapProcessing<P>, T>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        // Receive the `DeploymentData` from the `Deployer` agent and use it to get the
        // contracts.
        let deployment_data = messager.get_next::<DeploymentData>().await?.data;
        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());
        let mut init_event_steream = stream_event(dfmm.init_filter());

        // Get the intended tokens for the pool and do approvals.
        let mut tokens = Vec::new();
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
        let init_event = init_event_steream.next().await.unwrap();
        let pool_creation = messager.get_next::<PoolCreation<P>>().await?.data;
        let lp_token = ERC20::new(init_event.lp_token, client.clone());
        let instance = P::create_instance(strategy_contract, solver_contract, pool_creation.params);

        let pool = Pool::<P> {
            id: pool_creation.id,
            dfmm,
            instance,
            tokens,
            liquidity_token: lp_token,
        };

        let process = Self::Processor {
            token_admin: self.token_admin.clone(),
            data: SwapProcessing {
                messager,
                client,
                pool,
            },
            swap_type: self.swap_type.clone(),
        };

        let stream = process.data.messager.clone().stream()?;
        Ok(Some((process, stream)))
    }
}

#[async_trait::async_trait]
impl<P, T> Processor<Message> for Swap<SwapProcessing<P>, T>
where
    P: PoolType + Send + Sync,
    T: SwapType + Send + Sync + 'static,
{
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        // todo, swap only on the right event trigger
        let (swap_amount, input) = T::compute_swap_amount();
        let swap = self.data.pool.swap(swap_amount, input).await?;

        Ok(ControlFlow::Continue)
    }
}
