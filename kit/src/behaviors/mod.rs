use std::{boxed::Box, marker::PhantomData, pin::Pin, sync::Arc};

use arbiter_core::events::stream_event;
use arbiter_engine::{
    machine::{Behavior, ControlFlow, EventStream, Processor, State},
    messager::{Message, Messager, To},
};
#[allow(unused)]
use arbiter_macros::Behaviors;
use bindings::{arbiter_token::ArbiterToken, dfmm::DFMM};
use futures_util::{Stream, StreamExt};
pub use token::{MintRequest, TokenAdminQuery};

use self::{creator::Create, deploy::{Deploy, DeploymentData}, pool::PoolType, token::TokenAdmin};
use super::*;

pub const MAX: eU256 = eU256::MAX;

pub mod allocate;
pub mod creator;
pub mod deploy;
pub mod swap;
pub mod token;
pub mod update;

#[derive(Debug, Deserialize, Serialize)]
pub enum Behaviors<P: PoolType> {
    Create(Create<creator::Config<P>>),
    Deployer(Deploy),
    TokenAdmin(TokenAdmin<token::Config>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageTypes<P> where P: PoolType {
    #[serde(untagged)]
    Deploy(DeploymentData),
    #[serde(untagged)]
    Create(creator::PoolCreation<P>),
    #[serde(untagged)]
    TokenAdmin(token::Response),
    #[serde(untagged)]
    Update(P::Parameters),
}