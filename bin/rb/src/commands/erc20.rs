use alloy::primitives::address;

use clap::{Args, Subcommand};
use eyre::Result;

use tokens::erc20::Erc20;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Erc20Args {
    #[command(subcommand)]
    pub command: Option<Erc20Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Erc20Commands {
    Symbol,
}

pub async fn erc20_symbol(provider: lib::Provider) -> Result<()> {
    println!("Arb");

    let tok0_address = address!("52A6c53869Ce09a731CD772f245b97A4401d3348");

    let tok0_contract = Erc20::new(tok0_address, provider).await?;
    let tok0_symbol_result = tok0_contract.symbol().await;
    let tok0_symbol = match tok0_symbol_result {
        Ok(res) => res,
        Err(error) => panic!("Problem fetching ERC20 symbol: {error:?}"),
    };

    println!(" Token0: {}, {}", tok0_symbol, tok0_address);

    Ok(())
}
