use super::*;

pub fn mock_swap_behavior() -> Swap<swap::Config<ConstantSumPool>, OnCommand, Message> {
    let config = swap::Config::<ConstantSumPool> {
        token_admin: TOKEN_ADMIN.to_owned(),
        _phantom: PhantomData,
    };
    Swap::<swap::Config<ConstantSumPool>, OnCommand, Message> {
        data: config,
        swap_type: OnCommand {
            amount: parse_ether(0.5).unwrap(),
            input: InputToken::TokenX,
        },
        _phantom: PhantomData,
    }
}
