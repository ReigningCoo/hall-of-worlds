//! A player grinding herbs and an oil into ointment (`Ability::Grind`,
//! brewing plan PR 6): the client sends a `GrindOrder` (`ClientMessage::Grind`),
//! kept here while it winds up for `brewing::grind_ticks`. When it lands the
//! order is checked again against what is carried (`Inventory::check_grind`):
//! `doses` of each herb and the bottle's oil are used up, the bottle comes
//! back empty, the ointment is given (none when the mix came to nothing, decision
//! 15), Alchemy and Herbalism are practised and the mix remembered
//! (`Sheet::recipes`). Moving or being harmed calls it off (`stop_collecting`).
//! `can_use` has already checked the mortar and pestle is carried.

use bevy::prelude::*;
use shared::ability::{ALCHEMY, Ability};
use shared::brewing::{
    BrewSkills, GRIND_PRACTICE, GrindOrder, HERB_PRACTICE, Recipe, grind_ticks, new_ointment,
};
use shared::herb::HERBALISM;
use shared::item::Item;
use shared::item_form::ItemForm;
use shared::potion::BottleForm;
use shared::protocol::{Landed, ServerMessage};
use shared::skill::SkillId;

use super::Client;
use super::action::Action;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// `ClientMessage::Grind`: the order kept for when it begins, and Grind asked
/// for like any ability, one at a time. Always stays.
pub(super) fn ask_grind(client: &mut Client, order: GrindOrder) -> bool {
    client.grinding = Some(order);
    let waiting = client.waiting.iter();
    if !waiting
        .into_iter()
        .any(|a| matches!(a, Action::UseAbility { .. }))
    {
        client.waiting.push_back(Action::UseAbility {
            ability: Ability::Grind,
            target: None,
        });
    }
    true
}

/// Grind asked for with the order kept (`grinding`): begins if what is carried
/// allows it. Whether the client is still there, and what it cost.
pub(super) fn start_grind(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let skills = BrewSkills::of(&client.sheet.skills);
    let allowed = (client.grinding.as_ref())
        .map(|order| client.inventory.check_grind(order, &world.herbs, skills));
    if let Some(Err(why)) = &allowed {
        debug!("Not grinding: {why:?}");
    }
    if !matches!(allowed, Some(Ok(_))) {
        client.grinding = None;
        return (true, 0);
    }
    let tick = world.action_clock.ticks();
    let lands_in = grind_ticks(skills);
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::Grind,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::Grind,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// The wind-up is over: the ointment made from what is still carried, or
/// nothing if that changed. `false` if the client has gone.
pub(super) fn land_grind(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let Some(order) = client.grinding.take() else {
        return true;
    };
    let skills = BrewSkills::of(&client.sheet.skills);
    let Ok((made, oil)) = client.inventory.check_grind(&order, &world.herbs, skills) else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    let herbs = use_up(client, &order);
    empty_the_oil(client, world, order.oil);
    let doses = match made.contents.is_empty() {
        true => 0,
        false => made.doses,
    };
    if doses > 0 {
        let ointment = new_ointment(made.contents.clone(), doses, new_id());
        give(client, world, ointment);
        let recipe = Recipe::new(herbs.iter().cloned(), oil, made.contents);
        client.sheet.recipes.remember(recipe);
    }
    practise(client, ALCHEMY, GRIND_PRACTICE, tick);
    let handled = HERB_PRACTICE * herbs.len() as u32;
    practise(client, HERBALISM, handled, tick);
    client.unsaved = true;
    client.update_speed(tick);
    client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Ground(doses),
        })
}

/// Takes `doses` of every herb in the order: the herbs' keys.
fn use_up(client: &mut Client, order: &GrindOrder) -> Vec<String> {
    let mut keys = Vec::new();
    for &stack in &order.herbs {
        if let Some(herb) = client.inventory.find(stack).and_then(|it| it.top().herb()) {
            keys.push(herb.herb.clone());
        }
        for _ in 0..order.doses {
            client.inventory.take_one(stack, &mut new_id);
        }
    }
    keys
}

/// Takes one bottle of the oil and gives it back empty, keeping its look.
fn empty_the_oil(client: &mut Client, world: &mut ServerWorld, stack: shared::id::Id) {
    let Some(mut bottle) = client.inventory.take_one(stack, &mut new_id) else {
        return;
    };
    let look = bottle.bottle().map_or(0, |bottle| bottle.look);
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(None, look)));
    give(client, world, bottle);
}

/// Into what is carried, or onto the ground if there is no room.
pub(super) fn give(client: &mut Client, world: &mut ServerWorld, item: Item) {
    if let Err(item) = client.inventory.give(item, world.stacks, &mut new_id) {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
}

/// Practice in the skill at `path`, through its practice window.
pub(super) fn practise(client: &mut Client, path: &str, amount: u32, tick: u64) {
    if let Some(skill) = SkillId::find(path) {
        let amount = client.learning.skills.take(skill, amount, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
}
