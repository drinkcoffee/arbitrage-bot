use crate::{PriceDiff, Subscribe};
use actix::prelude::*;

use super::Control;

/// Monitor is the actor responsible for monitoring on-chain state
/// and notifying subscribers of changes.
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

impl Handler<Control> for Monitor {
    type Result = ();

    fn handle(&mut self, control: Control, ctx: &mut Self::Context) {
        match control {
            Control::Stop => {
                println!("Stopping Monitor");
                ctx.stop();
            }
            Control::Tick => {
                for sub in &self.price_diff_subs {
                    // TODO: handle error
                    sub.do_send(PriceDiff { diff: 0.1 });
                }
            }
        }
    }
}
