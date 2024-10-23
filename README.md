# Arbitrage bot written in Rust

**This is currently work in progress. Please assume the code is full of bugs and will not work as advertised.**

## Proposed Features

On Uniswap V3 (& V4) on EVM chains:

* Determine UniswapV3Pools available for a token pair. That is:
  * Program knows the Uniswap Factory contract(s) on each chain.
  * User supplies the addresses of the ERC20 tokens.
  * Display the symbols returned by each ERC 20 contract
  * Checks with factory contract to see what pool contracts exist for the standard fee options.
  * Display pool contract addresses
  * Display amount of liquidity in each pool and exchange rate.
* Analyse arbitrage opportunity:
  * User supplies the UniswapV3Pool address on each chain.
  * Fetch token0 and token1 
  * Display the symbols returned by each ERC 20 contract
  * Display the fee for the pool.
  * Display the exchange rate for each pool and determine the liquidity depth
  * Calculate the arbitrage opportunity
* Analyse arbitrage opportunity 2:
  * User supplies the UniswapV3Pool address on each chain, the token symbols and the fees.
  * User supplies an estimated bridge fee.
  * Display the exchange rate for each pool and determine the liquidity depth
  * Calculate the arbitrage opportunity, allowing for bridging fee
  * Display instructions for how to execute the arbitrage.
* Cyclic arbitrage:
  * User supplies the UniswapV3Pool address on each chain, the token symbols and the fees.
  * User supplies a private key for each chain.
  * User indicates where they have value, in what token and how much.
  * Code executes cyclic arbitrage:
    * Assess opportunity: Exit if no opportunity
    * Swap on chain A
    * Crosschain transfer A -> B
    * Swap on chain B
    * Crosschain transfer B -> A
    * Repeat.


