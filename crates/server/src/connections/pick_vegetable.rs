//! Picking a vegetable plant (`ClientMessage::Pick` on a vegetable's cell), in
//! turn with the player's steps: one beside or under them, full grown
//! (`VegetablePlant::can_pick`), gives its harvest (its tier, with Foraging and
//! Crops: `vegetable::harvest`) into their inventory, or onto the ground under
//! them if it doesn't fit; practises Crops, and keeps when it was picked so it
//! grows back as its `regrow` says (`sites::keep_harvest`).

use bevy::prelude::*;
use shared::herb::{FORAGING, skill_value};
use shared::protocol::{PickNews, ServerMessage};
use shared::rng::Seed;
use shared::skill::SkillId;
use shared::vegetable::{CROPS, Vegetables, harvest, new_item};

use super::Client;
use super::pick_herb::{PICK_PRACTICE, pick_cost};
use crate::ServerWorld;
use crate::ids::new_id;
use crate::sites::{keep_harvest, update_sight};

/// Picks the vegetable plant at `(x, y)`. Whether the client is still there,
/// and what it cost: nothing if it was refused (`PickNews::Refused`).
pub(super) fn pick_vegetable(
    client: &mut Client,
    world: &mut ServerWorld,
    x: i64,
    y: i64,
) -> (bool, u32) {
    let now = world.minutes().max(0.0) as u64;
    let tick = world.action_clock.ticks();
    let refused = |client: &mut Client, why: &str| {
        info!("Refused picking a vegetable at ({x}, {y}): {why}");
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
    let Some(plant) = visit.vegetable_at(x, y).cloned() else {
        return refused(client, "no vegetable there");
    };
    let Some(def) = Vegetables::built_in().get(&plant.vegetable) else {
        return refused(client, "a vegetable this server doesn't have");
    };
    if !plant.can_pick(def, now) {
        return refused(client, "not grown");
    }
    let key = visit.plant_key(x, y);
    let skills = (
        skill_value(&client.sheet.skills, FORAGING),
        skill_value(&client.sheet.skills, CROPS),
    );
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let count = harvest(def, skills, &mut rng);
    let item = new_item(def, count, new_id(), Some(world.made_now()));
    if let Err(item) = client.inventory.give(item, world.stacks, &mut new_id) {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
    if let Some(skill) = SkillId::find(CROPS) {
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
    let picked = ServerMessage::Pick(PickNews::PickedVegetable {
        x,
        y,
        vegetable: def.key.clone(),
        count,
    });
    let told = client.link.send(&picked)
        && client.send_inventory()
        && client.send_sheet(tick)
        && update_sight(client);
    (told, pick_cost())
}
