mod config;

use actix::prelude::*;
use config::Config;
use eyre::{Ok, Result};

mod actors;
use actors::*;

use tokio::select;
use tokio_util::sync::CancellationToken;

fn main() -> Result<()> {
    // Parse configuration.
    let config = Config::parse("./bin/rbtr/config.toml")?;
    println!("{:#?}", config);

    // Instantiate the system and cancellation token.
    let system = System::new();
    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    // Handle SIGINT/SIGTERM.
    ctrlc::set_handler(move || {
        cancel_clone.cancel();
    })?;

    // Block the main thread and run the actor system.
    system.block_on(async {
        // Set up actors and their subscriptions.
        let resolver = Resolver::new().start();

        // Setup up the monitors.
        let mut monitors = vec![];
        for monitor_config in config.monitors.iter() {
            let monitor = Monitor::new(
                monitor_config.provider.clone(),
                monitor_config.factory,
                monitor_config.token_one,
                monitor_config.token_two,
            )
            .start();
            subscribe_to_monitor(monitor.clone(), resolver.clone()).await;
            monitors.push(monitor);
        }

        // Drive the actors.
        let mut interval = tokio::time::interval(config.tick_rate);
        loop {
            select! {
                // Shutdown.
                _ = cancel.cancelled() => {
                    for monitor in monitors.into_iter() {
                        if let Err(e) = monitor.send(Control::Stop).await {
                            eprintln!("Failed to send Stop to Monitor: {e}");
                        }
                    }
                    if let Err(e) = resolver.send(Control::Stop).await {
                        eprintln!("Failed to send Stop to Resolver: {e}");
                    }
                    System::current().stop();
                    break;
                }
                // Tick.
                _ = interval.tick() => {
                    for monitor in monitors.iter() {
                        if let Err(e) = monitor.send(Control::Tick).await {
                            eprintln!("Failed to send Tick to Monitor: {e}");
                        }
                    }
                }
            }
        }
    });

    // Wait for the system to shut down.
    system.run()?;
    println!("Shutting down");
    Ok(())
}

async fn subscribe_to_monitor(monitor: Addr<Monitor>, resolver: Addr<Resolver>) {
    let subscribe = Subscribe::<PoolUpdate>(resolver.recipient());
    monitor
        .send(subscribe)
        .await
        .expect("Failed to set subscription to Monitor");
}
