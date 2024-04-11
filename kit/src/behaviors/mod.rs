use std::sync::Arc;

use arbiter_engine::{
    machine::{Behavior, Configuration, ControlFlow, EventStream},
    messager::{Messager, To},
};
use arbiter_macros::Behaviors;
use bindings::arbiter_token::ArbiterToken;
use serde::{Deserialize, Serialize};

use self::{
    creator::{PoolConfig, PoolCreator},
    deployer::Deployer,
    pool::PoolType,
    token_admin::{TokenAdmin, TokenAdminConfig}, /* token_admin::TokenAdmin,
                                                  * allocate::InitialAllocation, */
};
use super::*;

// pub mod allocate;
pub mod creator;
pub mod deployer;
pub mod token_admin;

#[derive(Debug, Deserialize, Serialize)]
pub enum Behaviors<P: PoolType> {
    Creator(PoolCreator<Configuration<PoolConfig<P>>>),
    Deployer(Deployer),
    TokenAdmin(TokenAdmin<Configuration<TokenAdminConfig>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<eAddress>,
}
