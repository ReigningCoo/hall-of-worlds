//! Harvesting a herb patch (`ClientMessage::Pick`), in turn with the
//! player's steps: one beside or under them, grown enough (`HerbPatch::can_pick`),
//! gives its herb (how many by its stage, Foraging and Herbs; as good as it is
//! rare: `herb::harvest`)
//! into their inventory, or onto the ground under them if it doesn't fit;
//! practises Foraging, teaches the herb's name, and keeps when it was picked so
//! it grows back by its herb's rule (`sites::keep_harvest`).

use bevy::prelude::*;
use shared::door::door_cost;
use shared::herb::{FORAGING, HERBS, harvest, new_item, skill_value};
use shared::protocol::{PickNews, ServerMessage};
use shared::rng::Seed;
use shared::skill::SkillId;

use super::Client;
use crate::ServerWorld;
use crate::ids::new_id;
use crate::sites::{keep_harvest, update_sight};

/// Foraging practice for one harvest (as much as one Heal gives Healing).
pub(super) const PICK_PRACTICE: u32 = 8;

/// What a harvest costs in time: as long as two steps.
pub(super) fn pick_cost() -> u32 {
    2 * door_cost()
}

/// Harvests the patch at `(x, y)`. Whether the client is still there, and what
/// it cost: nothing if it was refused (`PickNews::Refused`).
pub(super) fn pick_herb(
    client: &mut Client,
    world: &mut ServerWorld,
    x: i64,
    y: i64,
) -> (bool, u32) {
    let now = world.minutes().max(0.0) as u64;
    let tick = world.action_clock.ticks();
    let refused = |client: &mut Client, why: &str| {
        info!("Refused picking a herb at ({x}, {y}): {why}");
        (
            client
                .link
                .send(&ServerMessage::Pick(PickNews::Refused { x, y })),
            0,
        )
    };
    let Some(visit) = &client.visit else {
        return refused(client, "not on a level");
    };
    let (px, py) = visit.position();
    if (x - px).abs().max((y - py).abs()) > 1 {
        return refused(client, "out of reach");
    }
    let Some(patch) = visit.herb_at(x, y).cloned() else {
        return refused(client, "no herb there");
    };
    let Some(def) = world.herbs.get(&patch.herb).cloned() else {
        return refused(client, "a herb this server doesn't have");
    };
    if !patch.can_pick(&def, now) {
        return refused(client, "not grown enough");
    }
    let key = visit.plant_key(x, y);
    let skills = (
        skill_value(&client.sheet.skills, FORAGING),
        skill_value(&client.sheet.skills, HERBS),
    );
    let mut rng = Seed::new(new_id().seed().value()).rng();
    let stage = patch.stage(&def, now);
    let Some(got) = harvest(&def, stage, skills, &mut rng) else {
        return refused(client, "nothing to take this early");
    };
    let item = new_item(&def, got.count, new_id(), Some(world.made_now()));
    if let Err(item) = client.inventory.give(item, world.stacks, &mut new_id) {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
    if let Some(skill) = SkillId::find(FORAGING) {
        let amount = client.learning.skills.take(skill, PICK_PRACTICE, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
    client.sheet.herbs.learn_name(&def.key);
    client.unsaved = true;
    client.update_speed(tick);
    keep_harvest(world, key, now);
    if let Some(visit) = &mut client.visit {
        visit.plant_picked(x, y, now);
    }
    let picked = ServerMessage::Pick(PickNews::Picked {
        x,
        y,
        herb: def.key.clone(),
        quality: got.quality,
        count: got.count,
    });
    let told = client.link.send(&picked)
        && client.send_inventory()
        && client.send_sheet(tick)
        && update_sight(client);
    (told, pick_cost())
}
