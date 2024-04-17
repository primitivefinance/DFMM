use std::time::Duration;

use tracing::info;

include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn run_token_admin() {
    log();

    let mut world = World::new("test");
    let mut messager = world.messager.clone();

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);

    let task = tokio::spawn(async move {
        loop {
            if let Ok(message) = messager.get_next::<Vec<TokenData>>().await {
                let data = message.data;
                info!("Saw message data: {:#?}", data);

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
                assert_eq!(data[0].name, mock_data.token_data[0].name);
                assert_eq!(data[0].symbol, mock_data.token_data[0].symbol);
                assert_eq!(data[1].name, mock_data.token_data[1].name);
                assert_eq!(data[1].symbol, mock_data.token_data[1].symbol);

                info!("Asserts passed!");
                break;
            } else {
                info!("Got another message.");
            }
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
