pub mod prelude {
    use alloy::{
        primitives::Address,
        providers::RootProvider as AlloyRootProvider,
        transports::http::{reqwest::Url, Client, Http},
    };

    pub type Transport = Http<Client>;
    pub type RootProvider = AlloyRootProvider<Transport>;
    pub use alloy::providers::Provider;
}
