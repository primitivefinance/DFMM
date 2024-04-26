use super::*;

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

pub fn mock_swap_behavior() -> Swap<swap::Config<ConstantSumPool>, SwapOnCommand, Message> {
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
