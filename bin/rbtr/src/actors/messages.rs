use actix::prelude::*;

/// Control is a message to control the operation of an Actor.
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum Control {
    Tick,
    Stop,
}

/// PriceDiff is just an example message for now.
#[derive(Debug, Message)]
#[rtype(result = "()")]
#[allow(dead_code)]
pub struct PriceDiff {
    pub diff: f64,
}

/// Message to subscribe to some other message.
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Subscribe<M: Message<Result = ()> + Send>(pub Recipient<M>);
