use alloy_primitives::Address;
use clap::{Args, Subcommand};
use eyre::Result;

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
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Erc20SymbolArgs {
    pub address: Address,
}

pub async fn erc20_symbol(args: Erc20SymbolArgs, provider: RootProvider) -> Result<()> {
    let tok0_address = args.address;
    let tok0_contract = Erc20::new(tok0_address, provider).await?;
    let tok0_symbol = tok0_contract.symbol().await?;

    println!(" Token0: {}, {}", tok0_symbol, tok0_address);

    Ok(())
}
