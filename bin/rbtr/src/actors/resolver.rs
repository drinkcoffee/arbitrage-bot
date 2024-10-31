use actix::prelude::*;

use super::{PriceDiff, Stop};

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

impl Handler<Stop> for Resolver {
    type Result = ();

    fn handle(&mut self, _: Stop, ctx: &mut Self::Context) {
        println!("Stopping Resolver");
        ctx.stop();
    }
}
