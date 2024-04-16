include!("common.rs");

use std::str::FromStr;

use dfmm_kit::behaviors::deployer::DeploymentData;
use ethers::types::Address as eAddress;
use tracing::info;

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn run_deployer() {
    log();

    let mut world = World::new("test");
    let mut messager = world.messager.clone();

    spawn_deployer(&mut world);

    world.run().await.unwrap();

    if let Ok(message) = messager.get_next::<DeploymentData>().await {
        let data = message.data;
        info!("Saw message data: {:#?}", data);

        assert_eq!(
            eAddress::from_str("0xd26df90ce64eefc85fbfa01de29b8d8db161166e").unwrap(),
            data.weth
        );
        assert_eq!(
            eAddress::from_str("0x4dcb76f01b5624fecb5c7663892cb7977e9aaaa0").unwrap(),
            data.dfmm
        );
        assert_eq!(
            eAddress::from_str("0x8b0190d573c655293f1266ab9c8121f3a7ddbffc").unwrap(),
            data.geometric_mean
        );
        assert_eq!(
            eAddress::from_str("0xefe90bf7114239ba5bd78f8ecb25169ccb79d421").unwrap(),
            data.log_normal
        );
        assert_eq!(
            eAddress::from_str("0xe676315ff317009c870ef3371830db6e54eea748").unwrap(),
            data.constant_sum
        );
        assert_eq!(
            eAddress::from_str("0xc4867ade0303cce4261da6f267c30c6e27a8c223").unwrap(),
            data.n_token_geometric_mean
        );
    } else {
        panic!("No message received");
    }
}
