use super::*;
use crate::behaviors::token::Response;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Create<S: State> {
    pub token_admin: String,
    pub data: S::Data,
}

#[derive(Clone, Debug, Serialize, Deserialize, State)]
pub struct Config<P: PoolType> {
    pub base_config: BaseConfig,
    pub params: P::Parameters,
    pub allocation_data: P::AllocationData,
    pub token_list: Vec<String>,
}

#[async_trait::async_trait]
impl<P> Behavior<()> for Create<Config<P>>
where
    P: PoolType + Send + Sync + 'static,
{
    type Processor = ();
    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Self::Processor> {
        // Receive the `DeploymentData` from the `Deployer` agent and use it to get the
        // contracts.
        debug!("Starting the creator");
        let deployment_data = messager.get_next::<DeploymentData>().await?.data;

        debug!("Creator: Received deployment data {:?}", deployment_data);
        let (strategy_contract, solver_contract) =
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
        debug!(
            "Setting Controller Address to self address: {:?}",
            client.address()
        );
        if self.data.base_config.controller == eAddress::zero() {
            self.data.base_config.controller = client.address();
        }
        let params = P::set_controller(self.data.params.clone(), client.address());

        debug!("creating pool...");
        let pool = Pool::<P>::new(
            self.data.base_config.clone(),
            params.clone(),
            self.data.allocation_data.clone(),
            strategy_contract,
            solver_contract,
            dfmm,
            tokens,
        )
        .await?;

        debug!("Pool created!\n {:#?}", pool);

        messager
            .send(
                To::All,
                PoolCreation::<P> {
                    id: pool.id,
                    tokens: pool.tokens.iter().map(|t| t.address()).collect::<Vec<_>>(),
                    liquidity_token: pool.liquidity_token.address(),
                    params,
                    allocation_data: self.data.allocation_data.clone(),
                },
            )
            .await
            .unwrap();
        Ok(())
    }
}
