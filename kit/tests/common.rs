use std::{collections::VecDeque, marker::PhantomData};

use arbiter_engine::{agent::Agent, messager::Message, world::World};
use dfmm_kit::{
    behaviors::{
        creator::{self, Create},
        deploy::Deploy,
        swap::{self, Swap, SwapType},
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
use serde::{Deserialize, Serialize};
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

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

pub const PRICE: eU256 = WAD;
pub const RESERVE_X: eU256 = WAD;
pub const RESERVE_Y: eU256 = WAD;

pub const TARGET_TIMESTAMP: eU256 = WAD;

// Base Config
pub const POOL_NAME: &str = "TEST POOL";
pub const POOL_SYMBOL: &str = "TP";

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

pub fn spawn_constant_sum_updater(world: &mut World) {
    world.add_agent(
        Agent::builder(UPDATER)
            .with_behavior(mock_update_behavior())
            .with_behavior(mock_creator_behavior()),
    )
}

#[derive(Deserialize, Clone)]
pub struct SwapOnCommand {
    pub amount: eU256,
    pub input: InputToken,
}

impl SwapType<Message> for SwapOnCommand {
    fn compute_swap_amount(&self, event: Message) -> Option<(eU256, InputToken)> {
        debug!("Inside compute swap amount for `SwapOnCommand`");
        match serde_json::from_str::<ExecuteSwap>(&event.data) {
            Ok(_) => {
                debug!("Successfully deserialized message data for `SwapOnCommand`");
                Some((self.amount, self.input))
            }
            Err(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExecuteSwap;

fn mock_swap_behavior() -> Swap<swap::Config<ConstantSumPool>, SwapOnCommand, Message> {
    let config = swap::Config::<ConstantSumPool> {
        token_admin: TOKEN_ADMIN.to_owned(),
        _phantom: PhantomData,
    };
    Swap::<swap::Config<ConstantSumPool>, SwapOnCommand, Message> {
        data: config,
        swap_type: SwapOnCommand {
            amount: parse_ether(0.5).unwrap(),
            input: InputToken::TokenX,
        },
        _phantom: PhantomData,
    }
}

fn mock_token_admin_behavior() -> TokenAdmin<token::Config> {
    TokenAdmin::<token::Config> {
        data: mock_token_data(),
    }
}

fn mock_token_data() -> token::Config {
    let token_x = TokenData {
        name: TOKEN_X_NAME.to_owned(),
        symbol: TOKEN_X_SYMBOL.to_owned(),
        decimals: TOKEN_X_DECIMALS,
        address: None,
    };
    let token_y = TokenData {
        name: TOKEN_Y_NAME.to_owned(),
        symbol: TOKEN_Y_SYMBOL.to_owned(),
        decimals: TOKEN_Y_DECIMALS,
        address: None,
    };
    token::Config {
        token_data: vec![token_x, token_y],
    }
}
fn mock_update_behavior() -> Update<update::Config<ConstantSumPool>> {
    Update::<update::Config<ConstantSumPool>> {
        token_admin: TOKEN_ADMIN.to_owned(),
        data: update::Config {
            base_config: mock_base_config(),
            allocation_data: ConstantSumAllocationData {
                reserve_x: RESERVE_X,
                reserve_y: RESERVE_Y,
            },
            token_list: vec![TOKEN_X_NAME.to_owned(), TOKEN_Y_NAME.to_owned()],
            params: constant_sum_parameters_vec(),
        },
    }
}
fn mock_creator_behavior() -> Create<creator::Config<ConstantSumPool>> {
    Create::<creator::Config<ConstantSumPool>> {
        token_admin: TOKEN_ADMIN.to_owned(),
        data: creator::Config {
            params: ConstantSumParams {
                price: PRICE,
                swap_fee: parse_ether(0.003).unwrap(),
                controller: eAddress::zero(),
            },
            token_list: vec![TOKEN_X_NAME.to_owned(), TOKEN_Y_NAME.to_owned()],
            base_config: mock_base_config(),
            allocation_data: ConstantSumAllocationData {
                reserve_x: RESERVE_X,
                reserve_y: RESERVE_Y,
            },
        },
    }
}

fn mock_base_config() -> BaseConfig {
    BaseConfig {
        name: POOL_NAME.to_owned(),
        symbol: POOL_SYMBOL.to_owned(),
        swap_fee: parse_ether(0.003).unwrap(),
        controller_fee: 0.into(),
        controller: eAddress::zero(),
    }
}

pub fn constant_sum_parameters_vec() -> VecDeque<ConstantSumParams> {
    let prices: VecDeque<eU256> =
        vec![PRICE, parse_ether(2).unwrap(), parse_ether(3).unwrap()].into();
    let mut params = VecDeque::new();
    for price in prices {
        let parameter = ConstantSumParams {
            price,
            swap_fee: parse_ether(0.003).unwrap(),
            controller: eAddress::zero(),
        };
        params.push_back(parameter);
    }
    params
}

fn _constant_sum_parameters() -> ConstantSumParams {
    ConstantSumParams {
        price: parse_ether(1).unwrap(),
        swap_fee: parse_ether(0.003).unwrap(),
        controller: eAddress::zero(),
    }
}
