//! What a new character carries from the start besides clothes (owner,
//! 2026-11-22): a full lantern and two bottles of oil, as no one gives light
//! without a lit lantern (`shared::dungeon::CARRIED_LIGHT` is 0).

use shared::item::{Inventory, ItemKind, MadeAt, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::lantern::{FOUNT_ML, LanternForm};
use shared::potion::{BottleForm, Liquid, Oil};

use super::welcome::made_for;

/// How many bottles of oil a new character starts with.
pub(super) const OIL_BOTTLES: u32 = 2;

/// Gives a new character a lantern with a full fount and `OIL_BOTTLES` bottles
/// of Neutral Oil, into the first room they have; anything without room is
/// left out (a new character's own grid always has room).
pub(super) fn give_lantern_and_oil(
    inventory: &mut Inventory,
    made_at: MadeAt,
    limits: StackLimits,
) {
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
    for item in kit {
        if let Err(item) = inventory.give(item, limits, &mut crate::ids::new_id) {
            bevy::log::warn!("No room for a new character's {:?}", item.kind);
        }
    }
}

#[cfg(test)]
mod tests;
