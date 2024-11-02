use clap::{Args, Subcommand};
use eyre::Result;

use alloy::primitives::{address, Address};
use lib::prelude::*;

use tokens::erc20_constants;
use uniswap_v3_sdk::prelude::FeeAmount;
use uniswapv3pool::pool_constants;
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
    List(ListArgs),
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

const ADDR_ZERO: &str = "0000000000000000000000000000000000000000";
//const ADDRESS_ZERO: Address = address!("0000000000000000000000000000000000000000");

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ListArgs {
    #[arg(
        required = false,
        help = "UniswapV3Factory address",
        default_value = ADDR_ZERO,
        env
    )]
    pub factory_address: Address,
    #[arg(
        required = false,
        help = "Token A address",
        default_value = ADDR_ZERO,
        env
    )]
    pub token_one_address: Address,
    #[arg(
        required = false,
        help = "Token B address",
        default_value = ADDR_ZERO,
        env
    )]
    pub token_two_address: Address,
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

pub async fn pool_list(args: ListArgs, provider: RootProvider) -> Result<()> {
    let id = provider.get_chain_id().await?;
    let ListArgs {
        factory_address,
        token_one_address,
        token_two_address,
    } = args;

    let def_factory;
    let def_token_one;
    let def_token_two;

    match get_defaults(id) {
        Ok(res) => {
            def_factory = res.0;
            def_token_one = res.1;
            def_token_two = res.2;
        }
        Err(error) => {
            panic!("Error {:?}", error);
        }
    }

    println!(
        "args: f: {}, ta: {}, tb: {}",
        factory_address, token_one_address, token_two_address
    );
    println!(
        "defs: f: {:?}, ta: {}, tb: {}",
        def_factory, def_token_one, def_token_two
    );
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

fn get_defaults(id: u64) -> Result<(Address, Address, Address)> {
    match id {
        1 => Ok((
            pool_constants::ETHEREUM_UNISWAP_FACTORY,
            erc20_constants::ETHEREUM_WETH,
            erc20_constants::ETHEREUM_IMX,
        )),
        13371 => Ok((
            pool_constants::IMMUTABLE_QUICKSWAP_FACTORY,
            erc20_constants::IMMUTABLE_WETH,
            erc20_constants::IMMUTABLE_WIMX,
        )),
        _ => {
            panic!("No default values for chain {}", id)
        }
    }
}
