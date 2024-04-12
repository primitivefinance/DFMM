use arbiter_engine::{agent::Agent, world::World};
use dfmm_kit::{
    behaviors::{
        creator::{self, Creator},
        deployer::Deployer,
        token_admin::{self, TokenAdmin},
    },
    pool::{
        constant_sum::{ConstantSumAllocationData, ConstantSumParams, ConstantSumPool},
        BaseConfig,
    },
    TokenData,
};
use ethers::types::U256 as eU256;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub const WAD: eU256 = eU256([1_000_000_000_000_000_000, 0, 0, 0]);

pub fn log() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub fn spawn_deployer(world: &mut World) {
    let behavior = Deployer {};
    world.add_agent(Agent::builder("deployer").with_behavior(behavior));
}

pub fn spawn_token_admin(world: &mut World) {
    let token1 = TokenData {
        name: "US Dollar Coin".to_owned(),
        symbol: "USDC".to_owned(),
        decimals: 18,
        address: None,
    };

    let token2 = TokenData {
        name: "ShibaInuObamaSonic Coin".to_owned(),
        symbol: "SIOS".to_owned(),
        decimals: 18,
        address: None,
    };
    let config = token_admin::Config {
        token_data: vec![token1, token2],
    };
    let behavior = TokenAdmin::<token_admin::Config> { data: config };
    world.add_agent(Agent::builder("token_admin").with_behavior(behavior));
}

pub fn spawn_creator(world: &mut World) {
    let behavior = Creator::<creator::Config<ConstantSumPool>> {
        data: creator::Config {
            params: ConstantSumParams { price: WAD },
            token_list: vec!["Token X".to_string(), "Token Y".to_string()],
            base_config: BaseConfig {
                name: "Test Pool".to_string(),
                symbol: "TP".to_string(),
                swap_fee: 10000.into(),
                controller_fee: 0.into(),
            },
            allocation_data: ConstantSumAllocationData {
                reserve_x: WAD,
                reserve_y: WAD,
            },
        },
    };
    world.add_agent(Agent::builder("creator").with_behavior(behavior));
}
