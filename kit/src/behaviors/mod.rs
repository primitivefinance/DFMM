use std::sync::Arc;

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_engine::{
    machine::{
        Behavior, Configuration, ControlFlow, CreateStateMachine, Engine, EventStream, StateMachine,
    },
    messager::{Message, Messager, To},
};
use arbiter_macros::Behaviors;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::{
    creator::{PoolConfig, PoolCreator},
    deployer::Deployer,
    // bindings::idfmm::LogNormalParams, // TODO: We might want to just use these if we can.
    // pool::log_normal::LogNormalParams,
    pool::{PoolConfigurer, PoolType}, //token_admin::TokenAdmin,allocate::InitialAllocation,
};
use super::*;

// pub mod allocate;
pub mod deployer;
// pub mod token_admin;
pub mod creator;

#[derive(Debug, Deserialize, Serialize)]
pub enum Behaviors<PC: PoolType> {
    Creator(PoolCreator<Configuration<PoolConfig<PC>>>),
    Deployer(Deployer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<eAddress>,
}
