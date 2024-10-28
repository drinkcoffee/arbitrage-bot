use alloy::{
    eips::BlockId, 
    primitives::Address,
    providers::Provider,
    transports::Transport,
};


use uniswap_v3_sdk::extensions::EphemeralTickDataProvider;
use uniswap_v3_sdk::entities::TickDataProvider;

use eyre::Result;



pub struct UniswapV3PoolSdk {
    pub tick_data_provider: EphemeralTickDataProvider,
}

impl UniswapV3PoolSdk {
    pub async fn new<T, P>(
        pool_address: Address,
        provider: P,
        block_number: Option<BlockId>,
    ) -> Result<Self>
    where
        T: Transport + Clone,
        P: Provider<T> + Clone,
    {
        let tick_data_provider = 
            EphemeralTickDataProvider::new(
            pool_address,
            provider,
            None,
            None,
            block_number,
        )
        .await?;
        Ok(Self{tick_data_provider})
    }


    // Some temporary code so that we can see what information is available.
    pub async fn dump(&self)  -> Result<i64> {
        println!("Tick lower: {}", self.tick_data_provider.tick_lower);
        println!("Tick upper: {}", self.tick_data_provider.tick_upper);
        println!("Tick spacing: {}", self.tick_data_provider.tick_spacing);
        println!("Pool address: {}", self.tick_data_provider.pool);
        println!("Tick vector size: {}", self.tick_data_provider.ticks.len());

        let lower = self.tick_data_provider.tick_lower;
        let spacing = self.tick_data_provider.tick_spacing;
        // Note: This call result in error: Below smallest tick
        // let tick = self.tick_data_provider.get_tick(lower)?;
        // println!("Tick lower: index: {}", tick.index);
        // println!("Tick lower: liquidity gross: {}", tick.liquidity_gross);
        // println!("Tick lower: liquidity net: {}", tick.liquidity_net);

        let upper = self.tick_data_provider.tick_upper;
        // Note: This call result in error: Not contained in tick list
        // let tick = self.tick_data_provider.get_tick(upper)?;
        // println!("Tick upper: index: {}", tick.index);
        // println!("Tick upper: liquidity gross: {}", tick.liquidity_gross);
        // println!("Tick upper: liquidity net: {}", tick.liquidity_net);

        println!("Tick vector:");
        for tick in self.tick_data_provider.ticks.iter() {
            println!("Tick: {}, liq gross: {}, liq net: {}", tick.index, tick.liquidity_gross, tick.liquidity_net);
        }

        // Note: no matter whether I pass in upper or lower, this errors with Below smallest tick
        let (tick_num, initialized) = self.tick_data_provider.next_initialized_tick_within_one_word(
            upper,
            true,
            spacing,
        )?;
        println!("Next tick initialised: {}", initialized);
        println!("Next tick number: {}", tick_num);

        println!("Tick lower: {}", self.tick_data_provider.tick_lower);
        let hack = lower.as_i64();
        Ok(hack)
    }


}
