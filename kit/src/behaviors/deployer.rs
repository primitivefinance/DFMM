use arbiter_engine::messager::To;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    pub weth: eAddress,
    pub dfmm: eAddress,
    pub geometric_mean: eAddress,
    pub geometric_mean_solver: eAddress,
    pub log_normal: eAddress,
    pub log_normal_solver: eAddress,
    pub constant_sum: eAddress,
    pub constant_sum_solver: eAddress,
    pub token_x: eAddress,
    pub token_y: eAddress,
}

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let weth = WETH::deploy(client.clone(), ())?.send().await?;
        trace!("WETH deployed at {:?}", weth.address());

        let dfmm = DFMM::deploy(client.clone(), weth.address())?.send().await?;
        trace!("DFMM deployed at {:?}", dfmm.address());

        let geometric_mean = GeometricMean::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let geometric_mean_solver =
            GeometricMeanSolver::deploy(client.clone(), geometric_mean.address())?
                .send()
                .await?;
        trace!(
            "GeometricMean deployed at {:?} with solver at {:?}",
            geometric_mean.address(),
            geometric_mean_solver.address()
        );

        let log_normal = LogNormal::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let log_normal_solver = LogNormalSolver::deploy(client.clone(), log_normal.address())?
            .send()
            .await?;
        trace!(
            "LogNormal deployed at {:?} with solver at {:?}",
            log_normal.address(),
            log_normal_solver.address()
        );

        let constant_sum = ConstantSum::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        let constant_sum_solver =
            ConstantSumSolver::deploy(client.clone(), constant_sum.address())?
                .send()
                .await?;
        trace!(
            "ConstantSum deployed at {:?} with solver at {:?}",
            constant_sum.address(),
            constant_sum_solver.address()
        );

        let token_x = ArbiterToken::deploy(
            client.clone(),
            ("Token X".to_owned(), "ARBX".to_owned(), 18u8),
        )?
        .send()
        .await?;
        let token_y = ArbiterToken::deploy(
            client.clone(),
            ("Token Y".to_owned(), "ARBY".to_owned(), 18u8),
        )?
        .send()
        .await?;

        trace!(
            "Tokens deployed at {:?} and {:?}",
            token_x.address(),
            token_y.address()
        );

        let deployment_data = DeploymentData {
            weth: weth.address(),
            dfmm: dfmm.address(),
            geometric_mean: geometric_mean.address(),
            geometric_mean_solver: geometric_mean_solver.address(),
            log_normal: log_normal.address(),
            log_normal_solver: log_normal_solver.address(),
            constant_sum: constant_sum.address(),
            constant_sum_solver: constant_sum_solver.address(),
            token_x: token_x.address(),
            token_y: token_y.address(),
        };

        messager
            .send(To::All, serde_json::to_string(&deployment_data)?)
            .await?;
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use arbiter_engine::{agent::Agent, world::World};
    use ethers::types::Address;
    use futures_util::StreamExt;
    use tracing_subscriber::FmtSubscriber;

    use crate::behaviors::deployer::{Deployer, DeploymentData};

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn deployer_behavior_test() {
        let subscriber = FmtSubscriber::builder().finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let mut world = World::new("test");
        let messager = world.messager.clone();

        let agent = Agent::builder("token_admin_agent");
        world.add_agent(agent.with_behavior(Deployer {}));

        world.run().await.unwrap();
        let mut stream = messager.stream().expect("Failed to get messager stream");

        if let Some(res) = stream.next().await {
            let token_res_data = &res.data;
            println!("{}", token_res_data);

            let data: String =
                serde_json::from_str(token_res_data).expect("Failed to deserialize message data");

            let parsed_data: DeploymentData =
                serde_json::from_str(&data).expect("Failed to deserialize token data");

            println!("{:?}", parsed_data);

            assert_eq!(
                Address::from_str("0xb00efcb70090a21d46660adf95a16ec69623f694").unwrap(),
                parsed_data.weth
            );
            assert_eq!(
                Address::from_str("0x27781b40bd019ccb1dcb0c809135db71222e9353").unwrap(),
                parsed_data.dfmm
            );
            assert_eq!(
                Address::from_str("0x6e0035324097bfc66442e2d3f37ef378fb3750b2").unwrap(),
                parsed_data.geometric_mean
            );
            assert_eq!(
                Address::from_str("0x4be050270d209ef9f0c0435736c731767486279f").unwrap(),
                parsed_data.log_normal
            );
            assert_eq!(
                Address::from_str("0xaeb166f1355c6254d01a54317ef8d4d21bfcb4b0").unwrap(),
                parsed_data.constant_sum
            );
        } else {
            panic!("No message received");
        }
    }
}
