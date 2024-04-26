use super::*;

pub fn mock_creator_behavior() -> Create<create::Config<ConstantSumPool>> {
    Create::<create::Config<ConstantSumPool>> {
        token_admin: TOKEN_ADMIN.to_owned(),
        data: create::Config {
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
