include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn creator_behavior_test() {
    let mut world = World::new("test");
    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);
    spawn_creator(&mut world);

    world.run().await.unwrap();
}
