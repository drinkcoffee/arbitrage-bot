use clap::{Args, Parser, Subcommand};

mod erc20_commands;
use erc20_commands::erc20_symbol_command;

#[derive(Debug, Parser)]
#[command(name = "rb")]
#[command(about = "R-bitrage Bot", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Erc20(Erc20Args),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Erc20Args {
    #[command(subcommand)]
    command: Option<Erc20Commands>,
}

#[derive(Debug, Subcommand)]
enum Erc20Commands {
    Symbol,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Erc20(args) => match args.command {
            Some(Erc20Commands::Symbol) => erc20_symbol_command().await,
            None => unreachable!(),
        },
    }
}
