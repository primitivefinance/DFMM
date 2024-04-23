use std::time::Duration;

use arbiter_engine::messager::To;
use dfmm_kit::behaviors::TokenAdminQuery;
use futures_util::StreamExt;
use tracing::info;

include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn run_token_admin() {
    log(Level::DEBUG);

    let mut world = World::new("test");
    let messager = world.messager.for_agent("test");

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);

    let world_task = world.run();

    let task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Sleep because the world needs to give all of the agents time to build their
        // receivers. TODO: This is a bit of a hack and we could honestly make
        // the `World::run` better to handle this, but this works for now.
        tokio::time::sleep(Duration::from_millis(2000)).await;
        messager
            .send(
                To::Agent(TOKEN_ADMIN.to_owned()),
                TokenAdminQuery::GetAssetUniverse,
            )
            .await
            .unwrap();
        let mut stream = messager.stream().unwrap();
        while let Some(message) = stream.next().await {
            info!("Saw message: {:#?}", message);
            match serde_json::from_str::<Vec<(TokenData, eAddress)>>(&message.data) {
                Ok(data) => {
                    info!("Saw data: {:#?}", data);
                    let token_x = TokenData {
                        name: TOKEN_X_NAME.to_owned(),
                        symbol: TOKEN_X_SYMBOL.to_owned(),
                        decimals: TOKEN_X_DECIMALS,
                        address: None,
                    };
                    let token_y = TokenData {
                        name: TOKEN_Y_NAME.to_owned(),
                        symbol: TOKEN_Y_SYMBOL.to_owned(),
                        decimals: TOKEN_Y_DECIMALS,
                        address: None,
                    };
                    let mock_data = token::Config {
                        token_data: vec![token_x, token_y],
                    };
                    assert_eq!(data[0].0.name, mock_data.token_data[0].name);
                    assert_eq!(data[0].0.symbol, mock_data.token_data[0].symbol);
                    assert_eq!(data[1].0.name, mock_data.token_data[1].name);
                    assert_eq!(data[1].0.symbol, mock_data.token_data[1].symbol);
                    info!("Asserts passed!");
                    break;
                }
                Err(_) => {
                    continue;
                }
            }
        }
    });

    // Setup a timeout for the test to ensure it does not run indefinitely.
    let timeout_duration = Duration::from_secs(10); // Adjust the timeout as needed.

    tokio::select! {
        _ = world_task => {
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
