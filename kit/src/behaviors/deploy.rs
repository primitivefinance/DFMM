use arbiter_bindings::bindings::weth::WETH;
use bindings::{
    constant_sum::ConstantSum, constant_sum_solver::ConstantSumSolver,
    geometric_mean::GeometricMean, geometric_mean_solver::GeometricMeanSolver,
    log_normal::LogNormal, log_normal_solver::LogNormalSolver,
};

use super::*;
use crate::bindings::{
    n_token_geometric_mean::NTokenGeometricMean,
    n_token_geometric_mean_solver::NTokenGeometricMeanSolver,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Deploy {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    pub weth: eAddress,
    pub dfmm: eAddress,
    pub geometric_mean: eAddress,
    pub geometric_mean_solver: eAddress,
    pub n_token_geometric_mean: eAddress,
    pub n_token_geometric_mean_solver: eAddress,
    pub log_normal: eAddress,
    pub log_normal_solver: eAddress,
    pub constant_sum: eAddress,
    pub constant_sum_solver: eAddress,
}

#[async_trait::async_trait]
impl Behavior<()> for Deploy {
    type Processor = ();
    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Self::Processor> {
        let weth = WETH::deploy(client.clone(), ())?.send().await?;
        let dfmm = DFMM::deploy(client.clone(), weth.address())?.send().await?;
        let geometric_mean = GeometricMean::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let geometric_mean_solver =
            GeometricMeanSolver::deploy(client.clone(), geometric_mean.address())?
                .send()
                .await?;
        let log_normal = LogNormal::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let log_normal_solver = LogNormalSolver::deploy(client.clone(), log_normal.address())?
            .send()
            .await?;
        let constant_sum = ConstantSum::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let constant_sum_solver =
            ConstantSumSolver::deploy(client.clone(), constant_sum.address())?
                .send()
                .await?;
        let n_token_geometric_mean = NTokenGeometricMean::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let n_token_geometric_mean_solver =
            NTokenGeometricMeanSolver::deploy(client.clone(), n_token_geometric_mean.address())?
                .send()
                .await?;
        let deployment_data = DeploymentData {
            weth: weth.address(),
            dfmm: dfmm.address(),
            geometric_mean: geometric_mean.address(),
            geometric_mean_solver: geometric_mean_solver.address(),
            n_token_geometric_mean: n_token_geometric_mean.address(),
            n_token_geometric_mean_solver: n_token_geometric_mean_solver.address(),
            log_normal: log_normal.address(),
            log_normal_solver: log_normal_solver.address(),
            constant_sum: constant_sum.address(),
            constant_sum_solver: constant_sum_solver.address(),
        };
        debug!("Deployments completed: {:#?}", deployment_data);
        messager.send(To::All, deployment_data).await?;
        Ok(())
    }
}
