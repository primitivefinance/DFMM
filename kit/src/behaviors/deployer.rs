use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_bindings::bindings::weth::WETH;
use arbiter_engine::messager::To;
use bindings::{log_normal::LogNormal, geometric_mean::GeometricMean, dfmm::DFMM, constant_sum::ConstantSum};
use ethers::types::Address;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    pub weth: Address,
    pub dfmm: Address,
    pub g3m: Address,
    pub log_normal: Address,
    pub constant_sum: Address,
}

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let weth = WETH::deploy(client.clone(), ())?.send().await?;
        let dfmm = DFMM::deploy(client.clone(), weth.address())?.send().await?;
        let g3m = GeometricMean::deploy(client.clone(), dfmm.address())?.send().await?;
        let log_normal = LogNormal::deploy(client.clone(), dfmm.address())?.send().await?;
        let constant_sum = ConstantSum::deploy(client.clone(), dfmm.address())?.send().await?;

        let deployment_data = DeploymentData {
            weth: weth.address(),
            dfmm: dfmm.address(),
            g3m: g3m.address(),
            log_normal: log_normal.address(),
            constant_sum: constant_sum.address(),
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

    #[tokio::test]
    async fn token_admin_behavior_test() {
        let subscriber = FmtSubscriber::builder().finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();


        let mut world = World::new("test");
        let messager = world.messager.clone();

        let deployer = Deployer {};

        let agent = Agent::builder("token_admin_agent");
        world.add_agent(agent.with_behavior(deployer));

        world.run().await.expect("World failed to run");


        let mut stream = messager.stream().expect("Failed to get messager stream");
        if let Some(res) = stream.next().await {
            let token_res_data = &res.data;
            println!("{}", token_res_data);

            let data: String =
                serde_json::from_str(token_res_data).expect("Failed to deserialize message data");

            let parsed_data: DeploymentData =
                serde_json::from_str(&data).expect("Failed to deserialize token data");

            println!("{:?}", parsed_data);

            assert_eq!(Address::from_str("0xb00efcb70090a21d46660adf95a16ec69623f694").unwrap(), parsed_data.weth);
            assert_eq!(Address::from_str("0x27781b40bd019ccb1dcb0c809135db71222e9353").unwrap(), parsed_data.dfmm);
            assert_eq!(Address::from_str("0x6e0035324097bfc66442e2d3f37ef378fb3750b2").unwrap(), parsed_data.g3m);
            assert_eq!(Address::from_str("0x4be050270d209ef9f0c0435736c731767486279f").unwrap(), parsed_data.log_normal);
            assert_eq!(Address::from_str("0xaeb166f1355c6254d01a54317ef8d4d21bfcb4b0").unwrap(), parsed_data.constant_sum);

        }
    }



}
