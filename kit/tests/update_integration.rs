use std::time::Duration;

use arbiter_engine::messager::To;
use dfmm_kit::behaviors::update;
use futures_util::StreamExt;
use tracing::{debug, info};
use tracing_subscriber::registry::Data;
include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn run_updater_constant_sum() {
    log(Level::DEBUG);

    let mut world = World::new("test");
    let messager = world.messager.for_agent("test");

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);
    spawn_constant_sum_creator(&mut world);
    spawn_constant_sum_updater(&mut world);

    let task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Sleep because the world needs to give all of the agents time to build their
        // receivers. TODO: This is a bit of a hack and we could honestly make
        // the `World::run` better to handle this, but this works for now.
        tokio::time::sleep(Duration::from_millis(2000)).await;
        messager
            .send(
                To::Agent(CREATOR.to_owned()),
                update::UpdatoorQuerry::UpdateMeDaddy,
            )
            .await
            .unwrap();
        let mut stream = messager.stream().unwrap();
        // let mut count = 0;
        while let Some(message) = stream.next().await {
            info!("Saw message: {:#?}", message);

            // BUG IS HERE: for some reason we are never entering this loop
            match serde_json::from_str::<ConstantSumParams>(&message.data) {
                Ok(data) => {
                    info!("Saw data: {:#?}", data);
                    let mock_data = constant_sum_parameters();
                    assert_eq!(data, mock_data[0]);
                    info!("Asserts passed!");
                    break;
                }
                Err(_) => {
                    continue;
                }
            }
            // count += 1;
        }
    });

    // Setup a timeout for the test to ensure it does not run indefinitely.
    let timeout_duration = Duration::from_secs(5); // Adjust the timeout as needed.

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
