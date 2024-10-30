use clap::{Parser, Subcommand};

mod commands;
use commands::{
    chain_id, erc20_symbol, finalized, latest, pool_current_tick, pool_tick_dump, pool_tick_info,
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
            Erc20Commands::Symbol(args) => erc20_symbol(args, provider).await,
        },
        (Commands::Pool(args), _) => match args.command {
            PoolCommands::TickSpacing => pool_tick_spacing().await,
            PoolCommands::CurrentTick => pool_current_tick().await,
            PoolCommands::Dump => pool_tick_dump().await,
            PoolCommands::Info => pool_tick_info().await,
        },
        (Commands::Chain(args), provider) => match args.command {
            commands::ChainCommands::ID => chain_id(provider).await,
            commands::ChainCommands::Latest => latest(provider).await,
            commands::ChainCommands::Finalized => finalized(provider).await,
        },
    }
}
