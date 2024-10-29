use alloy::eips::BlockId;
use clap::{Args, Subcommand};
use eyre::{eyre, Context, Result};
use lib::prelude::*;

#[derive(Args)]
pub struct ChainArgs {
    #[command(subcommand)]
    pub command: ChainCommands,
}

#[derive(Subcommand)]
pub enum ChainCommands {
    ID,
    Latest,
    Finalized,
}

pub async fn id(provider: RootProvider) -> Result<u64> {
    provider
        .get_chain_id()
        .await
        .wrap_err("failed to get chain ID")
}

pub async fn latest(provider: RootProvider) -> Result<u64> {
    provider
        .get_block_number()
        .await
        .wrap_err("failed to get latest block number")
}

pub async fn finalized(provider: RootProvider) -> Result<u64> {
    let finalized = provider
        .get_block(BlockId::finalized(), Default::default())
        .await
        .wrap_err("failed to get finalized block")?
        .ok_or(eyre!("finalized block is still pending"))?;

    Ok((finalized.header.number))
}
