//! Eating from the ground: the top one of food or a herb in a bag in the
//! player's reach.

use shared::bonus::{edible, may_eat};
use shared::id::Id;
use shared::item::Item;

use super::bags::{leave_on_ground, lying};
use crate::ServerWorld;
use crate::connections::Client;
use crate::ids::new_id;

/// Takes the top one of this, if it is food or a herb lying in the player's reach, to be
/// eaten: the rest of its stack stays in its bag.
pub(crate) fn take_food(
    client: &Client,
    world: &mut ServerWorld,
    item: Id,
) -> Result<Item, &'static str> {
    let mut found = lying(client, world, item)?;
    if !edible(found.top().kind) {
        return Err("it can't be eaten");
    }
    if !may_eat(found.top().kind, client.inventory.creature) {
        return Err("it isn't for its kind");
    }
    let top = found.take_top(&mut new_id);
    let rest = top.is_some().then(|| found.clone());
    leave_on_ground(client, world, item, rest);
    Ok(top.unwrap_or(found))
}

/// The top one of this, if it lies in the player's reach: what eating it would eat.
pub(crate) fn food_lying(client: &Client, world: &mut ServerWorld, item: Id) -> Option<Item> {
    lying(client, world, item)
        .ok()
        .map(|found| found.top().clone())
}
