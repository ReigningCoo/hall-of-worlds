//! The receive box and the Trading setting, both saved with the character:
//! taking things out of the box (one to a place, or everything that fits),
//! setting how trades with players are taken, and telling both on joining.

use bevy::prelude::*;
use shared::protocol::TradeNews;
use shared::trade::{Place, TradingSetting};

use super::tell;
use crate::ServerWorld;
use crate::connections::{Client, Phase};
use crate::ids::new_id;

/// One thing taken out of the receive box to the place `to` says, or (none)
/// everything that fits. Answered with the box and the inventory. `false` if
/// the client has gone.
pub(super) fn take(client: &mut Client, world: &mut ServerWorld, to: Option<Place>) -> bool {
    let received = &mut client.trade.saved.received;
    let taken = match to {
        Some(to) => usize::from(received.take(&to, &mut client.inventory)),
        None => received.take_all(&mut client.inventory, world.stacks, &mut new_id),
    };
    if taken == 0 {
        debug!("{} took nothing from their receive box", client.in_log());
    } else {
        client.unsaved = true;
        client.update_speed(world.action_clock.ticks());
    }
    let received = client.trade.received().clone();
    client.send_inventory() && tell(client, TradeNews::Received(received))
}

/// How the character takes trades with players, answered with the setting.
/// Once playing only, so a character's saved one is never overwritten.
pub(in crate::connections) fn set_setting(client: &mut Client, setting: TradingSetting) -> bool {
    if client.phase != Phase::Playing {
        return true;
    }
    if client.trade.saved.setting != setting {
        client.trade.saved.setting = setting;
        client.unsaved = true;
    }
    tell(client, TradeNews::Setting(setting))
}

/// Tells a client joining its Trading setting and its receive box, unless
/// they are a new character's (Ask, empty), which the client starts with.
pub(in crate::connections) fn told_on_joining(client: &mut Client) -> bool {
    let (setting, received) = (client.trade.setting(), client.trade.received().clone());
    (setting == TradingSetting::default() || tell(client, TradeNews::Setting(setting)))
        && (received.is_empty() || tell(client, TradeNews::Received(received)))
}
