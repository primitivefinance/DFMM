use std::{boxed::Box, marker::PhantomData, sync::Arc};

use arbiter_engine::{
    machine::{Behavior, ControlFlow, EventStream, Processor, State},
    messager::{Message, Messager, To},
};
#[allow(unused)]
use arbiter_macros::{Behaviors, State};
use bindings::{arbiter_token::ArbiterToken, dfmm::DFMM};
use ethers::utils::parse_ether;
pub use token::{MintRequest, TokenAdminQuery};

use self::{
    creator::Create,
    deploy::{Deploy, DeploymentData},
    pool::{PoolCreation, PoolType},
    token::TokenAdmin,
};
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
    Swap(swap::Config<P>),
}

pub trait Configurable<T: for<'a> Deserialize<'a>> {
    fn configure(data: T) -> Self;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "P: PoolType")]
pub enum MessageTypes<P>
where
    P: PoolType,
{
    #[serde(untagged)]
    Deploy(DeploymentData),
    #[serde(untagged)]
    Create(PoolCreation<P>),
    #[serde(untagged)]
    TokenAdmin(token::Response),
    #[serde(untagged)]
    Update(P::Parameters),
}

#[derive(Debug)]
struct GetPoolTodo<P: PoolType> {
    deployment_data: Option<DeploymentData>,
    pool_creation: Option<PoolCreation<P>>,
}

impl<P: PoolType> GetPoolTodo<P> {
    async fn complete(messager: &mut Messager) -> Self {
        // Make an undone "TODO" list.
        let mut todo: GetPoolTodo<P> = GetPoolTodo {
            deployment_data: None,
            pool_creation: None,
        };
        let id = messager.id.clone();
        // Loop through the messager until we check off the boxes for this TODO list.
        debug!("{:#?} is looping through their TODO list.", id.clone());
        loop {
            if let Ok(msg) = messager.get_next::<MessageTypes<P>>().await {
                match msg.data {
                    MessageTypes::Deploy(deploy_data) => {
                        debug!("Updater: Got deployment data: {:?}", deploy_data);
                        todo.deployment_data = Some(deploy_data);
                        if todo.pool_creation.is_some() {
                            debug!("{:#?}: Got all the data.\n{:#?}", id.clone(), todo);
                            break todo;
                        }
                    }
                    MessageTypes::Create(pool_creation) => {
                        debug!("Updater: Got pool creation data: {:?}", pool_creation);
                        todo.pool_creation = Some(pool_creation);
                        if todo.deployment_data.is_some() {
                            debug!("{:#?}: Got all the data.\n{:#?}", id.clone(), todo);
                            break todo;
                        }
                    }
                    _ => continue,
                }
            } else {
                debug!(
                    "{:#?} got some other message variant it could ignore.",
                    id.clone()
                );
                continue;
            }
        }
    }
}
