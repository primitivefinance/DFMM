include!("common.rs");

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn run_token_admin() {
    log();

    let mut world = World::new("test");
    spawn_deployer(&mut world);
    spawn_token_admin(&mut world);

    world.run().await.unwrap();
}
