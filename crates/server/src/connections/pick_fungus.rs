//! Picking a fungus patch (`ClientMessage::Pick` on a fungus's cell), in turn
//! with the player's steps: one beside or under them, full grown
//! (`FungusPatch::can_pick`), gives its harvest (as many as a herb patch by
//! Foraging, a large piece whole: `fungus::harvest`) into their inventory, or
//! onto the ground under them if it doesn't fit; may teach its name (a
//! Mycology check with Foraging, `fungus::names_on_picking`); practises
//! Foraging and Mycology, and keeps
//! when it was picked so it grows back as its `regrow` says
//! (`sites::keep_harvest`).

use bevy::prelude::*;
use shared::fungus::{Fungi, MYCOLOGY, harvest, names_on_picking, new_item};
use shared::herb::{FORAGING, skill_value};
use shared::protocol::{PickNews, ServerMessage};
use shared::rng::Seed;
use shared::skill::SkillId;

use super::Client;
use super::pick_herb::{PICK_PRACTICE, pick_cost};
use crate::ServerWorld;
use crate::ids::new_id;
use crate::sites::{keep_harvest, update_sight};

/// Picks the fungus patch at `(x, y)`. Whether the client is still there, and
/// what it cost: nothing if it was refused (`PickNews::Refused`).
pub(super) fn pick_fungus(
    client: &mut Client,
    world: &mut ServerWorld,
    x: i64,
    y: i64,
) -> (bool, u32) {
    let now = world.minutes().max(0.0) as u64;
    let tick = world.action_clock.ticks();
    let refused = |client: &mut Client, why: &str| {
        info!("Refused picking a fungus at ({x}, {y}): {why}");
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
    let Some(patch) = visit.fungus_at(x, y).cloned() else {
        return refused(client, "no fungus there");
    };
    let Some(def) = Fungi::built_in().get(&patch.fungus) else {
        return refused(client, "a fungus this server doesn't have");
    };
    if !patch.can_pick(def, now) {
        return refused(client, "not grown");
    }
    let key = visit.plant_key(x, y);
    let foraging = skill_value(&client.sheet.skills, FORAGING);
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let count = harvest(def, foraging, &mut rng);
    let item = new_item(def, count, new_id(), Some(world.made_now()));
    if let Err(item) = client.inventory.give(item, world.stacks, &mut new_id) {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
    let mycology = skill_value(&client.sheet.skills, MYCOLOGY);
    let named = !client.sheet.fungi.knows_name(&def.key)
        && names_on_picking(mycology, foraging, rng.range_u64(0, 100) as u32)
        && client.sheet.fungi.learn_name(&def.key);
    for path in [FORAGING, MYCOLOGY] {
        if let Some(skill) = SkillId::find(path) {
            let amount = client.learning.skills.take(skill, PICK_PRACTICE, tick);
            client.sheet.skills.practise(skill, amount);
            client.sheet.skills.used(skill, client.activity.played);
        }
    }
    client.unsaved = true;
    client.update_speed(tick);
    keep_harvest(world, key, now);
    if let Some(visit) = &mut client.visit {
        visit.plant_picked(x, y, now);
    }
    let picked = ServerMessage::Pick(PickNews::PickedFungus {
        x,
        y,
        fungus: def.key.clone(),
        count,
        named,
    });
    let told = client.link.send(&picked)
        && client.send_inventory()
        && client.send_sheet(tick)
        && update_sight(client);
    (told, pick_cost())
}
