use clap::{Args, Subcommand};
use eyre::Result;

use tokens::erc20::Erc20;
use tokens::erc20_constants;

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
pub struct Erc20ListArgs {}

pub async fn erc20_symbol(args: Erc20SymbolArgs, provider: RootProvider) -> Result<()> {
    let tok0_address = args.address;
    let tok0_contract = Erc20::new(tok0_address, provider).await?;
    let tok0_symbol = tok0_contract.symbol().await?;

    println!(" Token0: {}, {}", tok0_symbol, tok0_address);

    Ok(())
}

pub async fn erc20_list(_args: Erc20ListArgs, provider: RootProvider) -> Result<()> {
    let id = provider.get_chain_id().await?;
    println!("Known tokens for chain {}", id);
    println!(" Symbol  Address                                     Decimals  Supply");

    match id {
        1 => {
            print_erc20_info(erc20_constants::ETHEREUM_WETH, provider.clone()).await?;
            print_erc20_info(erc20_constants::ETHEREUM_USDC, provider.clone()).await?;
            print_erc20_info(erc20_constants::ETHEREUM_IMX, provider.clone()).await?;
            print_erc20_info(erc20_constants::ETHEREUM_GOG, provider.clone()).await?;
        }
        13371 => {
            print_erc20_info(erc20_constants::IMMUTABLE_WETH, provider.clone()).await?;
            print_erc20_info(erc20_constants::IMMUTABLE_USDC, provider.clone()).await?;
            print_erc20_info(erc20_constants::IMMUTABLE_WIMX, provider.clone()).await?;
            print_erc20_info(erc20_constants::IMMUTABLE_GOG, provider.clone()).await?;
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
    println!(
        " {:<7} {}  {:<3}       {}",
        symbol, erc20_address, decimals, total_supply
    );
    Ok(())
}
