pub mod behaviors;
pub mod bindings;
pub mod pool;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::{Address as eAddress, U256 as eU256};
use serde::{Deserialize, Serialize};
use tracing::{error, info, trace, warn};

pub const WAD: eU256 = eU256([1_000_000_000_000_000_000, 0, 0, 0]);
pub const MAX: eU256 = eU256::MAX;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<eAddress>,
}

/// Used as an action to mint tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,

    /// The address to mint to.
    pub mint_to: eAddress,

    /// The amount to mint.
    pub mint_amount: u64,
}
