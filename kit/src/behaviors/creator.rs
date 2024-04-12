use arbiter_engine::{
    machine::{Behavior, Configuration, Processing, Processor, State},
    universe,
};
use bindings::dfmm::DFMM;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use tracing::debug;

use super::*;
use crate::{
    behaviors::{deployer::DeploymentData, token_admin::{Response, TokenAdminQuery}},
    bindings::dfmm,
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
pub struct PoolCreator<S: State> {
    pub data: S::Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolConfig<P: PoolType> {
    pub params: P::PoolParameters,
    pub initial_allocation_data: P::InitializationData,
    pub token_list: Vec<eAddress>,
}

pub struct PoolProcessor<P: PoolType> {
    pub pool: Pool<P>,
}

#[async_trait::async_trait]
impl<P> Behavior<()> for PoolCreator<Configuration<PoolConfig<P>>>
where
    P: PoolType + Send + Sync + 'static,
    P::StrategyContract: Send,
    P::SolverContract: Send,
{
    // type Processor = PoolCreator<Processing<PoolProcessor<P>>>;
    type Processor = ();
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<()>)>> {
        let mut stream = messager.clone().stream().unwrap();
        let res = stream.next().await.unwrap();
        let data: String = serde_json::from_str(&res.data).expect(
            "Failed to
deserialize message data",
        );
        let parsed_data: DeploymentData = serde_json::from_str(&data).expect(
            "Failed to deserialize
token data",
        );

        let _ = messager
            .send(
                To::Agent("token_admin_agent".to_owned()),
                TokenAdminQuery::GetAssetUniverse,
            )
            .await?;
        let res = stream.next().await.unwrap();
        let universe: Vec<(TokenData, eAddress)> =
            serde_json::from_str(&res.data).expect("failed to serde");

        let token_x = ArbiterToken::new(universe[0].clone().1, client.clone());
        let token_y = ArbiterToken::new(universe[1].clone().1, client.clone());

        let mint_x = MintRequest {
            token: universe[0].clone().0.name,
            mint_to: client.address(),
            mint_amount: 100_000_000_000,
        };

        let mint_y = MintRequest {
            token: universe[1].clone().0.name,
            mint_to: client.address(),
            mint_amount: 100_000_000_000,
        };

        let _ = messager
            .send(To::Agent("token_admin_agent".to_owned()), mint_x)
            .await?;
        let _ = messager
            .send(To::Agent("token_admin_agent".to_owned()), mint_y)
            .await?;
        /// These work ^^

        let res0 = stream.next().await.unwrap();
        let res1 = stream.next().await.unwrap();
        let res0: Response =
        serde_json::from_str(&res0.data).expect("failed to serde");
        let res1: Response =
        serde_json::from_str(&res1.data).expect("failed to serde");

        debug!("Mints res0 {:?}", res0);
        debug!("Mints res1 {:?}", res1);

        assert_eq!(res0, Response::Success);
        assert_eq!(res1, Response::Success);
        //

        let (strategy_contract, solver_contract) = P::get_contracts(&parsed_data, client.clone());
        let dfmm = DFMM::new(parsed_data.dfmm, client);

        let _ = token_x
            .clone()
            .approve(dfmm.address(), MAX)
            .send()
            .await?
            .await?;
        let _ = token_y
            .clone()
            .approve(dfmm.address(), MAX)
            .send()
            .await?
            .await?;

        let init_data = self.data.initial_allocation_data.clone();
        debug!("Got to before pool deployment");
        let pool = P::create_pool(
            init_data,
            vec![token_x, token_y],
            strategy_contract,
            solver_contract,
            dfmm,
        )
        .await?;
        trace!("Pool created at {:?}", pool.id);
        Ok(None)
    }
}

// #[async_trait::async_trait]
// impl<P, E> Processor<E> for PoolCreator<Processing<PoolProcessor<P>>>
// where
//     P: PoolType + Send + Sync + 'static,
//     E: Send + Sync + 'static + DeserializeOwned,
// {
//     async fn process(&mut self, _event: E) -> Result<ControlFlow> {
//         Ok(ControlFlow::Halt)
//     }
// }

mod test {
    use std::{str::FromStr, sync::WaitTimeoutResult};

    use arbiter_engine::{agent::Agent, world::World};
    use ethers::types::Address;
    use futures_util::StreamExt;
    use tracing::{level_filters::LevelFilter, Level};
    use tracing_subscriber::FmtSubscriber;

    use self::{
        bindings::constant_sum_solver::ConstantSumParams,
        pool::constant_sum::{ConstantSumInitData, ConstantSumPool},
    };
    use super::*;
    use crate::behaviors::{
        deployer::{Deployer, DeploymentData},
        Behaviors::Creator,
    };

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn deployer_behavior_test() {
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
