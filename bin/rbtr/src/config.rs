use eyre::{Context, Result};
use lib::prelude::*;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct RawConfig {
    url_one: String,
    url_two: String,
    token_one: Address,
    token_two: Address,
}

#[derive(Debug)]
pub struct Config {
    provider_one: RootProvider,
    provider_two: RootProvider,
    token_address_one: Address,
    token_address_two: Address,
}

impl Config {
    pub fn parse(file_name: impl AsRef<Path>) -> Result<Config> {
        // Read toml in and parse.
        let file_str =
            fs::read_to_string(file_name).wrap_err("failed to open file at specified path")?;
        let raw =
            toml::from_str::<RawConfig>(&file_str).wrap_err("failed to parse config from toml")?;

        // Parse URLs and create the providers.
        let url_one =
            Url::parse(&raw.url_one).wrap_err(format!("failed to parse url: {}", raw.url_one))?;
        let url_two =
            Url::parse(&raw.url_two).wrap_err(format!("failed to parse url: {}", raw.url_two))?;
        let provider_one = ProviderBuilder::new().on_http(url_one);
        let provider_two = ProviderBuilder::new().on_http(url_two);

        Ok(Self {
            provider_one,
            provider_two,
            token_address_one: raw.token_one,
            token_address_two: raw.token_two,
        })
    }
}
