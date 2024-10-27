use alloy::{primitives::address, providers::ProviderBuilder};

use eyre::Result;

use uniswapv3pool::univ3contract::UniswapV3PoolContract;


pub async fn pool_tick_spacing_command() -> Result<()> {
    println!("Arb");

    // Input that will be supplied - maybe via environment.
    let url = "https://rpc.immutable.com";

    let pool_address = address!("EE997F15Eaca3012E4825F1AeFE12136216CF3AF");

    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let pool_contract = UniswapV3PoolContract::new(pool_address, &provider).await?;
    let result = pool_contract.tick_spacing().await;
    let tick_spacing = match result {
        Ok(res) => res,
        Err(error) => panic!("Problem fetching Uniswap V3 tick spacing: {error:?}"),
    };

    println!(" Tick Spacing: {}", tick_spacing);

    Ok(())
}
