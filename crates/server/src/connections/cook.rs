//! A player cooking (`Ability::Cook`, cooking plan PR 9), as Alchemy is run
//! (`alchemy.rs`): the client sends a `CookOrder` (`ClientMessage::Cook`),
//! kept here while each run winds up for its steps' times
//! (`cooking::run_ticks`). When a run lands the order is checked again
//! against what is carried and the kitchen (`cook_kitchen.rs`,
//! `Inventory::check_cooking`), the ingredients are used up (whole bottles
//! given back empty, vessels and casks drawn from, intermediates from what is
//! carried or the hearth's storage), fungi handled harm as picking them
//! would, the chance and luck are rolled from a fresh seed
//! (`cooking::outcome`), and each output is given as a Dish or an
//! intermediate (`food::food_item`) with the culture put in if one ferments
//! (it is used up then). Each step's sub-skill is practised either way; a lit
//! hearth leaves Ash for the heat used (`crafting_bench::hearth_ash`). Then
//! the next run begins, until `count` runs are done or the ingredients run
//! out. Moving or being harmed calls it off, repeats and all
//! (`stop_collecting`). A run that works is remembered in the recipe book as
//! an own recipe (`Inventory::cooking_recipe`, read before anything is used
//! up; named as its last output was classified).

use bevy::prelude::*;
use shared::ability::Ability;
use shared::alchemy::CERTAIN;
use shared::cooking::{
    COOK_PRACTICE, CookOrder, CookingSkills, Dishes, Ingredients, Outcome, heat_seconds, outcome,
    run_ticks,
};
use shared::crafting_bench::hearth_ash;
use shared::food::food_item;
use shared::fungus::Fungi;
use shared::item::{Item, ItemKind};
use shared::protocol::{Cooked, Landed, ServerMessage};
use shared::rng::Seed;

use super::Client;
use super::action::Action;
use super::cook_kitchen::{KitchenHere, kitchen_here};
use super::cook_use::use_up;
use super::fungus_harm::handle_fungus;
use super::give::{give, practise};
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// `ClientMessage::Cook`: the order kept for when it begins, and Cook asked
/// for like any ability, one at a time. Always stays.
pub(super) fn ask_cook(client: &mut Client, order: CookOrder) -> bool {
    client.cooking = Some(order);
    if !(client.waiting.iter()).any(|a| matches!(a, Action::UseAbility { .. })) {
        client.waiting.push_back(Action::UseAbility {
            ability: Ability::Cook,
            target: None,
        });
    }
    true
}

/// The kitchen and what one run of the kept order makes, or `None` if it
/// can't be run now.
fn checked(
    client: &Client,
    world: &mut ServerWorld,
    order: &CookOrder,
) -> Option<(KitchenHere, shared::cooking::Checked)> {
    let skills = CookingSkills::of(&client.sheet.skills);
    let Ok(here) = kitchen_here(client, world, order) else {
        debug!("Not cooking: not at the hearth asked for");
        return None;
    };
    let checked = client.inventory.check_cooking(
        order,
        here.kitchen(),
        Ingredients::built_in(),
        Dishes::built_in(),
        skills,
    );
    match checked {
        Ok(checked) => Some((here, checked)),
        Err(why) => {
            debug!("Not cooking: {why:?}");
            None
        }
    }
}

/// Cook asked for with the order kept (`cooking`): a run begins if what is
/// carried and the kitchen allow it. Whether the client is still there, and
/// what it cost.
pub(super) fn start_cook(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let Some(order) = client.cooking.clone() else {
        return (true, 0);
    };
    if checked(client, world, &order).is_none() {
        client.cooking = None;
        return (true, 0);
    }
    let skills = CookingSkills::of(&client.sheet.skills);
    let tick = world.action_clock.ticks();
    let lands_in = run_ticks(&order.steps, skills);
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::Cook,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::Cook,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// A run's wind-up is over: what it makes from what is still carried, or
/// nothing if that changed; the next run asked for if any are left. `false`
/// if the client has gone.
pub(super) fn land_cook(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let Some(mut order) = client.cooking.take() else {
        return true;
    };
    let Some((here, checked)) = checked(client, world, &order) else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    for key in &checked.fungi {
        if let Some(def) = Fungi::built_in().get(key) {
            handle_fungus(client, def, tick);
        }
    }
    // Read before the run takes anything, for the recipe book.
    let own = (client.inventory).cooking_recipe(&order, &here.storage, Ingredients::built_in());
    use_up(client, world, &order, &here);
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let chance_roll = rng.range_u64(0, u64::from(CERTAIN)) as u32;
    let luck_roll = rng.range_u64(0, u64::from(CERTAIN)) as u32;
    let planned = &checked.planned;
    let made = match outcome(planned, chance_roll, luck_roll) {
        Outcome::Failed => Cooked::Failed,
        Outcome::Made { qualities } => {
            if let (Some(recipe), Some(last)) = (own, planned.outputs.last()) {
                (client.sheet.book).remember_cooking(recipe, last.dish.name.clone());
            }
            let made_at = Some(world.made_now());
            let mut items: Vec<Item> = (planned.outputs.iter().zip(qualities))
                .map(|(output, quality)| {
                    let ingredients = Ingredients::built_in();
                    food_item(
                        output,
                        quality,
                        checked.culture,
                        ingredients,
                        new_id(),
                        made_at,
                    )
                })
                .collect();
            let cultured = |item: &Item| {
                let rest = item.food().and_then(|food| food.rest.as_ref());
                rest.is_some_and(|rest| rest.culture.is_some())
            };
            if let Some(culture) = order.culture.filter(|_| items.iter().any(cultured)) {
                client.inventory.take_one(culture, &mut new_id);
            }
            let made = first_made(&items);
            for item in items.drain(..) {
                give(client, world, item);
            }
            made
        }
    };
    let skills = CookingSkills::of(&client.sheet.skills);
    let ash = if here.lit_hearth() {
        hearth_ash(heat_seconds(&order.steps, skills))
    } else {
        0
    };
    if ash > 0 {
        let mut item = Item::new(new_id(), ItemKind::Ash);
        item.count = ash;
        give(client, world, item);
    }
    for tool in &planned.practised {
        practise(client, tool.sub_skill(), COOK_PRACTICE, tick);
    }
    order.count -= 1;
    let left = order.count;
    if left > 0 {
        ask_cook(client, order);
    }
    client.unsaved = true;
    client.update_speed(tick);
    client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Cooked { made, ash, left },
        })
}

/// What a run that worked tells the client: its first output's kind, count
/// and quality, and how many outputs there were.
fn first_made(items: &[Item]) -> Cooked {
    let Some(first) = items.first() else {
        return Cooked::Failed;
    };
    Cooked::Made {
        kind: first.kind,
        count: first.count,
        quality: first.quality.unwrap_or_default(),
        outputs: items.len() as u32,
    }
}
