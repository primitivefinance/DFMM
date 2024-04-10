use anyhow::Ok;
use bindings::{log_normal::LogNormal, log_normal_solver::LogNormalSolver};
use ethers::abi::Address;
use serde::{Serialize, Deserialize};

use super::*;
pub struct LogNormalPool {
    pub strategy_contract: LogNormal<ArbiterMiddleware>,
    pub solver_contract: LogNormalSolver<ArbiterMiddleware>,
    pub parameters: LogNormalUpdateParameters,
}

pub enum LogNormalUpdateParameters {
    FeeUpdate(eU256),
    ControllerUpdate(Address),
    StrikeUpdate(eU256, eU256),
    SigmaUpdate(eU256, eU256),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogNormalParams {
    pub mean: eU256,
    pub width: eU256,
}

impl PoolConfigurer for LogNormalParams {
    type PoolParameters = Self;
    type InitialAllocationData = Bytes;
}

pub enum LogNormalAllocationData {
    GivenX(eU256),
    GivenY(eU256),
}

impl PoolType for LogNormalPool {
    type Parameters = LogNormalParams;
    type StrategyContract = LogNormal<ArbiterMiddleware>;
    type SolverContract = LogNormalSolver<ArbiterMiddleware>;
    type AllocationData = LogNormalAllocationData;

    async fn swap_data(
        &self,
        pool_id: eU256,
        input_token: InputToken,
        amount_in: eU256,
    ) -> Result<Bytes> {
        let (valid, _, _, data) = match input_token {
            InputToken::TokenX => {
                self.solver_contract
                    .simulate_swap(pool_id, true, amount_in)
                    .call()
                    .await?
            }
            InputToken::TokenY => {
                self.solver_contract
                    .simulate_swap(pool_id, false, amount_in)
                    .call()
                    .await?
            }
        };
        if valid {
            Ok(data)
        } else {
            anyhow::bail!("swap was invalid!")
        }
    }

    async fn update_data(&self, new_data: Self::Parameters) -> Result<Bytes> {
        // let bytes = match new_data {
        //     LogNormalUpdateParameters::FeeUpdate(fee) => {
        //         self.solver_contract.prepare_fee_update(fee).call().await?
        //     }
        //     LogNormalUpdateParameters::StrikeUpdate(strike, expiry) => {
        //         self.solver_contract
        //             .prepare_mean_update(strike, expiry)
        //             .call()
        //             .await?
        //     }
        //     LogNormalUpdateParameters::SigmaUpdate(sigma, expiry) => {
        //         self.solver_contract
        //             .prepare_width_update(sigma, expiry)
        //             .call()
        //             .await?
        //     }
        //     LogNormalUpdateParameters::ControllerUpdate(controller) => {
        //         self.solver_contract
        //             .prepare_controller_update(controller)
        //             .call()
        //             .await?
        //     }
        // };
        // Ok(bytes)
        todo!()
    }

    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        let data = match allocation_data {
            LogNormalAllocationData::GivenX(amount_x) => {
                self.solver_contract
                    .prepare_allocation_deltas_given_delta_x(pool_id, amount_x)
                    .call()
                    .await?
            }
            LogNormalAllocationData::GivenY(amount_y) => {
                self.solver_contract
                    .prepare_allocation_deltas_given_delta_y(pool_id, amount_y)
                    .call()
                    .await?
            }
        };
        Ok(data)
    }
}
