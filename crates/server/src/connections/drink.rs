//! Drinking a potion (through `consume.rs`, brewing decisions 17 to 21): its
//! Health and Mana at once, each attribute's dose an effect of its own lasting
//! by the bottle's quality (`brewing::drunk`); refused while a potion of the
//! same stats and signs is cooling down; the bottle back empty, into what is
//! carried or onto the ground.

use bevy::prelude::*;
use shared::brewing::{Combination, drunk};
use shared::item::{Item, Quality};
use shared::item_form::ItemForm;
use shared::potion::BottleForm;
use shared::vitals::{Points, Vital};

use super::Client;
use super::give::give;
use crate::ServerWorld;

/// Whether `potion` can be drunk at `now`: not while one of the same stats and
/// signs is cooling down (said in the log if so).
pub(super) fn may_drink(client: &Client, potion: &Item, now: u64) -> bool {
    let Some(bottle) = potion.bottle() else {
        return false;
    };
    let ready = client
        .sheet
        .drinks
        .ready(Combination::of(bottle.contents()), now);
    if !ready {
        warn!(
            "{} refused drinking {:?}: still cooling down",
            client.in_log(),
            potion.id
        );
    }
    ready
}

/// Drinks `potion`, one bottle already taken from where it was, at `now`, and
/// gives its bottle back empty.
pub(super) fn drink(client: &mut Client, world: &mut ServerWorld, mut potion: Item, now: u64) {
    let Some(bottle) = potion.bottle().cloned() else {
        return;
    };
    let contents = bottle.contents();
    let quality = potion.quality.unwrap_or(Quality::Common);
    let done = drunk(contents, quality, now);
    let limits = client.limits(now);
    let played = client.activity.played;
    let (sheet, situation) = client.sheet_and_situation();
    sheet.drinks.drank(Combination::of(contents), now);
    for effect in done.effects {
        sheet.add_effect(effect, situation, now);
    }
    for (vital, amount) in done.vitals {
        let size = Points(amount.unsigned_abs());
        match (vital, amount > 0) {
            (_, true) => sheet.vitals.restore(vital, size, limits, now),
            (Vital::Health, false) => sheet.vitals.hurt(size, limits, now, played),
            (Vital::Mana, false) => {
                sheet.vitals.spend_mana(size, limits, now, played);
            }
        }
    }
    client.unsaved = true;
    client.refresh_attributes(now);
    potion.form = Some(ItemForm::Bottle(BottleForm::new(None, bottle.look)));
    potion.quality = None;
    give(client, world, potion);
}
