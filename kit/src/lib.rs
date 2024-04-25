#![allow(unstable_features)]
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod behaviors;
pub mod bindings;
pub mod pool;

use std::fmt::Debug;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
pub use behaviors::token::TokenData;
use ethers::types::{Address as eAddress, I256 as eI256, U256 as eU256};
pub use pool::{BaseConfig, Pool, PoolType};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
