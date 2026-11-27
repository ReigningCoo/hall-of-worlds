//! Completing a trade: all of it at once (`shared::trade::complete`), then
//! what follows from it: the shop's new stock, standing in its town, and
//! Trading practice for a tip or a haggle (negative for a tip too big).

use bevy::prelude::*;
use shared::protocol::{ServerMessage, TradeEnd, TradeNews};
use shared::skill::SkillId;
use shared::trade::{Payment, ShopTrade, TRADE_PRACTICE, complete as complete_trade};

use super::{TRADING, end, tell, terms_rates, trading};
use crate::ServerWorld;
use crate::connections::Client;
use crate::ids::new_id;
use crate::sites::{ShopTerms, traded_with};

/// Completes the open trade, or tells why not, leaving it open. `false` if
/// the client has gone.
pub(super) fn complete(
    client: &mut Client,
    world: &mut ServerWorld,
    trade: ShopTrade,
    terms: &ShopTerms,
) -> bool {
    let made = world.made_now();
    let done = complete_trade(
        &trade,
        &terms.stock,
        &client.inventory,
        (terms_rates(client, terms), terms.kind),
        trading(client),
        world.stacks,
        Some(made),
        &mut new_id,
    );
    let done = match done {
        Ok(done) => done,
        Err(why) => {
            debug!("Refused completing a trade: {why:?}");
            return tell(client, TradeNews::Refused(why)) && tell(client, TradeNews::State(trade));
        }
    };
    client.inventory = done.inventory;
    client.unsaved = true;
    let things = (trade.buying.len() + trade.selling.len()) as u32;
    crate::quests::traded_in(client, terms.town, things);
    let sold = (trade.selling.as_slice(), made.minute);
    traded_with(world, terms, trade.with, done.stock.clone(), sold);
    let tick = world.action_clock.ticks();
    practise(client, done.payment, tick);
    if let Payment::Tip { copper, .. } = done.payment {
        crate::quests::tipped(client, terms.town, copper);
    }
    info!("{} traded, paying {}", client.in_log(), done.paid);
    client.update_speed(tick);
    let items = client.as_seen(done.stock);
    let why = TradeEnd::Completed {
        paid: done.paid,
        payment: done.payment,
    };
    end(client, trade.with, why)
        && client.send_inventory()
        && (client.link).send(&ServerMessage::TradeStock {
            with: trade.with,
            items,
        })
        && client.send_sheet(tick)
}

/// Trading practice for a modest tip or a haggle, and negative practice for
/// a tip past `TIP_PERCENT`.
fn practise(client: &mut Client, payment: Payment, tick: u64) {
    let Some(skill) = SkillId::find(TRADING) else {
        return;
    };
    match payment {
        Payment::Exact => {}
        Payment::Tip { generous: true, .. } => {
            client.sheet.skills.unpractise(skill, TRADE_PRACTICE);
        }
        Payment::Tip { .. } | Payment::Haggled { .. } => {
            let amount = client.learning.skills.take(skill, TRADE_PRACTICE, tick);
            client.sheet.skills.practise(skill, amount);
            client.sheet.skills.used(skill, client.activity.played);
        }
    }
}
