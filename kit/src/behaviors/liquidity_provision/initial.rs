use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Initial {
    pub input_x: Option<eU256>,
    pub input_y: Option<eU256>,
    pub price: eU256,
}

impl AllocationType<()> for Initial {
    fn change_allocation_amount(self, event: ()) -> Option<Allocation> {
        Some(Allocation {
            amount_x: self.input_x.map(I256::from_raw),
            amount_y: self.input_y.map(I256::from_raw),
            price: Some(self.price),
        })
    }
}
