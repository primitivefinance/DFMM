use futures_util::StreamExt;

use super::*;
use crate::behaviors::{deploy::DeploymentData, token::Response};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Create<S: State> {
    pub token_admin: String,
    pub data: S::Data,
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

#[async_trait::async_trait]
impl<P> Behavior<()> for Create<Config<P>>
where
    P: PoolType + Send + Sync + 'static,
    P::StrategyContract: Send,
    P::SolverContract: Send,
{
    type Processor = ();
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<()>)>> {
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
        debug!(
            "Setting Controller Address to self address: {:?}",
            client.address()
        );
        if self.data.base_config.controller == eAddress::zero() {
            self.data.base_config.controller = client.address();
        }
        debug!("creating pool...");
        // Create the pool.
        let pool = Pool::<P>::new(
            self.data.base_config.clone(),
            self.data.params.clone(),
            self.data.allocation_data.clone(),
            strategy_contract,
            solver_contract,
            dfmm,
            tokens,
        )
        .await?;

        debug!("Pool created!\n {:#?}", pool);

        // TODO: This won't actually work nicely on the receiving end as for whatever
        // reason, wrapping this into the enum breaks with the generic <P>. So we tuple
        // it up for now.
        // let pool_creation = PoolCreation::<P> {
        //     id: pool.id,
        //     params: self.data.params.clone(),
        //     allocation_data: self.data.allocation_data.clone(),
        // };
        let pool_creation = (
            pool.id,
            pool.tokens.iter().map(|t| t.address()).collect::<Vec<_>>(),
            pool.liquidity_token.address(),
            // TODO: This params will show the incorrect controller address
            // Would be nice to have it be the correct one set on line 88
            // so that the debug msg's are more helpful
            self.data.params.clone(),
            self.data.allocation_data.clone(),
        );
        messager.send(To::All, pool_creation).await.unwrap();
        Ok(None)
    }
}

// TODO: We should be able to use this but it is currently hard to work with due
// to `serde::Deserialize`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PoolCreation<P: PoolType> {
    pub id: eU256,
    pub params: P::Parameters,
    pub allocation_data: P::AllocationData,
}
