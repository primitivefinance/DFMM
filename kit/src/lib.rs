#![allow(unstable_features)]
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod behaviors;
pub mod bindings;
pub mod pool;

use std::fmt::Debug;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{machine::State, messager::Messager};
use arbiter_macros::State;
pub use behaviors::token::TokenData;
use ethers::types::{Address as eAddress, U256 as eU256};
pub use pool::{BaseConfig, Pool, PoolProcessing, PoolType};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
