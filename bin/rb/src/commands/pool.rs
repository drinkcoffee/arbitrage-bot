use alloy::{primitives::address, providers::ProviderBuilder};

use clap::{Args, Subcommand};
use eyre::Result;

use uniswapv3pool::univ3contract::UniswapV3PoolContract;
use uniswapv3pool::univ3sdk::UniswapV3PoolSdk;


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PoolArgs {
    #[command(subcommand)]
    pub command: Option<PoolCommands>,
}

#[derive(Debug, Subcommand)]
pub enum PoolCommands {
    TickSpacing,
    CurrentTick,
    Dump,
}

pub async fn pool_tick_spacing() -> Result<()> {
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

pub async fn pool_current_tick_command() -> Result<()> {
    println!("Arb");

    // Input that will be supplied - maybe via environment.
    let url = "https://rpc.immutable.com";

    let pool_address = address!("EE997F15Eaca3012E4825F1AeFE12136216CF3AF");

    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let pool_contract = UniswapV3PoolContract::new(pool_address, &provider).await?;
    let result = pool_contract.current_tick().await;
    let current_tick = match result {
        Ok(res) => res,
        Err(error) => panic!("Problem fetching Uniswap V3 tick spacing: {error:?}"),
    };

    println!(" Current Tick: {}", current_tick);

    Ok(())
}


pub async fn pool_tick_dump_command() -> Result<()> {
    println!("Arb");

    // Input that will be supplied - maybe via environment.
    let url = "https://rpc.immutable.com";

    let pool_address = address!("EE997F15Eaca3012E4825F1AeFE12136216CF3AF");

    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let pool = UniswapV3PoolSdk::new(pool_address, &provider, None).await?;
    let result = pool.dump().await;
    let ignore = match result {
        Ok(res) => res,
        Err(error) => panic!("Error thrown: {error:?}"),
    };

    println!(" Ignore: {}", ignore);

    Ok(())
}
