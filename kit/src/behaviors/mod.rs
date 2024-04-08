use std::sync::Arc;

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_engine::{
    machine::{Behavior, ControlFlow, CreateStateMachine, Engine, EventStream, StateMachine},
    messager::{Message, Messager, To},
};
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

use self::{allocate::InitialAllocation, deployer::Deployer, pool::PoolType, token_admin::TokenAdmin};
use super::*;

pub mod allocate;
pub mod deployer;
pub mod token_admin;

#[derive(Behaviors, Debug, Deserialize, Serialize)]
pub enum Behaviors<P> {
    Allocate(InitialAllocation<P>),
    Deployer(Deployer),
    TokenAdmin(TokenAdmin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<eAddress>,
}
