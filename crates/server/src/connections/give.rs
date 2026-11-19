//! Small helpers abilities share: an item given to the player (or put down
//! when there is no room), and practice through the practice window.

use shared::item::Item;
use shared::skill::SkillId;

use super::Client;
use crate::ServerWorld;
use crate::ids::new_id;

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
