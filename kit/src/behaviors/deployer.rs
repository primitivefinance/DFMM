use arbiter_bindings::bindings::weth::WETH;
use bindings::{
    constant_sum::ConstantSum, dfmm::DFMM, geometric_mean::GeometricMean, log_normal::LogNormal,
};
use ethers::types::Address;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    pub n_token_geometric_mean: Address,
    pub weth: Address,
    pub dfmm: Address,
    pub geometric_mean: Address,
    pub log_normal: Address,
    pub constant_sum: Address,
    pub token_x: Address,
    pub token_y: Address,
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

        let log_normal = LogNormal::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!("LogNormal deployed at {:?}", log_normal.address());

        let constant_sum = ConstantSum::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;
        trace!("ConstantSum deployed at {:?}", constant_sum.address());

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

        let n_token_geometric_mean = GeometricMean::deploy(client.clone(), dfmm.address())?
            .send()
            .await?;

        let deployment_data = DeploymentData {
            n_token_geometric_mean: n_token_geometric_mean.address(),
            weth: weth.address(),
            dfmm: dfmm.address(),
            geometric_mean: geometric_mean.address(),
            log_normal: log_normal.address(),
            constant_sum: constant_sum.address(),
            token_x: token_x.address(),
            token_y: token_y.address(),
        };

        messager
            .send(To::All, serde_json::to_string(&deployment_data)?)
            .await?;
        Ok(None)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use arbiter_engine::{agent::Agent, world::World};
//     use ethers::types::Address;
//     use futures_util::StreamExt;
//     use tracing_subscriber::FmtSubscriber;

//     use crate::behaviors::deployer::{Deployer, DeploymentData};

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn deployer_behavior_test() {
//         let subscriber = FmtSubscriber::builder().finish();
//         tracing::subscriber::set_global_default(subscriber).unwrap();

//         let mut world = World::new("test");
//         let messager = world.messager.clone();

//         let agent = Agent::builder("token_admin_agent");
//         world.add_agent(agent.with_behavior(Deployer {}));

//         world.run().await.unwrap();
//         let mut stream = messager.stream().expect("Failed to get messager
// stream");

//         if let Some(res) = stream.next().await {
//             let token_res_data = &res.data;
//             println!("{}", token_res_data);

//             let data: String =
//                 serde_json::from_str(token_res_data).expect("Failed to
// deserialize message data");

//             let parsed_data: DeploymentData =
//                 serde_json::from_str(&data).expect("Failed to deserialize
// token data");

//             println!("{:?}", parsed_data);

//             assert_eq!(
//
// Address::from_str("0xb00efcb70090a21d46660adf95a16ec69623f694").unwrap(),
//                 parsed_data.weth
//             );
//             assert_eq!(
//
// Address::from_str("0x27781b40bd019ccb1dcb0c809135db71222e9353").unwrap(),
//                 parsed_data.dfmm
//             );
//             assert_eq!(
//
// Address::from_str("0x6e0035324097bfc66442e2d3f37ef378fb3750b2").unwrap(),
//                 parsed_data.geometric_mean
//             );
//             assert_eq!(
//
// Address::from_str("0x4be050270d209ef9f0c0435736c731767486279f").unwrap(),
//                 parsed_data.log_normal
//             );
//             assert_eq!(
//
// Address::from_str("0xaeb166f1355c6254d01a54317ef8d4d21bfcb4b0").unwrap(),
//                 parsed_data.constant_sum
//             );
//             assert_eq!(
//
// Address::from_str("0xa4bb88cbfc92d86ae00842dcfa5a1ac32b0714b3").unwrap(),
//                 parsed_data.n_token_geometric_mean
//             );
//         } else {
//             panic!("No message received");
//         }
//     }
// }
