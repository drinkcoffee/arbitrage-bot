use duration_string::DurationString;
use eyre::{Context, Result};
use lib::prelude::*;
use serde::Deserialize;
use std::{fs, path::Path, time::Duration};

#[derive(Deserialize)]
struct RawConfig {
    tick_rate: DurationString,
    monitors: Vec<RawMonitorConfig>,
}

#[derive(Deserialize)]
struct RawMonitorConfig {
    rpc_url: String,
    factory: Address,
    token_one: Address,
    token_two: Address,
}

#[derive(Debug)]
pub struct Config {
    pub tick_rate: Duration,
    pub monitors: Vec<MonitorConfig>,
}

impl From<RawConfig> for Config {
    fn from(raw: RawConfig) -> Self {
        let monitors = raw.monitors.into_iter().map(MonitorConfig::from).collect();
        Self {
            tick_rate: raw.tick_rate.into(),
            monitors,
        }
    }
}

#[derive(Debug)]
pub struct MonitorConfig {
    pub provider: RootProvider,
    pub factory: Address,
    pub token_one: Address,
    pub token_two: Address,
}

impl From<RawMonitorConfig> for MonitorConfig {
    fn from(raw: RawMonitorConfig) -> Self {
        let rpc_url = Url::parse(&raw.rpc_url).unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url);
        Self {
            provider,
            factory: raw.factory,
            token_one: raw.token_one,
            token_two: raw.token_two,
        }
    }
}

impl Config {
    pub fn parse(file_name: impl AsRef<Path>) -> Result<Config> {
        // Read toml in and parse.
        let file_str =
            fs::read_to_string(file_name).wrap_err("failed to open file at specified path")?;
        let raw =
            toml::from_str::<RawConfig>(&file_str).wrap_err("failed to parse config from toml")?;

        Ok(raw.into())
    }
}
