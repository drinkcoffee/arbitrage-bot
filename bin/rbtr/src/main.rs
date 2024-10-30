mod config;
use config::Config;
use eyre::Result;

fn main() -> Result<()> {
    let config = Config::parse("./bin/rbtr/config.toml")?;
    println!("{:#?}", config);
    Ok(())
}
