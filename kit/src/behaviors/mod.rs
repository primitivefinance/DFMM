use std::sync::Arc;

use arbiter_engine::{
    machine::{Behavior, CreateStateMachine, Engine, EventStream, StateMachine},
    messager::Messager,
};
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

use self::{allocate::Allocate, deployer::Deployer};
use super::*;

pub mod allocate;
pub mod deployer;

#[derive(Behaviors, Debug, Deserialize, Serialize)]
pub enum Behaviors {
    Allocate(Allocate),
    Deployer(Deployer),
}
