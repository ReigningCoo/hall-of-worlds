//! Eating (`ClientMessage::Consume`): the top one of food, a fruit, a vegetable
//! or a herb carried, or lying in reach, is taken and eaten (`Sheet::eat`, a
//! fruit: `eat_fruit.rs`, a vegetable: `eat_vegetable.rs`, or a herb raw:
//! `eat_herb.rs`), in turn with the player's steps. Ointment is rubbed on
//! the same way (`rub_ointment.rs`), a potion drunk (`drink.rs`), and water
//! from a vessel, which stays (`drink_vessel.rs`).

use bevy::prelude::*;
use shared::bonus::{may_use_on_self, usable_on_self};
use shared::energy::handling_cost;
use shared::id::Id;
use shared::item::Item;
use shared::potion::POTION_GRAMS;

use super::Client;
use super::eat_fruit::servings_fit;
use super::strength_room::eating_allowed;
use crate::ServerWorld;
use crate::ids::new_id;

/// Eats one of `item`, carried or on the ground. Whether the client is still
/// there, and what it cost (handling it), nothing if it was refused.
pub(super) fn consume(client: &mut Client, world: &mut ServerWorld, item: Id) -> (bool, u32) {
    if let Some(drunk) = super::drink_vessel::drink_vessel(client, world, item) {
        return drunk;
    }
    let tick = world.action_clock.ticks();
    let carried = client.inventory.find(item).is_some();
    let food = match carried {
        true => client.inventory.find(item).map(|found| found.top().clone()),
        false => crate::ground::food_lying(client, world, item),
    };
    if let Some(potion) = food.as_ref().filter(|food| food.is_potion())
        && !super::drink::may_drink(client, potion, tick)
    {
        return (client.send_sheet(tick) && client.send_inventory(), 0);
    }
    let allowed = food
        .as_ref()
        .is_none_or(|food| eating_allowed(client, world, (item, food), tick));
    let room = !carried
        || food
            .as_ref()
            .is_none_or(|food| servings_fit(client, world, item, food));
    let taken = match (allowed && room, carried) {
        (false, _) if !room => Err("its servings have nowhere to go"),
        (false, _) => Err("its Strength is needed"),
        (true, true) => take_carried(client, item),
        (true, false) => crate::ground::take_food(client, world, item),
    };
    let cost = match taken {
        Ok(eaten) if eaten.herb().is_some() => {
            super::eat_herb::eat_herb(client, world, &eaten, tick);
            handling_cost(eaten.total_weight())
        }
        Ok(eaten) if eaten.fruit().is_some() => {
            super::eat_fruit::eat_fruit(client, world, (&eaten, carried), tick);
            handling_cost(eaten.total_weight())
        }
        Ok(drunk) if drunk.is_potion() => {
            super::drink::drink(client, world, drunk, tick);
            handling_cost(POTION_GRAMS)
        }
        Ok(used) if used.ointment().is_some() => {
            super::rub_ointment::rub_ointment(client, &used, tick);
            handling_cost(used.total_weight())
        }
        Ok(eaten) if eaten.vegetable().is_some() => {
            super::eat_vegetable::eat_vegetable(client, world, (&eaten, carried), tick);
            handling_cost(eaten.total_weight())
        }
        Ok(eaten) => {
            let eater = client.inventory.creature;
            let (sheet, situation) = client.sheet_and_situation();
            sheet.eat(eaten.kind, eater, situation, tick);
            client.unsaved = true;
            client.refresh_attributes(tick);
            handling_cost(eaten.total_weight())
        }
        Err(why) => {
            warn!("Refused eating {item:?}: {why}");
            0
        }
    };
    client.update_speed(tick);
    client.ground_told.again |= !carried;
    let told = client.send_inventory() && client.send_sheet(tick) && client.send_vitals(tick);
    (told, cost)
}

/// Takes the top one of a carried stack of food or herbs, to be eaten.
fn take_carried(client: &mut Client, item: Id) -> Result<Item, &'static str> {
    let found = client.inventory.find(item).ok_or("it isn't carried")?;
    if !usable_on_self(found.top()) {
        return Err("it can't be eaten");
    }
    if !may_use_on_self(found.top(), client.inventory.creature) {
        return Err("it isn't for its kind");
    }
    if found.holds_quest_item() || client.inventory.is_worn(item) {
        return Err("it can't be eaten from there");
    }
    let taken = client.inventory.take_one(item, &mut new_id);
    taken.ok_or("it can't be taken from there")
}
