use arbiter_engine::machine::{Behavior, State};
use bindings::dfmm::DFMM;

use self::pool::BaseConfig;
use super::*;
use crate::{
    behaviors::{
        deployer::DeploymentData,
        token_admin::{Response, TokenAdminQuery},
    },
    pool::{Pool, PoolType},
};

pub const MAX: eU256 = eU256::MAX;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator<S: State> {
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
impl<P> Behavior<()> for Creator<Config<P>>
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
        // Receive the `DeploymentData` from the `Deployer` agent.
        let deployment_data = messager.get_next::<DeploymentData>().await?.data;

        // Get all the tokens from the `TokenAdmin` agent.
        let token_admin = "token_admin_agent".to_owned();
        messager
            .send(
                To::Agent(token_admin.clone()),
                TokenAdminQuery::GetAssetUniverse,
            )
            .await?;
        let universe = messager
            .get_next::<Vec<(TokenData, eAddress)>>()
            .await?
            .data;
        let token_x = ArbiterToken::new(universe[0].clone().1, client.clone());
        let token_y = ArbiterToken::new(universe[1].clone().1, client.clone());

        // Get the `TokenAdmin` to mint us enough tokens to create the pool.
        let mint_x = TokenAdminQuery::MintRequest(MintRequest {
            token: universe[0].clone().0.name,
            mint_to: client.address(),
            mint_amount: 100_000_000_000,
        });
        let mint_y = TokenAdminQuery::MintRequest(MintRequest {
            token: universe[1].clone().0.name,
            mint_to: client.address(),
            mint_amount: 100_000_000_000,
        });
        messager
            .send(To::Agent(token_admin.clone()), mint_x)
            .await?;
        messager.send(To::Agent(token_admin), mint_y).await?;
        let mint_x_response = messager.get_next::<Response>().await?.data;
        let mint_y_response = messager.get_next::<Response>().await?.data;
        assert_eq!(mint_x_response, Response::Success);
        assert_eq!(mint_y_response, Response::Success);

        // Go to deploy the pool.
        let (strategy_contract, solver_contract) =
            P::get_contracts(&deployment_data, client.clone());
        let dfmm = DFMM::new(deployment_data.dfmm, client);

        token_x
            .clone()
            .approve(dfmm.address(), MAX)
            .send()
            .await?
            .await?;
        token_y
            .clone()
            .approve(dfmm.address(), MAX)
            .send()
            .await?
            .await?;

        let _pool = Pool::<P>::new(
            self.data.base_config.clone(),
            self.data.params.clone(),
            self.data.allocation_data.clone(),
            strategy_contract,
            solver_contract,
            dfmm,
            vec![token_x, token_y],
        )
        .await?;
        Ok(None)
    }
}
