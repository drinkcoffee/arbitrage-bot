use alloy::eips::BlockId;
use clap::{Args, Subcommand};
use eyre::{eyre, Context, Result};
use lib::prelude::*;

#[derive(Debug, Args)]
pub struct ChainArgs {
    #[command(subcommand)]
    pub command: ChainCommands,
}

#[derive(Debug, Subcommand)]
pub enum ChainCommands {
    ID,
    Latest,
    Finalized,
}

pub async fn chain_id(provider: RootProvider) -> Result<()> {
    let id = provider
        .get_chain_id()
        .await
        .wrap_err("failed to get chain ID")?;

    println!("Chain ID: {id}");
    Ok(())
}

pub async fn latest(provider: RootProvider) -> Result<()> {
    let latest = provider
        .get_block_number()
        .await
        .wrap_err("failed to get latest block number")?;

    println!("Latest block number: {latest}");
    Ok(())
}

pub async fn finalized(provider: RootProvider) -> Result<()> {
    let finalized = provider
        .get_block(BlockId::finalized(), Default::default())
        .await
        .wrap_err("failed to get finalized block")?
        .ok_or(eyre!("finalized block is still pending"))?;

    println!(
        "Most recent finalized block number: {}",
        finalized.header.number
    );
    Ok(())
}
