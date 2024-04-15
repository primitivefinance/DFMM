use arbiter_bindings::bindings::weth::WETH;
use bindings::{
    constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver, dfmm::DFMM,
    geometric_mean::GeometricMean, geometric_mean_solver::GeometricMeanSolver,
    log_normal::LogNormal, log_normal_solver::LogNormalSolver,
};
use ethers::types::Address;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    pub weth: Address,
    pub dfmm: Address,
    pub geometric_mean: Address,
    pub geometric_mean_solver: Address,
    pub n_token_geometric_mean: Address,
    pub log_normal: Address,
    pub log_normal_solver: Address,
    pub constant_sum: Address,
    pub constant_sum_solver: Address,
}

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    type Processor = ();
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<()>)>> {
        let weth = WETH::deploy(client.clone(), ())?.send().await?;
        trace!("WETH deployed at {:?}", weth.address());

        let dfmm = DFMM::deploy(client.clone(), weth.address())?.send().await?;
        trace!("DFMM deployed at {:?}", dfmm.address());

        let geometric_mean = GeometricMean::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!("GeometricMean deployed at {:?}", geometric_mean.address());

        let geometric_mean_solver = GeometricMeanSolver::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!(
            "GeometricMeanSolver deployed at {:?}",
            geometric_mean.address()
        );

        let log_normal = LogNormal::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!("LogNormal deployed at {:?}", log_normal.address());

        let log_normal_solver = LogNormalSolver::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!(
            "LogNormalSolver deployed at {:?}",
            log_normal_solver.address()
        );

        let constant_sum = ConstantSum::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!("ConstantSum deployed at {:?}", constant_sum.address());

        let constant_sum_solver = ConstantSumSolver::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!("ConstantSumSolver deployed at {:?}", constant_sum.address());

        let n_token_geometric_mean = GeometricMean::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;

        let deployment_data = DeploymentData {
            n_token_geometric_mean: n_token_geometric_mean.address(),
            weth: weth.address(),
            dfmm: dfmm.address(),
            geometric_mean: geometric_mean.address(),
            geometric_mean_solver: geometric_mean_solver.address(),
            log_normal: log_normal.address(),
            log_normal_solver: log_normal_solver.address(),
            constant_sum: constant_sum.address(),
            constant_sum_solver: constant_sum_solver.address(),
        };

        debug!("Deployments completed: {:#?}", deployment_data);

        messager.send(To::All, deployment_data).await?;
        Ok(None)
    }
}
