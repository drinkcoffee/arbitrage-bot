use alloy::{
    providers::RootProvider,
    transports::http::{Client, Http},
};

pub type Provider = RootProvider<Http<Client>>;
