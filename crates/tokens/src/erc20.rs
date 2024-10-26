use alloy::{
    primitives::Address,
    providers::Provider,
    sol,
};

use eyre::Result;

use alloy::transports::Transport;


// ERC 20 contract specifying return value names
sol! {
    #[sol(rpc)]    
    interface IERC20 {
        function totalSupply() external view returns (uint256 supply);
        function balanceOf(address account) external view returns (uint256 balance);
        function transfer(address recipient, uint256 amount) external returns (bool success);
        function allowance(address owner, address spender) external view returns (uint256 allowance);
        function approve(address spender, uint256 amount) external returns (bool success);
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool success);
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);

        function symbol() external view returns (string sym);
    }

}

pub struct Erc20 {
    pub token_contract: IERC20,
}

impl Erc20 {
    pub async fn new<T, P>(
        token_address: Address,
        provider: P,
    ) -> Result<Self>
    where
        T: Transport + Clone,
        P: Provider<T> + Clone,
    {
        self.token_contract = IERC20::new(token_address, &provider);
    }

    pub async fn symbol(&self) -> Result<String> {
        let tok0_symbol = self.token_contract.symbol().call().await?.sym;
        Ok(tok0_symbol)
    }
}
