
// To determine address of Uniswap V3 pair, call getPool on: 
    // https://etherscan.io/address/0x1F98431c8aD98523631AE4a59f267346ea31F984#readContract
    // Tokens on Ethereum:
    // WETH: https://etherscan.io/token/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
    // IMX: https://etherscan.io/token/0xf57e7e7c23978c3caec3c3548e3d615c346e79ff
    // GOG: https://etherscan.io/token/0x9ab7bb7fdc60f4357ecfef43986818a2a3569c62
    // UniswapV3Pool: IMX - WETH - fee: 3000, https://etherscan.io/address/0x81fbBc40Cf075FD7De6AfCe1bc72EDA1BB0e13aa

    // Immutable zkEVM
    // UniswapV3Pool: IMX - WETH - fee: 3000, https://explorer.immutable.com/address/0xEE997F15Eaca3012E4825F1AeFE12136216CF3AF


RPC: 
Ethereum https://eth.llamarpc.com
Immutable https://rpc.immutable.com

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


