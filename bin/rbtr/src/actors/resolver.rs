use actix::prelude::*;

use super::{Control, PriceDiff};

/// Resolver is the actor responsible for handling all data and events
/// derived from on-chain state that other actors may provide.
pub struct Resolver {}

impl Resolver {
    pub fn new() -> Self {
        Resolver {}
    }
}

impl Actor for Resolver {
    type Context = Context<Self>;
}

impl Handler<PriceDiff> for Resolver {
    type Result = ();

    fn handle(&mut self, msg: PriceDiff, _: &mut Self::Context) -> Self::Result {
        println!("Price diff: {msg:?}");
    }
}

impl Handler<Control> for Resolver {
    type Result = ();

    fn handle(&mut self, control: Control, ctx: &mut Self::Context) {
        match control {
            Control::Stop => {
                println!("Stopping Resolver");
                ctx.stop();
            }
            _ => (),
        }
    }
}
