use arbiter_engine::machine::{Behavior, State};
use bindings::dfmm::DFMM;
use tracing::debug;

use self::pool::PoolConfig;
use super::*;
use crate::{
    behaviors::{
        deployer::DeploymentData,
        token_admin::{Response, TokenAdminQuery},
    },
    bindings::idfmm::InitParams,
    pool::{Pool, PoolType},
};

// Idea: Let's make a behavior that has two states:
// State 1. This is for configuration and it should have everything be
// `Serialize`/`Deserialize` so that it can be read in from a config.
// State 2. This is the "built" version of the behavior that may now own client,
// messager, or contracts (etc.) and other things that had to be gotten from
// running the `startup` method.

// Example:
// Let's make a "pool_creator" type of behavior that will take some
// configuration for a pool and work to attempt to deploy that pool.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator<S: State> {
    pub data: S::Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config<P: PoolType> {
    pub name: String,
    pub symbol: String,
    pub params: P::PoolParameters,
    pub init_config: P::InitConfig,
    pub token_list: Vec<eAddress>,
}

impl<P: PoolType> PoolConfig for Config<P> {
    fn get_init_params(&self) -> InitParams {
        InitParams {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            strategy: todo!(),
            tokens: self.token_list.clone(),
            data: todo!(),
            fee_collector: todo!(),
            controller_fee: todo!(),
        }
    }
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

        // let init_bytes = solver_contract
        //     .get_initial_pool_data(
        //         &self.data.init_config.reserve_x,
        //         &self.data.init_config.reserve_y,
        //         &self.data.init_config.params.clone(),
        //     )
        //     .call()
        //     .await?;
        let params = P::InitConfig::get_init_params(
            &self.data.init_config,
            deployment_data.constant_sum,
            vec![token_x.address(), token_y.address()], todo!()
        );

        let pool = dfmm.init(params).send().await?.await?.unwrap();
        debug!(
            "Pool created with status {:?}, (1 means success)",
            pool.status
        );
        Ok(None)
    }
}

mod test {
    use arbiter_engine::{agent::Agent, world::World};
    use tracing::{level_filters::LevelFilter, Level};
    use tracing_subscriber::FmtSubscriber;

    use super::*;
    use crate::behaviors::deployer::Deployer;

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn creator_behavior_test() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .pretty()
            .finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let mut world = World::new("test");

        // deployer
        let agent = Agent::builder("deployer");
        world.add_agent(agent.with_behavior(Deployer {}));

        // Token Admin
        let token_admin_config = default_admin_config();
        let token_admin = Agent::builder("token_admin_agent");
        world.add_agent(token_admin.with_behavior(token_admin_config));

        // Pool Creator
        let creator = Agent::builder("pool_creator_agent");
        let pool_creator_config = default_creator_config();
        world.add_agent(creator.with_behavior(pool_creator_config));

        world.run().await.unwrap();
    }
}
