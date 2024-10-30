use lib::prelude::Provider;
mod common;
use common::spawn_anvil;

#[tokio::test]
async fn token() {
    let (anvil, provider) = spawn_anvil().await;
    println!("{}", provider.get_chain_id().await.unwrap())
}
