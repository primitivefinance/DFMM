use self::{bindings::dfmm::DFMM, deployer::DeploymentData, pool::BaseConfig};
use super::*;
use crate::behaviors::token_admin::Response;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Swap<S: State> {
    // to get tokens on start up
    pub token_admin: String,
    pub data: S::Data,
}

#[derive(Debug, Clone)]
pub struct SwapProcessing {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
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

impl State for SwapProcessing {
    type Data = Self;
}

#[async_trait::async_trait]
impl<P> Behavior<Message> for Creator<Config<P>>
where
    P: PoolType + Send + Sync + 'static,
    P::StrategyContract: Send,
    P::SolverContract: Send,
{
    type Processor = Swap<SwapProcessing>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        // Receive the `DeploymentData` from the `Deployer` agent and use it to get the
        // contracts.
        let deployment_data = messager.get_next::<DeploymentData>().await?.data;
        let (_strategy_contract, _solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client.clone());

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

        let process = Self::Processor {
            token_admin: self.token_admin.clone(),
            data: SwapProcessing { messager, client },
        };

        let stream = process.data.messager.clone().stream()?;
        Ok(Some((process, stream)))
    }
}

#[async_trait::async_trait]
impl Processor<Message> for Swap<SwapProcessing> {
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let _query: TokenAdminQuery =
            serde_json::from_str(&event.data).unwrap_or(TokenAdminQuery::NoOp);
        Ok(ControlFlow::Continue)
    }
}
