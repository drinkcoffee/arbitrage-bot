use alloy_primitives::{address, Address};
use uniswap_sdk_core::{prelude::Token, token};
use uniswap_v3_sdk::prelude::*;

fn main() {
    println!("Analyse");


    let usdc = token!(1, "A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", 6);
    let dai = token!(1, "6B175474E89094C44Da98b954EedeAC495271d0F", 18);
    let result = Pool::get_address(&usdc, &dai, FeeAmount::LOW, None, None);
    assert_eq!(result, address!("6c6Bc977E13Df9b0de53b251522280BB72383700"));

    println!("Address: {result}");
}
