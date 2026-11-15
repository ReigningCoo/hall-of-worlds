//! How a kind of item on the action bar looks (owner, 2026-11-15): its picture
//! in an ability's frame, with how many are carried; greyed, with a dull red
//! frame, when none are carried or the player's kind won't eat it (pressing it
//! then does nothing).

use bevy::prelude::*;
use shared::ability::BarThing;
use shared::bonus::may_eat;
use shared::creature::CreatureKind;
use shared::item::ItemKind;

use super::page::look;
use crate::widgets::{AbilityLook, GRID_COLOUR};

/// How a kind of item looks on the bar, with `count` carried.
pub(super) fn item_look(kind: ItemKind, count: u32, eater: CreatureKind) -> AbilityLook {
    AbilityLook {
        icon: Handle::default(),
        name: kind.def().name.to_string(),
        passive: false,
        usable: can_eat(kind, count, eater),
        colour: GRID_COLOUR,
        item: Some(kind),
    }
}

/// Whether pressing it eats one.
pub(super) fn can_eat(kind: ItemKind, count: u32, eater: CreatureKind) -> bool {
    count > 0 && may_eat(kind, eater)
}

/// How an ability or a kind of item looks, with `count` carried of an item.
pub(super) fn thing_look(
    thing: BarThing,
    count: u32,
    eater: CreatureKind,
    assets: &AssetServer,
) -> AbilityLook {
    match thing {
        BarThing::Ability(ability) => look(ability, eater, assets),
        BarThing::Item(kind) => item_look(kind, count, eater),
    }
}
