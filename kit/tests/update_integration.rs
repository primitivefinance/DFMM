use std::time::Duration;

use arbiter_engine::messager::To;
use dfmm_kit::behaviors::MessageTypes;
use futures_util::StreamExt;
use tracing::{info, warn};
include!("common.rs");

// TODO: It is a bit odd in this test that when we see the `ConstantSumParams`
// we see that the controller is the 0 address. This is not actually correct
// though as it should be: "0x62007c61825f5052695f5de39baab5c23299876b".
// This is just due to the redundancy we have with the `ConstantSumParams` in
// and `BaseConfig` that we are currently using.

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn run_updater_constant_sum() {
    log(Level::DEBUG);

    let mut world = World::new("test");
    let mut messager = world.messager.for_agent("test");

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);
    spawn_constant_sum_updater(&mut world);

    // NOTE: This test should be improved so that we check the contract state was
    // actually changed and updated correctly
    let task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Sleep because the world needs to give all of the agents time to build their
        // receivers. TODO: This is a bit of a hack and we could honestly make
        // the `World::run` better to handle this, but this works for now.
        tokio::time::sleep(Duration::from_millis(2000)).await;
        for _ in 0..3 {
            messager
                .send(
                    To::Agent(UPDATER.to_owned()),
                    update::UpdateRequest::ApplyUpdate,
                )
                .await
                .unwrap();
        }
        let mut count = 0;
        let mut stream = messager.stream().unwrap();
        while let Some(message) = stream.next().await {
            match serde_json::from_str::<MessageTypes<ConstantSumPool>>(&message.data) {
                Ok(data) => {
                    info!("deserialized data: {:#?}", data);
                    match data {
                        MessageTypes::Deploy(_) => continue,
                        MessageTypes::Create(_) => continue,
                        MessageTypes::TokenAdmin(_) => continue,
                        MessageTypes::Update(params) => {
                            info!("TEST SEES updated params: {:#?}", params);
                            let mock_data = constant_sum_parameters_vec();
                            assert_eq!(params, mock_data[count]);
                            if count >= 2 {
                                break;
                            } else {
                                count += 1;
                            }
                        }
                    }
                }
                Err(_) => {
                    warn!(
                        "Failed to parse message data into ConstantSumParams, instead got: {:#?}",
                        message.data
                    );
                    continue;
                }
            }
        }
    });

    // Setup a timeout for the test to ensure it does not run indefinitely.
    let timeout_duration = Duration::from_secs(10); // Adjust the timeout as needed.

    tokio::select! {
        _ = world.run() => {
            panic!("World run unexpectedly completed");
        },
        result = task => {
            match result {
                Ok(_) => println!("Task completed successfully and test should pass."),
                Err(e) => panic!("Task encountered an error: {:?}", e),
            }
        },
        _ = tokio::time::sleep(timeout_duration) => {
            panic!("Test timed out");
        }
    }
}
