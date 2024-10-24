use alloy::providers::ProviderBuilder;
use eyre::Result;
use uniswap_sdk_core::{prelude::*, token};
use uniswap_v3_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Construct provider
    let rpc_url = "https://rpc.immutable.com".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Construct a pool
    let factory_address = address!("56c2162254b0E4417288786eE402c2B41d4e181e");
    let eth = token!(1, "52A6c53869Ce09a731CD772f245b97A4401d3348", 18);
    let imx = token!(1, "3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D", 18);
    let pool = Pool::from_pool_key(
        13371,
        factory_address,
        eth.address,
        imx.address,
        FeeAmount::MEDIUM,
        provider,
        None,
    )
    .await?;

    // Use the pool
    println!("token0:    {:?}", pool.token0.symbol().unwrap());
    println!("token1:    {:?}", pool.token1.symbol().unwrap());
    println!("pool addr: {:?}", pool.address(None, Some(factory_address)));
    println!("liquidity: {:?}", pool.liquidity);

    Ok(())
}
