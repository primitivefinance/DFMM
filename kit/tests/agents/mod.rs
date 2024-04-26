use std::{collections::VecDeque, marker::PhantomData, str::FromStr, time::Duration};

use arbiter_engine::{agent::Agent, messager::Message, world::World};
use dfmm_kit::{
    behaviors::{
        create::{self, Create},
        deploy::Deploy,
        swap::{
            self,
            on_command::{ExecuteSwap, OnCommand},
            Swap,
        },
        token::{self, TokenAdmin},
        update::{self, Update},
    },
    bindings::constant_sum_solver::ConstantSumParams,
    pool::{
        constant_sum::{ConstantSumAllocationData, ConstantSumPool},
        BaseConfig, InputToken,
    },
    TokenData,
};
use ethers::{
    types::{Address as eAddress, U256 as eU256},
    utils::parse_ether,
};
use futures_util::StreamExt;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

mod creator_integration;
mod deploy_integration;
mod mock_agents;
mod swap_integration;
mod update_integration;

use mock_agents::{
    creator_agent::mock_creator_behavior,
    swap_agent::mock_swap_behavior,
    token_agent::mock_token_admin_behavior,
    update_agent::{mock_constant_sum_parameters, mock_update_behavior},
};

pub const WAD: eU256 = eU256([1_000_000_000_000_000_000, 0, 0, 0]);

// Agent IDs
pub const DEPLOYER: &str = "deployer";
pub const TOKEN_ADMIN: &str = "token_admin";
pub const CREATOR: &str = "creator";
pub const UPDATER: &str = "updater";
pub const SWAPPER: &str = "swapper";

// Token Data
pub const TOKEN_X_NAME: &str = "Token X";
pub const TOKEN_X_SYMBOL: &str = "TKNX";
pub const TOKEN_X_DECIMALS: u8 = 18;
pub const TOKEN_Y_NAME: &str = "Token Y";
pub const TOKEN_Y_SYMBOL: &str = "TKNY";
pub const TOKEN_Y_DECIMALS: u8 = 18;

// Pool Parameters
pub const POOL_NAME: &str = "TEST POOL";
pub const POOL_SYMBOL: &str = "TP";
pub const PRICE: eU256 = WAD;
pub const RESERVE_X: eU256 = WAD;
pub const RESERVE_Y: eU256 = WAD;

// Update Parameters
pub const TARGET_TIMESTAMP: eU256 = WAD;

pub fn log(level: Level) {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(level)
            .pretty()
            .finish(),
    )
    .unwrap();
}

pub fn spawn_deployer(world: &mut World) {
    world.add_agent(Agent::builder(DEPLOYER).with_behavior(Deploy {}));
}

pub fn spawn_token_admin(world: &mut World) {
    world.add_agent(Agent::builder(TOKEN_ADMIN).with_behavior(mock_token_admin_behavior()));
}

pub fn spawn_constant_sum_creator(world: &mut World) {
    world.add_agent(Agent::builder(CREATOR).with_behavior(mock_creator_behavior()));
}

pub fn spawn_constant_sum_swapper(world: &mut World) {
    world.add_agent(Agent::builder(SWAPPER).with_behavior(mock_swap_behavior()))
}

pub fn spawn_constant_sum_creator_and_updater(world: &mut World) {
    world.add_agent(
        Agent::builder(UPDATER)
            .with_behavior(mock_update_behavior())
            .with_behavior(mock_creator_behavior()),
    )
}
