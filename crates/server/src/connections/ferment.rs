//! A player setting a cask fermenting (`Ability::Ferment`, drinks plan PR 6):
//! the client sends a `FermentOrder` (`ClientMessage::Ferment`), kept here
//! while it winds up for `ferment::ferment_ticks`. When it lands the order is
//! checked again (`ferment::plan`, the client's own rule): the cask, carried
//! or lying in reach, is sealed holding the drink it will be, fermenting; the
//! fruit or grain added is used up; the yeasts are marked used, not used up;
//! Fermenting is practised. Moving or being harmed calls it off
//! (`stop_collecting`).

use bevy::prelude::*;
use shared::ability::Ability;
use shared::ferment::{
    FERMENT_PRACTICE, FERMENTING, FermentOrder, FermentPlan, ferment_ticks, plan,
};
use shared::item::Item;
use shared::item_form::ItemForm;
use shared::protocol::{Landed, ServerMessage};

use super::Client;
use super::action::Action;
use super::grind::practise;
use super::pour::{Side, put_back, side};
use crate::ServerWorld;
use crate::combat::{Act, Pending};

/// `ClientMessage::Ferment`: the order kept for when it begins, and Ferment
/// asked for like any ability, one at a time. Always stays.
pub(super) fn ask_ferment(client: &mut Client, order: FermentOrder) -> bool {
    client.fermenting = Some(order);
    let asked = (client.waiting.iter()).any(|a| matches!(a, Action::UseAbility { .. }));
    if !asked {
        client.waiting.push_back(Action::UseAbility {
            ability: Ability::Ferment,
            target: None,
        });
    }
    true
}

/// What the order would do with what is carried and lying in reach now: the
/// cask's side, the fruit or grain added and the plan.
fn checked(
    client: &Client,
    world: &mut ServerWorld,
    order: &FermentOrder,
) -> Result<(Side, FermentPlan), String> {
    let cask = side(client, world, order.cask)?;
    if cask.stack.count() != 1 {
        return Err("a stacked cask".into());
    }
    let add = match order.add {
        Some(id) if client.inventory.is_worn(id) => return Err("added from what is worn".into()),
        Some(id) => Some(client.inventory.find(id).ok_or("nothing to add")?.clone()),
        None => None,
    };
    let yeasts = client
        .inventory
        .usable_yeasts(order.yeast, order.yeasts.max(1));
    let plan = plan(cask.stack.top(), add.as_ref(), &yeasts).map_err(|why| why.words())?;
    Ok((cask, plan))
}

/// Ferment asked for with the order kept (`fermenting`): begins if it would
/// work. Whether the client is still there, and what it cost.
pub(super) fn start_ferment(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let allowed = (client.fermenting.clone()).map(|order| checked(client, world, &order));
    if let Some(Err(why)) = &allowed {
        debug!("Not fermenting: {why}");
    }
    if !matches!(allowed, Some(Ok(_))) {
        client.fermenting = None;
        return (true, 0);
    }
    let tick = world.action_clock.ticks();
    let lands_in = ferment_ticks();
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::Ferment,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::Ferment,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// The wind-up is over: the cask set fermenting, or nothing if what it needs
/// has changed. `false` if the client has gone.
pub(super) fn land_ferment(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let Some(order) = client.fermenting.take() else {
        return true;
    };
    let (cask, plan) = match checked(client, world, &order) {
        Ok(found) => found,
        Err(why) => {
            debug!("Ferment fizzled: {why}");
            return client.link.send(&ServerMessage::AbilityLanded {
                who: None,
                target: None,
                landed: Landed::Fizzled,
            });
        }
    };
    if let Some(add) = order.add {
        client.inventory.take_added(add);
    }
    client
        .inventory
        .use_yeasts(order.yeast, order.yeasts.max(1));
    let form = cask.stack.top().cask().map(|form| plan.started(form));
    let started = Item {
        form: form.map(ItemForm::Cask),
        ..cask.stack.top().clone()
    };
    put_back(client, world, &cask, started);
    practise(client, FERMENTING, FERMENT_PRACTICE, tick);
    client.unsaved = true;
    client.ground_told.again = true;
    client.update_speed(tick);
    let days = shared::ferment::days_left(plan.secs) as u32;
    client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fermented { days },
        })
}
