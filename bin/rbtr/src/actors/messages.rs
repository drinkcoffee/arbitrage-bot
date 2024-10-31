use actix::prelude::*;

/// Message to progress the Actor by a single tick.
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Tick();

/// Message to stop the Actor.
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Stop();

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
