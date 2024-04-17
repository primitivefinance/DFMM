use std::time::Duration;

use tokio::time::timeout;
use tracing::info;
include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn run_creator_constant_sum() {
    log(Level::DEBUG);

    let mut world = World::new("test");
    let mut messager = world.messager.clone();

    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);
    spawn_constant_sum_creator(&mut world);

    let task = tokio::spawn(async move {
        loop {
            if let Ok(message) = messager
                .get_next::<creator::PoolCreation<ConstantSumPool>>()
                .await
            {
                let data = message.data;
                info!("Saw message data: {:#?}", data);

                let mock_creation = creator::PoolCreation::<ConstantSumPool> {
                    id: data.id,
                    params: ConstantSumParams {
                        price: WAD,
                        swap_fee: ethers::utils::parse_ether(0.003).unwrap(),
                        controller: eAddress::zero(),
                    },
                    allocation_data: ConstantSumAllocationData {
                        reserve_x: RESERVE_X,
                        reserve_y: RESERVE_Y,
                    },
                };
                assert_eq!(data.id, mock_creation.id);
                assert_eq!(data.params, mock_creation.params);
                assert_eq!(data.allocation_data, mock_creation.allocation_data);
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

// #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
// async fn run_creator_geometric_mean() {
//     log();

//     let mut world = World::new("test");
//     let mut messager = world.messager.clone();

//     spawn_deployer(&mut world);
//     spawn_token_admin(&mut world);
//     spawn_geometric_mean_creator(&mut world);

//     let task = tokio::spawn(async move {
//         loop {
//             if let Ok(message) = messager
//                 .get_next::<creator::PoolCreation<GeometricMeanPool>>()
//                 .await
//             {
//                 let data = message.data;
//                 info!("Saw message data: {:#?}", data);

//                 let mock_creation =
// creator::PoolCreation::<GeometricMeanPool> {                     id: data.id,
//                     params: GeometricMeanParams {
//                         target_weight_y: eU256::zero(),
//                         target_weight_x: WAD,
//                     },
//                     allocation_data: GeometricMeanAllocationData {
//                         amount_x: RESERVE_X,
//                         price: WAD,
//                     },
//                 };
//                 assert_eq!(data.id, mock_creation.id);
//                 assert_eq!(data.params, mock_creation.params);
//                 assert_eq!(data.allocation_data,
// mock_creation.allocation_data);                 info!("Asserts passed!");
//                 break;
//             } else {
//                 info!("Got another message.");
//             }
//         }
//     });

//     // Setup a timeout for the test to ensure it does not run indefinitely.
//     let timeout_duration = Duration::from_secs(5); // Adjust the timeout as
// needed.

//     tokio::select! {
//         _ = world.run() => {
//             panic!("World run unexpectedly completed");
//         },
//         result = task => {
//             match result {
//                 Ok(_) => println!("Task completed successfully and test
// should pass."),                 Err(e) => panic!("Task encountered an error:
// {:?}", e),             }
//         },
//         _ = tokio::time::sleep(timeout_duration) => {
//             panic!("Test timed out");
//         }
//     }
// }
