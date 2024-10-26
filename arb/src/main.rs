use alloy::{
    primitives::{address},
    providers::{ProviderBuilder},
};

use eyre::Result;

use tokens::erc20::{Erc20};


async fn main() -> Result<()> {
    println!("Arb");

    // Input that will be supplied - maybe via environment.
    let url = "https://rpc.immutable.com";

    let tok0_address = address!("52A6c53869Ce09a731CD772f245b97A4401d3348");
//    let tok1_address = address!("3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D");

    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);


    let tok0_contract = Erc20::new(tok0_address, &provider).await?;
    let tok0_symbol = tok0_contract.symbol();

    println!(" Token0: {}, {}", tok0_symbol, tok0_address);

    Ok(())
}
    