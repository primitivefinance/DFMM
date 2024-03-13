use self::bindings::idfmm::InitParams;
use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Initialize {
    pub rx: Option<eU256>,
    pub ry: Option<eU256>,
    pub pool_parameters: PoolParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PoolParameters {
    ConstantSum(ConstantSumParams),
}

#[async_trait::async_trait]
impl Behavior<()> for Initialize {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        // await the deployment of DFMM contracts
        while let Ok(message) = messager.get_next().await {
            let deployment_data = match serde_json::from_str::<DeploymentData>(&message.data) {
                Ok(data) => data,
                Err(_) => continue,
            };
            match &self.pool_parameters {
                PoolParameters::ConstantSum(params) => {
                    let solver =
                        ConstantSumSolver::new(deployment_data.constant_sum, client.clone());
                    let data = solver
                        .get_initial_pool_data(self.rx.unwrap(), self.ry.unwrap(), params.clone())
                        .call()
                        .await?;
                    let dfmm = DFMM::new(deployment_data.dfmm, client);
                    dfmm.init(InitParams {
                        strategy: deployment_data.constant_sum,
                        token_x: deployment_data.token_x,
                        token_y: deployment_data.token_y,
                        data,
                    })
                    .send()
                    .await?
                    .await?;
                    break;
                }
            }
        }
        Ok(None)
    }
}
