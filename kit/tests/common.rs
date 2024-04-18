use arbiter_engine::{agent::Agent, world::World};
use dfmm_kit::{
    behaviors::{
        creator::{self, Create},
        deploy::Deploy,
        token::{self, TokenAdmin},
        update::{self, Update},
    },
    bindings::constant_sum_solver::ConstantSumParams,
    pool::{
        constant_sum::{ConstantSumAllocationData, ConstantSumPool},
        BaseConfig,
    },
    TokenData,
};
use ethers::types::{Address as eAddress, U256 as eU256};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub const WAD: eU256 = eU256([1_000_000_000_000_000_000, 0, 0, 0]);
pub const DEPLOYER: &str = "deployer";
pub const TOKEN_ADMIN: &str = "token_admin";
pub const CREATOR: &str = "creator";
pub const UPDATER: &str = "updater";

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
    let data = token::Config {
        token_data: vec![token_x, token_y],
    };
    world
        .add_agent(Agent::builder(TOKEN_ADMIN).with_behavior(TokenAdmin::<token::Config> { data }));
}

pub fn spawn_constant_sum_creator(world: &mut World) {
    world.add_agent(Agent::builder(CREATOR).with_behavior(creator()));
}

pub fn spawn_constant_sum_updater(world: &mut World) {
    let params = constant_sum_parameters();
    world.add_agent(Agent::builder(UPDATER).with_behavior(Update::<
        update::Config<ConstantSumPool>,
    > {
        token_admin: TOKEN_ADMIN.to_owned(),
        data: update::Config {
            base_config: BaseConfig {
                name: "Test Pool".to_string(),
                symbol: "TP".to_string(),
                swap_fee: ethers::utils::parse_ether(0.003).unwrap(),
                controller_fee: 0.into(),
            },
            allocation_data: ConstantSumAllocationData {
                reserve_x: RESERVE_X,
                reserve_y: RESERVE_Y,
            },
            token_list: vec![TOKEN_X_NAME.to_owned(), TOKEN_Y_NAME.to_owned()],
            params,
        },
    }))
}

pub fn constant_sum_parameters() -> Vec<ConstantSumParams> {
    let prices = vec![
        PRICE,
        ethers::utils::parse_ether(2).unwrap(),
        ethers::utils::parse_ether(3).unwrap(),
    ];
    let mut params = vec![];
    for price in prices {
        let parameter = ConstantSumParams {
            price,
            swap_fee: ethers::utils::parse_ether(0.003).unwrap(),
            controller: eAddress::zero(),
        };
        params.push(parameter);
    }
    params
}

fn creator() -> Create<creator::Config<ConstantSumPool>> {
    Create::<creator::Config<ConstantSumPool>> {
        token_admin: TOKEN_ADMIN.to_owned(),
        data: creator::Config {
            params: ConstantSumParams {
                price: PRICE,
                swap_fee: ethers::utils::parse_ether(0.003).unwrap(),
                controller: eAddress::zero(),
            },
            token_list: vec![TOKEN_X_NAME.to_owned(), TOKEN_Y_NAME.to_owned()],
            base_config: BaseConfig {
                name: "Test Pool".to_string(),
                symbol: "TP".to_string(),
                swap_fee: ethers::utils::parse_ether(0.003).unwrap(),
                controller_fee: 0.into(),
            },
            allocation_data: ConstantSumAllocationData {
                reserve_x: RESERVE_X,
                reserve_y: RESERVE_Y,
            },
        },
    }
}
