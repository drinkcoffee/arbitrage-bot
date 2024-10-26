use std::{collections::HashMap, fmt::Display};


use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::Transport, 
};
use alloy_primitives::{address, Address, ChainId};
use eyre::Result;
use uniswap_sdk_core::{
    prelude::{BaseCurrencyCore, Token},
    token,
};
use uniswap_v3_sdk::prelude::{FeeAmount, Pool};
use uniswap_sdk_core::prelude::*;

use tokens::erc20::Erc20;
use std::result::Result::Ok;

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
                let token0_ratio = pool.token0_price().to_significant(5, Rounding::RoundHalfUp).unwrap();
                let token1_ratio = pool.token1_price().to_significant(5, Rounding::RoundHalfUp).unwrap();
                let liquidity = pool.liquidity;

                if liquidity != 0 {
                    writeln!(
                        f,
                        "LiquidityPool {{ fee: {:?}, token0: {}, token1: {}, address: {}, liquidity: {} price: {}, {}}}",
                        pool.fee,
                        pool.token0.symbol().unwrap(),
                        pool.token1.symbol().unwrap(),
                        pool.address(None, Some(self.factory_address)),
                        liquidity,
                        token0_ratio, 
                        token1_ratio
                    )?;
    
                }
                else {
                    writeln!(
                        f,
                        "LiquidityPool {{ fee: {:?}, token0: {}, token1: {}, address: {}, liquidity: {}}}",
                        pool.fee,
                        pool.token0.symbol().unwrap(),
                        pool.token1.symbol().unwrap(),
                        pool.address(None, Some(self.factory_address)),
                        0
                    )?;
    
                }
            } else {
                writeln!(f, "Fee: {:?} - Pool not found", fee)?;
            }
            std::fmt::Result::Ok(())
        })
    }
}



#[must_use]
async fn is_invalid_contract(url: &str, addr: Address) -> Result<bool> {
    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let code = provider.get_code_at(addr).await?;
    Ok(code.len() == 0)
}


#[tokio::main]
async fn main() -> Result<()> {
    // Input that will be supplied - maybe via environment.
    let url = "https://rpc.immutable.com";
    // TODO when we use environment variables, the addresses will be strings. 
    // address! only works with string literals.
    let chain_id = 13371;
    let factory_address = address!("56c2162254b0E4417288786eE402c2B41d4e181e");
    let tok0_address = address!("52A6c53869Ce09a731CD772f245b97A4401d3348");
    let tok0_decimals = 18;
    let tok1_address = address!("3A0C2Ba54D6CBd3121F01b96dFd20e99D1696C9D");
    let tok1_decimals = 18;

    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    if is_invalid_contract(url, factory_address).await? {
        panic!("UniswapV3Factory address is not a valid contract: {}", factory_address);
    }
    if is_invalid_contract(url, tok0_address).await? {
        panic!("Token 0 address is not a valid contract: {}", tok0_address);
    }
    if is_invalid_contract(url, tok1_address).await? {
        panic!("Token 1 address is not a valid contract: {}", tok1_address);
    }


    let tok0_contract = Erc20::new(tok0_address, &provider).await?;
    let tok0_symbol_result = tok0_contract.symbol().await;
    let tok0_symbol = match tok0_symbol_result {
        Ok(res) => res,
        Err(error) => panic!("Problem fetching Token 0 ERC20 symbol: {error:?}"),
    };

    let tok1_contract = Erc20::new(tok1_address, &provider).await?;
    let tok1_symbol_result = tok1_contract.symbol().await;
    let tok1_symbol = match tok1_symbol_result {
        Ok(res) => res,
        Err(error) => panic!("Problem fetching Token 1 ERC20 symbol: {error:?}"),
    };


    // Construct a pool
    let tok0 = token!(chain_id, tok0_address, tok0_decimals);
    let tok1 = token!(chain_id, tok1_address, tok1_decimals);

    println!("Liquidity Pools Available");
    println!(" RPC Provider: {url}");
    println!(" UniswapV3Factory contract: {factory_address}");
    println!(" Token0: {}, {}", tok0_symbol, tok0_address);
    println!(" Token1: {}, {}", tok1_symbol, tok1.address);

    let factory = LiquidityPoolFactory::new(chain_id, factory_address, tok0, tok1, provider).await?;
    println!("{}", factory);

    Ok(())
}


