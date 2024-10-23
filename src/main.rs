use eyre::Result;
use uniswap_sdk_core::{prelude::*, token};
use uniswap_v3_sdk::prelude::*;

fn main() -> Result<()> {
    // Construct a pool
    let eth = Ether::on_chain(1);
    let weth = eth.wrapped().clone();
    let usdc = token!(
        1,
        "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        6,
        "USDC",
        "USD Coin"
    );
    let fee_amount = FeeAmount::LOWEST;
    let ratio = encode_sqrt_ratio_x96(1, 1);
    let liquidity = 0;
    let pool = Pool::new(weth, usdc.clone(), fee_amount, ratio, liquidity)?;

    // Use the pool
    let output_amount = &CurrencyAmount::from_raw_amount(usdc, 100).unwrap();
    let input_amount = pool.get_input_amount(output_amount, None);
    println!("Input amount: {:?}", input_amount);

    Ok(())
}
