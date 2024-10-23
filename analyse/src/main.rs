use alloy::{
    primitives::{address, Address, U256},
    providers::{Provider, ProviderBuilder},
    sol,
};
// use alloy::{
//     network::EthereumWallet,
//     node_bindings::Anvil,
//     primitives::U256,
//     providers::ProviderBuilder,
//     signers::local::PrivateKeySigner,
//     sol,
// };

use eyre::Result;
use uniswap_sdk_core::{prelude::Token, token};
use uniswap_v3_sdk::prelude::*;

fn main() {
    println!("Analyse");

    stuff1();

    let uni3_pool_eth_imx = address!("EE997F15Eaca3012E4825F1AeFE12136216CF3AF");
    stuff2(uni3_pool_eth_imx).ok();

    stuff3().ok();
}

pub fn stuff1() {
    // On Immutable:
    // UniswapV3Factory: https://explorer.immutable.com/address/0x56c2162254b0E4417288786eE402c2B41d4e181e
    // WIMX: https://explorer.immutable.com/token/0x3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D
    // ETH: https://explorer.immutable.com/token/0x52A6c53869Ce09a731CD772f245b97A4401d3348
    // UniswapV3Pool: IMX - WETH - fee: 3000, https://explorer.immutable.com/address/0xEE997F15Eaca3012E4825F1AeFE12136216CF3AF

    // Values for Ethereum
    // let usdc = token!(1, "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", 6);
    // let dai = token!(1, "6B175474E89094C44Da98b954EedeAC495271d0F", 18);

    let uni3_factory = address!("56c2162254b0E4417288786eE402c2B41d4e181e");
    let eth = token!(1, "52A6c53869Ce09a731CD772f245b97A4401d3348", 18);
    let imx = token!(1, "3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D", 18);
    let uni3_pool_eth_imx = address!("EE997F15Eaca3012E4825F1AeFE12136216CF3AF");

    let result = Pool::get_address(&eth, &imx, FeeAmount::MEDIUM, None, Some(uni3_factory));

    assert_eq!(result, uni3_pool_eth_imx);

    println!("Address: {result}");

    // LOWEST = 100,
    // LOW = 500,
    // MEDIUM = 3000,
    // HIGH = 10000,
}

#[tokio::main]
async fn stuff2(uni3_pool_eth_imx: Address) -> Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    //     let rpc_url = "https://eth.merkle.io".parse()?;
    let rpc_url = "https://rpc.immutable.com".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get storage slot 0 from the Uniswap V3 USDC-ETH pool on Ethereum mainnet.
    let pool_address = uni3_pool_eth_imx;
    let storage_slot = U256::from(0);
    // The provider calls the RPC at the latest block by default. A block can exlpicitly be set
    // using `.block()`.
    let storage = provider.get_storage_at(pool_address, storage_slot).await?;

    println!("Slot 0: {storage:?}");

    Ok(())
}

// See https://alloy.rs/highlights/the-sol!-procedural-macro.html
sol! {
    contract IERC20Extended {
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address recipient, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
        function symbol() public view returns (string);
    }
}

#[tokio::main]
async fn stuff3() -> Result<()> {
    let imx_addr = address!("3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D");

    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://rpc.immutable.com".parse()?;
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    //let contract = IERC20Extended::new(provider, imx_addr);

    //println!("Contract at address: {}", contract.address());

    // let builder = contract.setNumber(U256::from(42));
    // let tx_hash = builder.send().await?.watch().await?;

    // println!("Set number to 42: {tx_hash}");

    // // Increment the number to 43.
    // let builder = contract.increment();
    // let tx_hash = builder.send().await?.watch().await?;

    // println!("Incremented number: {tx_hash}");

    // // Retrieve the number, which should be 43.
    // let builder = contract.number();
    // let number = builder.call().await?.number.to_string();

    // println!("Retrieved number: {number}");

    Ok(())
}
