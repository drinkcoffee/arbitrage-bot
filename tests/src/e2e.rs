use ethers::utils::Anvil;

#[tokio::test]
async fn end_to_end() {
    let port = 8545u16;
    let url = format!("http://localhost:{}", port).to_string();

    let anvil = Anvil::new()
    .port(port)
    .mnemonic("abstract vacuum mammal awkward pudding scene penalty purchase dinner depart evoke puzzle")
    .spawn();

    drop(anvil); // this will kill the instance
}
