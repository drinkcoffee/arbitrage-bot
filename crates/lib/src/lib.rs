use alloy::{
    providers::RootProvider,
    transports::http::{Client, Http},
};

pub type Transport = Http<Client>;
pub type Provider = RootProvider<Transport>;
