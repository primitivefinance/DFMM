use std::{boxed::Box, marker::PhantomData, sync::Arc};

use arbiter_engine::{
    machine::{
        Behavior, ControlFlow, CreateStateMachine, Engine, EventStream, Processor, State,
        StateMachine,
    },
    messager::{Message, Messager, To},
};
#[allow(unused)]
use arbiter_macros::{Behaviors, State};
use bindings::{arbiter_token::ArbiterToken, dfmm::DFMM, erc20::ERC20};
use ethers::utils::parse_ether;
pub use token::{MintRequest, Response, TokenAdminQuery};

use self::{
    create::Create,
    deploy::{Deploy, DeploymentData},
    pool::{constant_sum::ConstantSumPool, PoolCreation, PoolType},
    swap::Swap,
    token::TokenAdmin,
};
use super::*;

pub const MAX: eU256 = eU256::MAX;

pub mod allocate;
pub mod create;
pub mod deploy;
pub mod swap;
pub mod token;
pub mod update;

// TODO: This should never have generics in it (at least, at the moment until
// some crazy magic happens in `arbiter_engine`). The reason why is that
// `World::from_config<C>` will not work with this. It looks for a type
// parameter, and if you have generics it will need them to be fixed in place.
// However, this isn't so bad. Extending this enum is super simple and it does
// not break anything on top of it really (unlike having a `PoolType` as an
// enum). This is definitely not the most elegant thing though and I think this
// could be made much better with some TLC.
#[derive(Behaviors, Debug, Deserialize, Serialize)]
pub enum Behaviors {
    Deployer(Deploy),
    TokenAdmin(TokenAdmin<token::Config>),
    CreateConstantSum(Create<create::Config<ConstantSumPool>>),
    SwapConstantSum(Swap<swap::Config<ConstantSumPool>, swap::on_command::OnCommand, Message>),
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

// TODO: This is used by `Allocate` and `Swap` at the moment, so it was moved
// here since it was more central. However, there is likely a better way to
// combine all of these things.
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
