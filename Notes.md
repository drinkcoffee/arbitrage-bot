
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


abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

// Define the contract binding
abigen!(
    UniswapV3Pool,
    r#"[
        function token0() external view returns (address)
        function token1() external view returns (address)
    ]"#,
);
abigen!(
    Erc20,
    r#"[
        function symbol() external view returns (string)
    ]"#,
);