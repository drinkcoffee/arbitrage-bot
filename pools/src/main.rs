use std::{collections::HashMap, fmt::Display};

use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::Transport,
};
use alloy_primitives::{address, Address, ChainId};
use eyre::{Ok, Result};
use uniswap_sdk_core::{
    prelude::{BaseCurrencyCore, Token},
    token,
};
use uniswap_v3_sdk::prelude::{FeeAmount, Pool};

pub struct LiquidityPoolFactory {
    pub token0: Token,
    pub token1: Token,
    pub factory_address: Address,
    pub pools: HashMap<FeeAmount, Option<Pool>>,
}

impl LiquidityPoolFactory {
    pub async fn new<T, P>(
        chain_id: ChainId,
        factory_address: Address,
        token0: Token,
        token1: Token,
        provider: P,
    ) -> Result<Self>
    where
        T: Transport + Clone,
        P: Provider<T> + Clone,
    {
        let mut pools = HashMap::new();
        let fees = &[
            FeeAmount::LOWEST,
            FeeAmount::LOW_200,
            FeeAmount::LOW_300,
            FeeAmount::LOW_400,
            FeeAmount::LOW,
            FeeAmount::MEDIUM,
            FeeAmount::HIGH,
        ];

        fees.iter().for_each(|fee| {
            tokio_scoped::scope(|scope| {
                scope.spawn(async {
                    let pool = Pool::from_pool_key(
                        chain_id,
                        factory_address,
                        token0.address,
                        token1.address,
                        *fee,
                        provider.clone(),
                        None,
                    )
                    .await;
                    if let Err(ref e) = pool {
                        eprintln!("Error: {:?}", e);
                    }
                    pools.insert(*fee, pool.ok());
                });
            })
        });
        Ok(Self {
            token0,
            token1,
            factory_address,
            pools,
        })
    }
}

impl Display for LiquidityPoolFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pools.iter().try_for_each(|(fee, pool)| {
            if let Some(pool) = pool {
                writeln!(
                    f,
                    "LiquidityPool {{ fee: {:?}, token0: {}, token1: {}, address: {}, liquidity: {} }}",
                    pool.fee,
                    pool.token0.symbol().unwrap(),
                    pool.token1.symbol().unwrap(),
                    pool.address(None, Some(self.factory_address)),
                    pool.liquidity,
                )?;
            } else {
                writeln!(f, "Fee: {:?} - Pool not found", fee)?;
            }
            std::fmt::Result::Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://rpc.immutable.com".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Construct a pool
    let factory_address = address!("56c2162254b0E4417288786eE402c2B41d4e181e");
    let eth = token!(1, "52A6c53869Ce09a731CD772f245b97A4401d3348", 18);
    let imx = token!(1, "3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D", 18);

    let factory = LiquidityPoolFactory::new(13371, factory_address, eth, imx, provider).await?;
    println!("{}", factory);

    Ok(())
}
