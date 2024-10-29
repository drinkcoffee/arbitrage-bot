use alloy::{primitives::address, providers::ProviderBuilder};

use clap::{Args, Subcommand};
use eyre::Result;

use uniswapv3pool::univ3contract::UniswapV3PoolContract;
use uniswapv3pool::univ3sdk::UniswapV3PoolSdk;

use uniswap_v3_sdk::prelude::FeeAmount;


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

    //let pool_address = address!("EE997F15Eaca3012E4825F1AeFE12136216CF3AF");

    let factory_address = address!("56c2162254b0E4417288786eE402c2B41d4e181e");
    let tok0_address = address!("52A6c53869Ce09a731CD772f245b97A4401d3348");
    let tok1_address = address!("3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D");


    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let pool = UniswapV3PoolSdk::from_pool_key (
        13371, 
        factory_address,
        tok0_address,
        tok1_address,
        FeeAmount::MEDIUM,
        &provider,
        None).await?;
    let result = pool.dump().await;
    let ignore = match result {
        Ok(res) => res,
        Err(error) => panic!("Error thrown: {error:?}"),
    };

    println!(" Ignore: {}", ignore);

    Ok(())
}
