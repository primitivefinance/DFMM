use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn run_creator_constant_sum() {
    // log(Level::DEBUG);

    let mut world = World::new("test");
    let mut messager = world.messager.clone();

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);
    spawn_constant_sum_creator(&mut world);

    let task = tokio::spawn(async move {
        loop {
            if let Ok(message) = messager
                .get_next::<dfmm_kit::pool::PoolCreation<ConstantSumPool>>()
                .await
            {
                let data = message.data;
                info!("Saw message data: {:#?}", data);

                let id = data.id;
                let params = ConstantSumParams {
                    price: WAD,
                    swap_fee: ethers::utils::parse_ether(0.003).unwrap(),
                    controller: eAddress::from_str("0x6965a885fde448e06b1cadd5bf15698c47cf4ab3")
                        .unwrap(),
                };
                let allocation_data = ConstantSumAllocationData {
                    reserve_x: RESERVE_X,
                    reserve_y: RESERVE_Y,
                };

                assert_eq!(data.id, id);
                assert_eq!(data.params, params);
                assert_eq!(data.allocation_data, allocation_data);
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
