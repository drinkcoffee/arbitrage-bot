use clap::{Args, Subcommand};
use eyre::Result;

use alloy::primitives::{Address, address};
use tokens::erc20::Erc20;

use lib::prelude::*;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Erc20Args {
    #[command(subcommand)]
    pub command: Erc20Commands,
}

#[derive(Debug, Subcommand)]
pub enum Erc20Commands {
    Symbol(Erc20SymbolArgs),
    List(Erc20ListArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Erc20SymbolArgs {
    pub address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Erc20ListArgs {
}


pub async fn erc20_symbol(args: Erc20SymbolArgs, provider: RootProvider) -> Result<()> {
    let tok0_address = args.address;
    let tok0_contract = Erc20::new(tok0_address, provider).await?;
    let tok0_symbol = tok0_contract.symbol().await?;

    println!(" Token0: {}, {}", tok0_symbol, tok0_address);

    Ok(())
}

const ETHEREUM_WETH: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const ETHEREUM_USDC: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
const ETHEREUM_IMX: Address = address!("f57e7e7c23978c3caec3c3548e3d615c346e79ff");
const ETHEREUM_GOG: Address = address!("9ab7bb7fdc60f4357ecfef43986818a2a3569c62");

const IMMUTABLE_USDC: Address = address!("6de8aCC0D406837030CE4dd28e7c08C5a96a30d2");
const IMMUTABLE_WETH: Address = address!("52A6c53869Ce09a731CD772f245b97A4401d3348");
const IMMUTABLE_WIMX: Address = address!("3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D");
const IMMUTABLE_GOG: Address = address!("b00ed913aAFf8280C17BfF33CcE82fE6D79e85e8");

pub async fn erc20_list(_args: Erc20ListArgs, provider: RootProvider) -> Result<()> {
    let id = provider.get_chain_id().await?;
    println!("Known tokens for chain {}", id);
    println!(" Symbol  Address                                     Decimals  Supply");

    match id {
        1 => {
            print_erc20_info(ETHEREUM_WETH, provider.clone()).await?;
            print_erc20_info(ETHEREUM_USDC, provider.clone()).await?;
            print_erc20_info(ETHEREUM_IMX, provider.clone()).await?;
            print_erc20_info(ETHEREUM_GOG, provider.clone()).await?;
        }
        13371 => {
            print_erc20_info(IMMUTABLE_WETH, provider.clone()).await?;
            print_erc20_info(IMMUTABLE_USDC, provider.clone()).await?;
            print_erc20_info(IMMUTABLE_WIMX, provider.clone()).await?;
            print_erc20_info(IMMUTABLE_GOG, provider.clone()).await?;
        }
        _ => {
            println!(" No known tokens for this chain");
        }
    }

    Ok(())
}

pub async fn print_erc20_info(erc20_address: Address, provider: RootProvider) -> Result<()> {
    let contract = Erc20::new(erc20_address, provider).await?;
    let symbol = contract.symbol().await?;
    let decimals = contract.decimals().await?;
    let total_supply = contract.total_supply().await?;
    println!(" {:<7} {}  {:<3}       {}", symbol, erc20_address, decimals, total_supply);
    Ok(())
}
