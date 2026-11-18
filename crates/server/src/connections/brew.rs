//! A player brewing a potion (`Ability::Brew`, brewing plan PR 7): the client
//! sends a `BrewOrder` (`ClientMessage::Brew`: an ointment and a bottle of
//! water), kept here while it winds up for `brewing::brew_ticks`. When it lands
//! the order is checked again against what is carried (`Inventory::check_brew`):
//! one ointment is used up and the bottle of water comes back holding the
//! potion, or empty when none forms (decision 15). Alchemy and Herbalism are
//! practised either way; a potion teaches its herbs' effects on its stats
//! (`brewing::learn_from_potion`), and the mix remembers what it brewed into.
//! Moving or being harmed calls it off (`stop_collecting`). `can_use` has
//! already checked the cauldron is carried.

use bevy::prelude::*;
use shared::ability::{ALCHEMY, Ability};
use shared::brewing::{
    BREW_HERB_PRACTICE, BREW_PRACTICE, BrewOrder, BrewSkills, BrewedWith, brew_ticks, brewed_into,
    learn_from_potion,
};
use shared::herb::HERBALISM;
use shared::protocol::{Landed, ServerMessage};

use super::Client;
use super::action::Action;
use super::grind::{give, practise};
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// `ClientMessage::Brew`: the order kept for when it begins, and Brew asked
/// for like any ability, one at a time. Always stays.
pub(super) fn ask_brew(client: &mut Client, order: BrewOrder) -> bool {
    client.brewing = Some(order);
    let waiting = client.waiting.iter();
    if !waiting
        .into_iter()
        .any(|a| matches!(a, Action::UseAbility { .. }))
    {
        client.waiting.push_back(Action::UseAbility {
            ability: Ability::Brew,
            target: None,
        });
    }
    true
}

/// Brew asked for with the order kept (`brewing`): begins if what is carried
/// allows it. Whether the client is still there, and what it cost.
pub(super) fn start_brew(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let allowed = (client.brewing.as_ref()).map(|order| client.inventory.check_brew(order));
    if let Some(Err(why)) = &allowed {
        debug!("Not brewing: {why:?}");
    }
    if !matches!(allowed, Some(Ok(_))) {
        client.brewing = None;
        return (true, 0);
    }
    let tick = world.action_clock.ticks();
    let lands_in = brew_ticks(BrewSkills::of(&client.sheet.skills));
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::Brew,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::Brew,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// The wind-up is over: the potion brewed from what is still carried, or
/// nothing if that changed. `false` if the client has gone.
pub(super) fn land_brew(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let Some(order) = client.brewing.take() else {
        return true;
    };
    let Ok(made) = client.inventory.check_brew(&order) else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let ointment = client.inventory.take_one(order.ointment, &mut new_id);
    let bottle = client.inventory.take_one(order.water, &mut new_id);
    let (Some(_), Some(bottle)) = (ointment, bottle) else {
        return true;
    };
    let potion = brewed_into(bottle, made.potion.clone());
    let quality = potion.quality;
    give(client, world, potion);
    if let Some(brewed) = &made.potion {
        let sheet = &mut client.sheet;
        learn_from_potion(
            &mut sheet.herbs,
            &sheet.recipes,
            &world.herbs,
            &made.ointment,
            brewed,
        );
    }
    let brewed = BrewedWith {
        water: made.water,
        potion: made.potion,
    };
    client.sheet.recipes.brewed(&made.ointment, brewed);
    practise(client, ALCHEMY, BREW_PRACTICE, tick);
    practise(client, HERBALISM, BREW_HERB_PRACTICE, tick);
    client.unsaved = true;
    client.update_speed(tick);
    client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Brewed(quality),
        })
}
