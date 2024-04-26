use std::{str::FromStr, time::Duration};

use arbiter_core::{events::stream_event, middleware::ArbiterMiddleware};
use arbiter_engine::messager::To;
use dfmm_kit::bindings::dfmm::DFMM;
use futures_util::StreamExt;
use tracing::{info, warn};
include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn run_swapper_constant_sum() {
    log(Level::TRACE);

    let mut world = World::new("test");
    let mut messager = world.messager.for_agent("test");
    let client = ArbiterMiddleware::new(world.environment.as_ref().unwrap(), None).unwrap();

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);
    spawn_constant_sum_creator(&mut world);
    spawn_constant_sum_swapper(&mut world);

    let task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Sleep because the world needs to give all of the agents time to build their
        // receivers. TODO: This is a bit of a hack and we could honestly make
        // the `World::run` better to handle this, but this works for now.
        tokio::time::sleep(Duration::from_millis(2000)).await;

        // TODO: Send a specific message and see if we get the swap.
        messager
            .send(To::Agent(SWAPPER.to_owned()), ExecuteSwap)
            .await
            .unwrap();
        debug!("message sent to swapper");

        let dfmm_address =
            eAddress::from_str("0x4dcb76f01b5624fecb5c7663892cb7977e9aaaa0").unwrap();
        let mut filter = stream_event(DFMM::new(dfmm_address, client).swap_filter());
        if let Some(message) = filter.next().await {
            debug!("Swap Event: {:?}", message);
            assert_eq!(message.input_amount, parse_ether(0.5).unwrap())
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
