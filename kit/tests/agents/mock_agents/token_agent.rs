use super::*;

pub fn mock_token_admin_behavior() -> TokenAdmin<token::Config> {
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
