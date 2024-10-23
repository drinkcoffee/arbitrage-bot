# Pools

* Determine UniswapV3Pools available for a token pair. That is:
  * Program knows the Uniswap Factory contract(s) on each chain.
  * User supplies the addresses of the ERC20 tokens.
  * Display the symbols returned by each ERC 20 contract
  * Checks with factory contract to see what pool contracts exist for the standard fee options.
  * Display pool contract addresses
  * Display amount of liquidity in each pool and exchange rate.
