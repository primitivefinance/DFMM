use anyhow::Ok;
use bindings::{log_normal::LogNormal, log_normal_solver::LogNormalSolver};
use ethers::abi::Address;

use super::*;
pub struct LogNormalPool {
    pub strategy_contract: LogNormal<ArbiterMiddleware>,
    pub solver_contract: LogNormalSolver<ArbiterMiddleware>,
    pub parameters: LogNormalUpdateParameters,
}

pub enum LogNormalUpdateParameters {
    FeeUpdate(eU256),
    StrikeUpdate(eU256, eU256),
    SigmaUpdate(eU256, eU256),
    TauUpdate(eU256, eU256),
    ControllerUpdate(Address),
}

pub struct LogNormalAllocationData {
    pub allocate: bool,
    pub amount: eU256,
}

impl PoolType for LogNormalPool {
    type Parameters = LogNormalUpdateParameters;
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
        let bytes = match new_data {
            LogNormalUpdateParameters::FeeUpdate(fee) => {
                self.solver_contract.prepare_fee_update(fee).call().await?
            }
            LogNormalUpdateParameters::StrikeUpdate(strike, expiry) => {
                self.solver_contract
                    .prepare_strike_update(strike, expiry)
                    .call()
                    .await?
            }
            LogNormalUpdateParameters::SigmaUpdate(sigma, expiry) => {
                self.solver_contract
                    .prepare_sigma_update(sigma, expiry)
                    .call()
                    .await?
            }
            LogNormalUpdateParameters::TauUpdate(tau, expiry) => {
                self.solver_contract
                    .prepare_tau_update(tau, expiry)
                    .call()
                    .await?
            }
            LogNormalUpdateParameters::ControllerUpdate(controller) => {
                self.solver_contract
                    .prepare_controller_update(controller)
                    .call()
                    .await?
            }
        };
        Ok(bytes)
    }

    async fn change_allocation_data(
        &self,
        pool_id: eU256,
        allocation_date: Self::AllocationData,
    ) -> Result<Bytes> {
        todo!()
    }
}
