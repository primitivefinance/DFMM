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
    ControllerUpdate(Address),
}

pub enum LogNormalAllocationData {
    AllocateGivenX(eU256),
    AllocateGivenY(eU256),
    DeallocateGivenX(eU256),
    DeallocateGivenY(eU256),
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
                    .prepare_mean_update(strike, expiry)
                    .call()
                    .await?
            }
            LogNormalUpdateParameters::SigmaUpdate(sigma, expiry) => {
                self.solver_contract
                    .prepare_width_update(sigma, expiry)
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

    async fn allocation_data(
        &self,
        pool_id: eU256,
        allocation_data: Self::AllocationData,
    ) -> Result<Bytes> {
        let (next_x, next_y, next_l) = match allocation_data {
            LogNormalAllocationData::AllocateGivenX(amount_x) => {
                self.solver_contract
                    .allocate_given_x(pool_id, amount_x)
                    .call()
                    .await?
            }
            LogNormalAllocationData::AllocateGivenY(amount_y) => {
                self.solver_contract
                    .allocate_given_y(pool_id, amount_y)
                    .call()
                    .await?
            }
            LogNormalAllocationData::DeallocateGivenX(amount_x) => {
                self.solver_contract
                    .deallocate_given_x(pool_id, amount_x)
                    .call()
                    .await?
            }
            LogNormalAllocationData::DeallocateGivenY(amount_y) => {
                self.solver_contract
                    .deallocate_given_y(pool_id, amount_y)
                    .call()
                    .await?
            }
        };
        let token_1 = ethers::abi::Token::Uint(next_x);
        let token_2 = ethers::abi::Token::Uint(next_y);
        let token_3 = ethers::abi::Token::Uint(next_l);
        let bytes: Bytes = ethers::abi::encode(&[token_1, token_2, token_3])
            .iter()
            .collect();
        Ok(bytes)
    }
}
