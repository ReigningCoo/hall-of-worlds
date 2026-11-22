//! Filling from the panel: a bottle of oil dragged from a grid (or the
//! ground in reach) onto the space under the lantern asks the server to pour
//! it in (`ClientMessage::FillLantern`); the bottle itself goes back where it
//! was, coming back empty when the server answers with `Inventory`. Green
//! over the space when it would go in (`shared::lantern::fill`), red if not.

use bevy::prelude::*;
use shared::item::Item;
use shared::lantern::fill;
use shared::protocol::ClientMessage;

use super::panel::{FillSlot, LanternPanel};
use crate::connection::ToServer;
use crate::dom::{Dropped, HeldObject, HeldOver, SlotShown};
use crate::inventory::Carried;

/// How far up from the node in front the space may be: its picture's parts.
const UP: usize = 3;

/// Tints the space while something is held over it, and fills on letting go.
pub(super) fn drop_oil(
    object: Res<HeldObject>,
    mut dropped: MessageReader<Dropped>,
    panel: Res<LanternPanel>,
    carried: Res<Carried>,
    parents: Query<&ChildOf>,
    mut slots: Query<(Entity, &mut SlotShown), With<FillSlot>>,
    mut to_server: MessageWriter<ToServer>,
) {
    let Some(open) = &panel.0 else {
        dropped.clear();
        return;
    };
    let lantern = carried.0.find(open.lantern);
    let on_space = |over: &HeldOver, space: Entity| {
        let mut at = over.front;
        for _ in 0..=UP {
            match at {
                Some(node) if node == space => return true,
                Some(node) => at = parents.get(node).ok().map(ChildOf::parent),
                None => return false,
            }
        }
        false
    };
    let fills = |bottle: &Item| lantern.is_some_and(|lantern| fill(lantern, bottle.top()).is_ok());
    for drop in dropped.read() {
        let Some(held) = drop.thing.item() else {
            continue;
        };
        let over = slots.iter().any(|(space, _)| on_space(&drop.over, space));
        if over && fills(&held.item) {
            to_server.write(ToServer(ClientMessage::FillLantern {
                lantern: open.lantern,
                bottle: held.id,
            }));
        }
    }
    for (space, mut slot) in &mut slots {
        let aim = (object.thing())
            .and_then(|thing| thing.item())
            .filter(|_| on_space(&object.over, space))
            .map(|held| fills(&held.item));
        if slot.aim != aim {
            slot.aim = aim;
        }
    }
}
