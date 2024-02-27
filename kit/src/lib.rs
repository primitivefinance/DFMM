pub mod behaviors;
pub mod bindings;
pub mod pool;
use anyhow::Result;
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::middleware::ArbiterMiddleware;
use bindings::{
    constant_sum::ConstantSum,
    constant_sum_solver::{ConstantSumParams, ConstantSumSolver},
    dfmm::DFMM,
    geometric_mean::GeometricMean,
    geometric_mean_solver::{GeometricMeanParams, GeometricMeanSolver},
    log_normal::LogNormal,
    log_normal_solver::{LogNormalParams, LogNormalSolver},
    weth::WETH,
};
use ethers::types::{Address as eAddress, U256 as eU256};
use pool::{constant_sum::ConstantSumPool, Pool, PoolType, Token};
use serde::{Deserialize, Serialize};
use tracing::trace;
