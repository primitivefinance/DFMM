use std::sync::Arc;

use arbiter_engine::{
    machine::{
        Behavior, Configuration, ControlFlow, CreateStateMachine, Engine, EventStream, Processing,
        Processor, State, StateMachine,
    },
    messager::Messager,
};
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

use self::{
    deployer::Deployer,
    pool::{Pool, PoolConfigurer, PoolType},
};
use super::*;

pub mod deployer;

#[derive(Behaviors, Debug, Deserialize, Serialize)]
pub enum Behaviors {
    Deployer(Deployer),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBehavior<S: State> {
    pub data: S::Data,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TestBehaviorConfig<P: PoolConfigurer> {
    pub params: P::Parameters,
}

pub struct TestBehaviorProcessor<P: PoolType> {
    pub pool: Pool<P>,
}

#[async_trait::async_trait]
impl<P: PoolConfigurer> Behavior<()> for TestBehavior<Configuration<TestBehaviorConfig<P>>> {
    type Processor = ();
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<((), EventStream<()>)>> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<P: PoolType> Processor<()> for TestBehavior<Processing<TestBehaviorProcessor<P>>> {
    async fn process(&mut self, _event: ()) -> Result<ControlFlow> {
        Ok(ControlFlow::Halt)
    }
}
