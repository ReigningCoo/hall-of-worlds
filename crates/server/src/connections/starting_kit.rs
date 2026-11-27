//! What a new character carries from the start besides clothes (owner,
//! 2026-11-22): a full lantern and two bottles of oil, as no one gives light
//! without a lit lantern (`shared::dungeon::CARRIED_LIGHT` is 0); and a gold
//! coin to trade with, for testing (trading plan; to be lowered to a real
//! starting amount later).

use shared::item::{Inventory, ItemKind, MadeAt, Quality, StackLimits, coin_items};
use shared::item_form::ItemForm;
use shared::lantern::{FOUNT_ML, LanternForm};
use shared::potion::{BottleForm, Liquid, Oil};

use super::welcome::made_for;

/// How many bottles of oil a new character starts with.
pub(super) const OIL_BOTTLES: u32 = 2;

/// The coins a new character starts with, in copper: one gold coin.
pub(super) const STARTING_COPPER: u64 = 10_000;

/// Gives a new character a lantern with a full fount, `OIL_BOTTLES` bottles of
/// Neutral Oil and `STARTING_COPPER` in coins, into the first room they have;
/// anything without room is left out (a new character's own grid always has
/// room).
pub(super) fn give_starting_kit(inventory: &mut Inventory, made_at: MadeAt, limits: StackLimits) {
    let mut lantern = made_for(
        inventory,
        ItemKind::Lantern,
        None,
        false,
        Quality::Common,
        made_at,
    );
    lantern.form = Some(ItemForm::Lantern(LanternForm {
        ml: FOUNT_ML,
        burnt: 0,
    }));
    let mut kit = vec![lantern];
    for _ in 0..OIL_BOTTLES {
        let mut bottle = made_for(
            inventory,
            ItemKind::Bottle,
            None,
            false,
            Quality::Common,
            made_at,
        );
        let look = match &bottle.form {
            Some(ItemForm::Bottle(form)) => form.look,
            _ => 0,
        };
        let oil = Some(Liquid::Oil(Oil::Neutral));
        bottle.form = Some(ItemForm::Bottle(BottleForm::new(oil, look)));
        kit.push(bottle);
    }
    kit.extend(coin_items(
        STARTING_COPPER,
        limits.most,
        Some(made_at),
        &mut crate::ids::new_id,
    ));
    for item in kit {
        if let Err(item) = inventory.give(item, limits, &mut crate::ids::new_id) {
            bevy::log::warn!("No room for a new character's {:?}", item.kind);
        }
    }
}

#[cfg(test)]
mod tests;
