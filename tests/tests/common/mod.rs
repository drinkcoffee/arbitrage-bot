use ethers::utils::{Anvil, AnvilInstance};
use lib::prelude::*;
use rand::Rng;

pub async fn spawn_anvil() -> (AnvilInstance, RootProvider) {
    let port: u16 = rand::thread_rng().gen_range(1000..9999);
    // This will panic if a port clash occurs.
    let anvil = Anvil::new().port(port).spawn();
    let provider = ProviderBuilder::new().on_http(Url::parse(&anvil.endpoint()).unwrap());
    (anvil, provider)
}
