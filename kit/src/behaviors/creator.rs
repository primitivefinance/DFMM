use super::*;
use crate::{behaviors::deployer::DeploymentData, pool::{constant_sum::ConstantSumPool, Pool, PoolType}};
use arbiter_engine::machine::{Behavior, Configuration, Processing, Processor, State};
use futures_util::StreamExt;
use bindings::{constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver, dfmm::DFMM};
use serde::de::DeserializeOwned;

// Idea: Let's make a behavior that has two states:
// State 1. This is for configuration and it should have everything be `Serialize`/`Deserialize` so that it can be read in from a config.
// State 2. This is the "built" version of the behavior that may now own client, messager, or contracts (etc.) and other things that had to be gotten from running the `startup` method.

// Example:
// Let's make a "pool_creator" type of behavior that will take some configuration for a pool and work to attempt to deploy that pool.

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
impl<P, E> Behavior<E> for PoolCreator<Configuration<PoolConfig<P>>>
where
    P: PoolType + Send + Sync + 'static,
    E: Send + Sync + 'static + DeserializeOwned,
{
    type Processor = PoolCreator<Processing<PoolProcessor<P>>>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<E>)>> {

        let mut stream = messager.stream()?;
        let res = stream.next().await.unwrap();
        let data: String =
                serde_json::from_str(&res.data).expect("Failed to
deserialize message data");
        let parsed_data: DeploymentData =
            serde_json::from_str(&data).expect("Failed to deserialize
token data");

        let token_x = ArbiterToken::new(parsed_data.token_x, client.clone());
        let token_y = ArbiterToken::new(parsed_data.token_y, client);
        let (strategy_contract, solver_contract) = P::get_contracts(&parsed_data, client);
        let dfmm = DFMM::new(parsed_data.dfmm, client);
        let init_data = self.data.initial_allocation_data.clone();
        let pool = P::create_pool(self.data.initial_allocation_data, vec![token_x, token_y], strategy_contract, solver_contract, dfmm).await?;
        trace!("Pool created at {:?}", pool.id);
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<P, E> Processor<E> for PoolCreator<Processing<PoolProcessor<P>>>
where
    P: PoolType + Send + Sync + 'static,
    E: Send + Sync + 'static + DeserializeOwned,
{
    async fn process(&mut self, _event: E) -> Result<ControlFlow> {
        Ok(ControlFlow::Halt)
    }
}

mod test {
    use std::str::FromStr;

    use arbiter_engine::{agent::Agent, world::World};
    use ethers::types::Address;
    use futures_util::StreamExt;
    use tracing_subscriber::FmtSubscriber;

    use crate::behaviors::deployer::{Deployer, DeploymentData};
    use crate::behaviors::Behaviors::Creator;
    use self::{bindings::constant_sum_solver::ConstantSumParams, pool::constant_sum::{ConstantSumInitData, ConstantSumPool}};

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn deployer_behavior_test() {
        let subscriber = FmtSubscriber::builder().finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let mut world = World::new("test");
        let messager = world.messager.clone();

        let agent = Agent::builder("token_admin_agent");
        let creator = Agent::builder("pool_creator_agent");
        world.add_agent(creator.with_behavior(PoolCreator::<Configuration<PoolConfig<ConstantSumPool>>> 
        { data: PoolConfig {
            params: ConstantSumParams {
                price: 0.into(),
                swap_fee: 0.into(),
                controller: Address::zero(),
            },
            initial_allocation_data: ConstantSumInitData {
                name: "Test Pool".to_string(),
                symbol: "TP".to_string(),
                reserve_x: 0.into(),
                reserve_y: 0.into(),
                token_x_name: "Token X".to_string(),
                token_y_name: "Token Y".to_string(),
                params: ConstantSumParams {
                    price: eU256::zero(),
                    swap_fee: eU256::zero(),
                    controller: Address::zero(),
                },
            },
            token_list: vec![Address::zero(), Address::zero()],
            }
        }));

        world.add_agent(agent.with_behavior(Deployer {}));

        world.run().await.unwrap();
    }
}