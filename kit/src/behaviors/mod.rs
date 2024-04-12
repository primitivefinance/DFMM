use std::sync::Arc;

use arbiter_engine::{
    agent::Agent,
    machine::{Behavior, Configuration, ControlFlow, EventStream},
    messager::{Messager, To},
};
use arbiter_macros::Behaviors;
use bindings::arbiter_token::ArbiterToken;
use serde::{Deserialize, Serialize};

use self::{
    bindings::constant_sum_solver::ConstantSumParams,
    creator::Creator,
    deployer::Deployer,
    pool::{
        constant_sum::{ConstantSumConfig, ConstantSumPool},
        PoolType,
    },
    token_admin::TokenAdmin,
};
use super::*;

// pub mod allocate;
pub mod creator;
pub mod deployer;
pub mod token_admin;

#[derive(Debug, Deserialize, Serialize)]
pub enum Behaviors<P: PoolType> {
    Creator(Creator<creator::Config<P>>),
    Deployer(Deployer),
    TokenAdmin(TokenAdmin<token_admin::Config>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

pub(crate) fn default_admin_config() -> TokenAdmin<token_admin::Config> {
    let token1 = TokenData {
        name: "US Dollar Coin".to_owned(),
        symbol: "USDC".to_owned(),
        decimals: 18,
    };

    let token2 = TokenData {
        name: "ShibaInuObamaSonic Coin".to_owned(),
        symbol: "SIOS".to_owned(),
        decimals: 18,
    };
    let config = token_admin::Config {
        token_data: vec![token1, token2],
    };
    TokenAdmin::<token_admin::Config> { data: config }
}

pub(crate) fn default_creator_config() -> Creator<creator::Config<ConstantSumPool>> {
    Creator::<creator::Config<ConstantSumPool>> {
        data: creator::Config {
            params: ConstantSumParams {
                price: WAD,
                swap_fee: 0.into(),
                controller: eAddress::random(),
            },
            init_config: ConstantSumConfig {
                name: "Test Pool".to_string(),
                symbol: "TP".to_string(),
                reserve_x: WAD,
                reserve_y: WAD,
                token_x_name: "Token X".to_string(),
                token_y_name: "Token Y".to_string(),
                params: ConstantSumParams {
                    price: WAD,
                    swap_fee: 10000.into(),
                    controller: eAddress::zero(),
                },
            },
            token_list: vec![eAddress::zero(), eAddress::zero()],
        },
    }
}
