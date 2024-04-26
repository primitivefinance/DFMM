use super::*;

pub fn mock_update_behavior() -> Update<update::Config, ConstantSumPool> {
    Update::<update::Config, ConstantSumPool> {
        token_admin: TOKEN_ADMIN.to_owned(),
        params: mock_constant_sum_parameters(),
        data: update::Config,
    }
}

pub fn mock_constant_sum_parameters() -> VecDeque<ConstantSumParams> {
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
