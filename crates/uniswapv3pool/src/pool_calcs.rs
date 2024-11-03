use uniswap_v3_sdk::prelude::FeeAmount;

pub fn fee_to_float(fee: FeeAmount) -> f32 {
    let fee_num: usize = fee as usize;
    let fee_num = fee_num as f32;
    fee_num / 10000.0
}

pub fn tick_to_exchange_rate(tick: i32, token_one_decimals: u64, token_two_decimals: u64) -> f64 {
    let tick = tick as f64;
    let val: f64 = 1.0001;

    if token_one_decimals != token_two_decimals {
        println!("Exchange when token decimals not the same not yet supported");
    }
    val.powf(tick)
}
