use std::sync::Arc;

use arbiter_engine::{
    machine::{Behavior, ControlFlow, EventStream, Processor, State},
    messager::{Message, Messager, To},
};
#[allow(unused)]
use arbiter_macros::Behaviors;
use bindings::arbiter_token::ArbiterToken;
pub use token_admin::{MintRequest, TokenAdminQuery};

use self::{creator::Creator, deployer::Deployer, pool::PoolType, token_admin::TokenAdmin};
use super::*;

pub const MAX: eU256 = eU256::MAX;

// pub mod allocate;
pub mod creator;
pub mod deployer;
pub mod swap;
pub mod token_admin;

#[derive(Debug, Deserialize, Serialize)]
pub enum Behaviors<P: PoolType> {
    Creator(Creator<creator::Config<P>>),
    Deployer(Deployer),
    TokenAdmin(TokenAdmin<token_admin::Config>),
}
