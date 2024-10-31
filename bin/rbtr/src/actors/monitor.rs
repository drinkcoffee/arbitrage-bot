use crate::{PriceDiff, Subscribe};
use actix::prelude::*;

use super::{Stop, Tick};

/// Monitor is the actor responsible for monitoring on-chain state
/// and notifying subscribers of changes.
///
/// It is driven by the `Tick` message.
pub struct Monitor {
    price_diff_subs: Vec<Recipient<PriceDiff>>,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            price_diff_subs: vec![],
        }
    }
}

impl Actor for Monitor {
    type Context = Context<Self>;
}

impl Handler<Subscribe<PriceDiff>> for Monitor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe<PriceDiff>, _: &mut Self::Context) {
        self.price_diff_subs.push(msg.0);
    }
}

impl Handler<Tick> for Monitor {
    type Result = ();

    fn handle(&mut self, _: Tick, _: &mut Self::Context) {
        for sub in &self.price_diff_subs {
            // TODO: handle error
            sub.do_send(PriceDiff { diff: 0.1 });
        }
    }
}

impl Handler<Stop> for Monitor {
    type Result = ();

    fn handle(&mut self, _: Stop, ctx: &mut Self::Context) {
        println!("Stopping Monitor");
        ctx.stop();
    }
}
