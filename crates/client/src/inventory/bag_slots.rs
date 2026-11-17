//! The belt's bag slots, under the hands in the Held column (owner, 2026-11-15):
//! as many show as the worn belt gives (`Inventory::bag_slot_count`), two to a
//! row, none without a belt. An empty bag that can go on a belt is dropped on
//! one to tie it (`TieBag`); its inside then adds to the end of the own grid.
//! Dragged off into a grid, it is untied (`MoveItem`), if the cells it adds are
//! empty.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Inventory, MoveRefused};

use super::hold::{Held, Target};
use super::{Carried, Holding};
use crate::dom::{Dom, SlotShown, SlotThing};

/// As many slots as the best belt gives (Artifact): the markup has this many.
pub(super) const MOST_SLOTS: usize = 6;

/// The picture in an empty slot, saying a bag goes there.
const EMPTY_ICON: &str = "icons/lorc/swap-bag.png";

/// What the empty slot's (white) icon is drawn in: the bin's ink, faint.
const EMPTY_TINT: Color = Color::srgba(0.24, 0.16, 0.08, 0.55);

/// The picture in an empty slot, loaded once.
#[derive(Resource)]
pub(super) struct EmptyIcon(Handle<Image>);

impl FromWorld for EmptyIcon {
    fn from_world(world: &mut World) -> Self {
        Self(world.resource::<AssetServer>().load(EMPTY_ICON))
    }
}

/// On a bag slot (counting from 0) and what is drawn in it: dropping here ties
/// a bag to the belt.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BagSlotNode(pub u8);

/// The slot named so in the clothes' markup (`bag0` to `bag5`).
pub(super) fn bag_slot_named(name: &str) -> Option<u8> {
    let slot: u8 = name.strip_prefix("bag")?.parse().ok()?;
    ((slot as usize) < MOST_SLOTS).then_some(slot)
}

/// What a bag slot's node holds: the slot, drawing itself.
pub(super) fn bag_slot(slot: u8) -> impl Bundle {
    (BagSlotNode(slot), SlotShown::default())
}

/// What dropping something carried on this slot would do: tie it there, if it
/// is an empty bag for a belt (`Inventory::check_tie`).
pub(super) fn tie_aim(carried: &Inventory, held: Held, slot: u8) -> Target {
    let allowed = carried.check_tie(held.id, slot).is_ok();
    Target::BagSlot { slot, allowed }
}

/// Why something held can't leave where it is, as far as bag slots go, when
/// letting go there would take it away: a tied bag whose cells are in use, or
/// the belt they are tied to (`Inventory::check_lift`).
pub(super) fn lift_refused(carried: &Inventory, id: Id, aim: Target) -> Option<MoveRefused> {
    let leaving = !matches!(
        aim,
        Target::Nowhere | Target::Body { .. } | Target::BagSlot { .. }
    );
    leaving.then(|| carried.check_lift(id).err()).flatten()
}

/// Shows as many slots as the belt gives, what is tied to each, and green or red
/// on the one something held is over.
pub(super) fn show_bag_slots(
    carried: Res<Carried>,
    holding: Res<Holding>,
    mut slots: Query<(&BagSlotNode, &mut SlotShown)>,
    new_slots: Query<(), Added<BagSlotNode>>,
    mut dom: Dom,
    mut shown_count: Local<Option<usize>>,
    empty_icon: Res<EmptyIcon>,
) {
    let inventory = &carried.0;
    let count = inventory.bag_slot_count().min(MOST_SLOTS);
    if *shown_count != Some(count) || !new_slots.is_empty() {
        *shown_count = Some(count);
        dom.set_flag("clothes/bags", "hidden", count == 0);
        for row in 0..MOST_SLOTS / 2 {
            dom.set_flag(&format!("clothes/bag_row{row}"), "hidden", row * 2 >= count);
        }
        for slot in 0..MOST_SLOTS {
            dom.set_flag(&format!("clothes/bag{slot}"), "hidden", slot >= count);
        }
    }
    let held = holding.held.map(|held| held.item.id);
    for (&BagSlotNode(slot), mut shown) in &mut slots {
        let thing = inventory
            .bag_slots
            .get(slot as usize)
            .filter(|bag| Some(bag.id) != held)
            .map(|bag| SlotThing {
                item: bag.clone(),
                count: 1,
                faded: false,
                pressable: true,
            });
        let aim = match holding.aim {
            Some(Target::BagSlot { slot: at, allowed }) if at == slot => Some(allowed),
            _ => None,
        };
        // (Only when it differs: the slot draws itself again when changed.)
        if shown.thing != thing {
            shown.thing = thing;
        }
        if shown.aim != aim {
            shown.aim = aim;
        }
        let icon = shown.thing.is_none().then(|| empty_icon.0.clone());
        if shown.icon != icon {
            shown.icon = icon;
            shown.icon_tint = EMPTY_TINT;
        }
    }
}

/// The words for a move refused by the bag slots' rules, and the like.
pub(super) fn explain_move(why: MoveRefused) -> String {
    match why {
        MoveRefused::NotForBelt => "Only a pouch or a bag can be tied to a belt.".into(),
        MoveRefused::NotEmpty => "Only an empty bag can be tied to a belt.".into(),
        MoveRefused::NoSlot => "Your belt has no slot for it.".into(),
        MoveRefused::BagInUse => "Empty the end of your inventory to untie this bag.".into(),
        MoveRefused::BeltInUse => "The bags tied to this belt need it.".into(),
        MoveRefused::NoRoom => "There is no room for it.".into(),
        MoveRefused::IntoItself => "It can't go inside itself.".into(),
        MoveRefused::TooBig => "It is too big to go in there.".into(),
        MoveRefused::TooHeavy => "It is too heavy to go in there.".into(),
        MoveRefused::TooDeep => "Too many bags are inside each other already.".into(),
        _ => "It can't go there.".into(),
    }
}

#[cfg(test)]
mod tests;
