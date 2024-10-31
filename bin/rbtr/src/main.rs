mod config;
use std::time::Duration;

use actix::prelude::*;
use config::Config;
use eyre::{Ok, Result};

mod actors;
use actors::*;

use tokio::{select, task};
use tokio_util::sync::CancellationToken;

fn main() -> Result<()> {
    // Parse config
    let config = Config::parse("./bin/rbtr/config.toml")?;
    println!("{:#?}", config);

    let system = System::new();
    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    // Handle cancellation and wait for shutdown
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        cancel_clone.cancel();
    })?;

    system.block_on(async {
        // Set up actors and their subscriptions
        let resolver = Resolver::new().start();
        let monitor = Monitor::new().start();
        let subscribe = Subscribe::<PriceDiff>(resolver.clone().recipient());
        monitor
            .send(subscribe)
            .await
            .expect("Failed to set initial subscription to Monitor");

        // Drive the actors
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            select! {
                _ = cancel.cancelled() => {
                    if let Err(e) = monitor.send(Stop {}).await {
                        eprintln!("Failed to send Stop to Monitor: {e}");
                    }
                    if let Err(e) = resolver.send(Stop {}).await {
                        eprintln!("Failed to send Stop to Resolver: {e}");
                    }
                    System::current().stop();
                    break;
                }
                _ = interval.tick() => {
                    if let Err(e) = monitor.send(Tick {}).await {
                        eprintln!("Failed to send Tick to Monitor: {e}");
                    }
                }
            }
        }
    });

    system.run()?;
    println!("Shutting down");
    Ok(())
}
