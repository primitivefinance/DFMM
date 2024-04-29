pub mod behaviors;
pub mod bindings;
pub mod pool;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use ethers::types::{Address as eAddress, U256 as eU256};
use tracing::{error, trace, warn};
