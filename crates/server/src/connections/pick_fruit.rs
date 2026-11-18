//! Picking a fruit plant (`ClientMessage::Pick` on a fruit plant's cell), in
//! turn with the player's steps: one beside or under them, ripe
//! (`FruitPlant::can_pick`), gives its harvest (its tier, with Foraging and
//! Orchards: `fruit::harvest`; big fruits whole) into their inventory, or onto
//! the ground under them if it doesn't fit; practises Orchards, and keeps when
//! it was picked so it grows back from its leaves (`sites::keep_harvest`).

use bevy::prelude::*;
use shared::fruit::{Fruits, ORCHARDS, harvest, new_item};
use shared::herb::{FORAGING, skill_value};
use shared::protocol::{PickNews, ServerMessage};
use shared::rng::Seed;
use shared::skill::SkillId;

use super::Client;
use super::pick_herb::{PICK_PRACTICE, pick_cost, pick_herb};
use crate::ServerWorld;
use crate::ids::new_id;
use crate::sites::{keep_harvest, update_sight};

/// Harvests the plant at `(x, y)`, a herb patch, fruit plant or vegetable. Whether the
/// client is still there, and what it cost.
pub(super) fn pick(client: &mut Client, world: &mut ServerWorld, x: i64, y: i64) -> (bool, u32) {
    let visit = client.visit.as_ref();
    if visit.is_some_and(|visit| visit.fruit_at(x, y).is_some()) {
        pick_fruit(client, world, x, y)
    } else if visit.is_some_and(|visit| visit.vegetable_at(x, y).is_some()) {
        super::pick_vegetable::pick_vegetable(client, world, x, y)
    } else {
        pick_herb(client, world, x, y)
    }
}

/// Picks the fruit plant at `(x, y)`. Whether the client is still there, and
/// what it cost: nothing if it was refused (`PickNews::Refused`).
fn pick_fruit(client: &mut Client, world: &mut ServerWorld, x: i64, y: i64) -> (bool, u32) {
    let now = world.minutes().max(0.0) as u64;
    let tick = world.action_clock.ticks();
    let refused = |client: &mut Client, why: &str| {
        info!("Refused picking fruit at ({x}, {y}): {why}");
        let refused = ServerMessage::Pick(PickNews::Refused { x, y });
        (client.link.send(&refused), 0)
    };
    let Some(visit) = &client.visit else {
        return refused(client, "not on a level");
    };
    let (px, py) = visit.position();
    if (x - px).abs().max((y - py).abs()) > 1 {
        return refused(client, "out of reach");
    }
    let Some(plant) = visit.fruit_at(x, y).cloned() else {
        return refused(client, "no fruit plant there");
    };
    let Some(def) = Fruits::built_in().get(&plant.fruit) else {
        return refused(client, "a fruit this server doesn't have");
    };
    if !plant.can_pick(def, now) {
        return refused(client, "not ripe");
    }
    let key = visit.plant_key(x, y);
    let skills = (
        skill_value(&client.sheet.skills, FORAGING),
        skill_value(&client.sheet.skills, ORCHARDS),
    );
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let count = harvest(def, skills, &mut rng);
    let item = new_item(def, count, new_id(), Some(world.made_now()));
    if let Err(item) = client.inventory.give(item, world.stacks, &mut new_id) {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
    if let Some(skill) = SkillId::find(ORCHARDS) {
        let amount = client.learning.skills.take(skill, PICK_PRACTICE, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
    client.unsaved = true;
    client.update_speed(tick);
    keep_harvest(world, key, now);
    if let Some(visit) = &mut client.visit {
        visit.plant_picked(x, y, now);
    }
    let picked = ServerMessage::Pick(PickNews::PickedFruit {
        x,
        y,
        fruit: def.key.clone(),
        count,
    });
    let told = client.link.send(&picked)
        && client.send_inventory()
        && client.send_sheet(tick)
        && update_sight(client);
    (told, pick_cost())
}
