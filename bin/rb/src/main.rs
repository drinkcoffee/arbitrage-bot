use alloy::{providers::ProviderBuilder, transports::http::reqwest::Url};
use clap::{Parser, Subcommand};

mod commands;
use commands::{
    erc20_symbol, id, pool_current_tick_command, pool_tick_dump_command, pool_tick_info_command,
    pool_tick_spacing, ChainArgs, Erc20Args, Erc20Commands, PoolArgs, PoolCommands,
};
use lib::prelude::*;

#[derive(Debug, Parser)]
#[command(name = "rb")]
#[command(about = "R-bitrage Bot", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        global = true,
        required = false,
        help = "URL to remote RPC",
        default_value = "https://rpc.immutable.com",
        env
    )]
    rpc: Url,
}

impl Cli {
    fn into_command(self) -> (Commands, RootProvider) {
        (self.command, ProviderBuilder::new().on_http(self.rpc))
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Erc20(Erc20Args),
    Pool(PoolArgs),
    Chain(ChainArgs),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    match cli.into_command() {
        (Commands::Erc20(args), provider) => match args.command {
            Some(Erc20Commands::Symbol(args)) => erc20_symbol(args, provider).await,
            None => unreachable!(),
        },
        (Commands::Pool(args), _) => match args.command {
            Some(PoolCommands::TickSpacing) => pool_tick_spacing().await,
            Some(PoolCommands::CurrentTick) => pool_current_tick_command().await,
            Some(PoolCommands::Dump) => pool_tick_dump_command().await,
            Some(PoolCommands::Info) => pool_tick_info_command().await,
            None => unreachable!(),
        },
        (Commands::Chain(args), provider) => match args.command {
            commands::ChainCommands::ID => id(provider).await,
            commands::ChainCommands::Latest => todo!(),
            commands::ChainCommands::Finalized => todo!(),
        },
    }
}
