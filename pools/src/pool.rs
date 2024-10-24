use alloy::providers::{ProviderBuilder, RootProvider};
use eyre::Result;
use uniswap_sdk_core::{prelude::*, token};
use uniswap_v3_sdk::prelude::*;


struct PoolType {
    // TODO import Http and Client
    provider: RootProvider<Http<Client>>,
    chain_id: ChainId,
    factory_address: Address,
    tok0: Token,
    tok1: Token,
    fee: FeeAmount,
}


impl APool for PoolType {
    async fn check_pool(&self) -> Result<()> {
        // Construct a pool
        let pool = Pool::from_pool_key(
            self.chain_id,
            self.factory_address,
            self.tok0.address,
            self.tok1.address,
            self.fee,
            self.provider,
            None,
        )
        .await?;

        // Use the pool
        println!("Liquidity Pool: ");
        // TODO need to catch the following call if the contract doesn't exist
        println!(" Token0:    {:?}", pool.token0.symbol().unwrap());
        println!(" Token1:    {:?}", pool.token1.symbol().unwrap());
        println!(" Fee tier:  {:?}", self.fee);
        println!(" Pool addr: {:?}", pool.address(None, Some(self.factory_address)));
        println!(" Liquidity: {:?}", pool.liquidity);

        Ok(())

    }
}

