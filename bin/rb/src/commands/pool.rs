use clap::{Args, Subcommand};
use eyre::Result;

use lib::prelude::*;

use uniswap_v3_sdk::prelude::FeeAmount;
use uniswapv3pool::univ3contract::UniswapV3PoolContract;
use uniswapv3pool::univ3sdk::UniswapV3PoolSdk;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PoolArgs {
    #[command(subcommand)]
    pub command: PoolCommands,
}

#[derive(Debug, Subcommand)]
pub enum PoolCommands {
    TickSpacing(TickSpacingArgs),
    CurrentTick(CurrentTickArgs),
    Dump(DumpArgs),
    Info(InfoArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TickSpacingArgs {
    pub pool_address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CurrentTickArgs {
    pub pool_address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DumpArgs {
    pub factory_address: Address,
    pub token_one_address: Address,
    pub token_two_address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InfoArgs {
    pub factory_address: Address,
    pub token_one_address: Address,
    pub token_two_address: Address,
}

pub async fn pool_tick_spacing(args: TickSpacingArgs, provider: RootProvider) -> Result<()> {
    let pool_contract = UniswapV3PoolContract::new(args.pool_address, provider).await?;
    let tick_spacing = pool_contract.tick_spacing().await?;

    println!(" Tick Spacing: {}", tick_spacing);
    Ok(())
}

pub async fn pool_current_tick(args: CurrentTickArgs, provider: RootProvider) -> Result<()> {
    let pool_contract = UniswapV3PoolContract::new(args.pool_address, provider).await?;
    let result = pool_contract.current_tick().await?;

    println!(" Current Tick: {}", result);
    Ok(())
}

pub async fn pool_tick_dump(args: DumpArgs, provider: RootProvider) -> Result<()> {
    let DumpArgs {
        factory_address,
        token_one_address,
        token_two_address,
    } = args;

    let pool = UniswapV3PoolSdk::from_pool_key(
        13371,
        factory_address,
        token_one_address,
        token_two_address,
        FeeAmount::MEDIUM,
        provider,
        None,
    )
    .await?;
    let dump = pool.dump().await?;

    println!("Pool dump: {}", dump);
    Ok(())
}

pub async fn pool_tick_info(args: InfoArgs, provider: RootProvider) -> Result<()> {
    let InfoArgs {
        factory_address,
        token_one_address,
        token_two_address,
    } = args;

    let pool = UniswapV3PoolSdk::from_pool_key(
        13371,
        factory_address,
        token_one_address,
        token_two_address,
        FeeAmount::MEDIUM,
        provider,
        None,
    )
    .await?;
    let info = pool.info().await?;

    println!("Pool info: {}", info);
    Ok(())
}
