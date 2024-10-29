pub use prelude::{Provider, RootProvider, Transport};

mod prelude {
    use alloy::{
        providers::RootProvider as AlloyRootProvider,
        transports::http::{Client, Http},
    };

    pub type Transport = Http<Client>;
    pub type RootProvider = AlloyRootProvider<Transport>;
    pub use alloy::providers::Provider;
}
