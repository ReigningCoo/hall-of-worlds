//! Drinking from a vessel (through `consume.rs`, drinks plan PRs 2 and 3): a
//! serving (`vessel::SERVING_ML`) from a tankard, cup or any vessel, carried
//! or lying in reach; water does nothing, juice and cordial restore Mana
//! (`juice::drink_effect`), and a fermented drink twice as much, with its
//! units of alcohol (`alcohol.rs`). The vessel stays where it is. A bottle of a drink
//! is drunk whole and given back empty (`drink_bottle`).

use bevy::prelude::*;
use shared::energy::handling_cost;
use shared::id::Id;
use shared::item::Item;
use shared::item_form::ItemForm;
use shared::juice::drink_effect;
use shared::potion::{BottleForm, Liquid};
use shared::pour::BOTTLE_ML;

use super::Client;
use super::give::give;
use super::pour::{put_back, side};
use crate::ServerWorld;

/// Drinks from `item` if it is a vessel, carried or in reach: whether the
/// client is still there and what it cost. `None` if it isn't a vessel.
pub(super) fn drink_vessel(
    client: &mut Client,
    world: &mut ServerWorld,
    item: Id,
) -> Option<(bool, u32)> {
    let side = side(client, world, item).ok()?;
    let vessel = side.stack.top().vessel()?.clone();
    let liquid = vessel.liquid.clone();
    let (after, ml) = vessel.drunk();
    if ml == 0 {
        warn!(
            "{} refused drinking from {item:?}: not water",
            client.in_log()
        );
    } else {
        let changed = Item {
            form: Some(ItemForm::Vessel(after)),
            ..side.stack.top().clone()
        };
        put_back(client, world, &side, changed);
        if let Some(liquid) = liquid {
            drink_effect_of(client, &liquid, ml, world.action_clock.ticks());
        }
        client.unsaved = true;
    }
    client.ground_told.again = true;
    Some((client.send_inventory(), handling_cost(ml)))
}

/// What drinking `ml` of `liquid` does to the player now, if anything.
fn drink_effect_of(client: &mut Client, liquid: &Liquid, ml: u32, now: u64) {
    client.drink_units(liquid, ml, now);
    let Some(effect) = drink_effect(liquid, ml, now) else {
        return;
    };
    let (sheet, situation) = client.sheet_and_situation();
    sheet.add_effect(effect, situation, now);
    client.refresh_attributes(now);
}

/// Drinks a bottle of water, juice or cordial (`Item::is_drinkable_bottle`),
/// already taken from where it was, at `now`: its effect, and the bottle back
/// empty, into what is carried or onto the ground.
pub(super) fn drink_bottle(
    client: &mut Client,
    world: &mut ServerWorld,
    mut bottle: Item,
    now: u64,
) {
    let Some(form) = bottle.bottle().cloned() else {
        return;
    };
    if let Some(liquid) = &form.liquid {
        drink_effect_of(client, liquid, BOTTLE_ML, now);
    }
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(None, form.look)));
    bottle.quality = None;
    give(client, world, bottle);
    client.unsaved = true;
}
