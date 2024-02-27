use std::sync::Arc;

use arbiter_engine::{
    machine::{Behavior, CreateStateMachine, Engine, EventStream, StateMachine},
    messager::Messager,
};
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub use self::deployer::{Deployer, DeploymentData};
use super::*;

pub mod deployer;
pub mod initialize;
pub mod liquidity_provision;

#[derive(Behaviors, Debug, Deserialize, Serialize)]
pub enum Behaviors {
    Deployer(Deployer),
}
