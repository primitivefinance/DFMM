use super::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OnCommand {
    pub amount: eU256,
    pub input: InputToken,
}

impl SwapType<Message> for OnCommand {
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
