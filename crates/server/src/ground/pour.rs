//! Containers lying in reach, for pouring and drinking (`connections/pour.rs`):
//! the whole stack as it is, one put back changed, or the top one taken.

use shared::id::Id;
use shared::item::Item;

use super::bags::{leave_on_ground, lying};
use crate::ServerWorld;
use crate::connections::Client;
use crate::ids::new_id;

/// The stack `item` heads, if it lies in the player's reach.
pub(crate) fn lying_stack(
    client: &Client,
    world: &mut ServerWorld,
    item: Id,
) -> Result<Item, &'static str> {
    lying(client, world, item)
}

/// Puts `item` in place of the one with its id lying in reach.
pub(crate) fn replace_lying(client: &Client, world: &mut ServerWorld, item: Item) {
    let id = item.id;
    leave_on_ground(client, world, id, Some(item));
}

/// Takes the top one of the stack `item` lying in reach, the rest staying.
pub(crate) fn take_top_lying(
    client: &Client,
    world: &mut ServerWorld,
    item: Id,
) -> Result<Item, &'static str> {
    let mut found = lying(client, world, item)?;
    let top = found.take_top(&mut new_id).ok_or("it is alone")?;
    leave_on_ground(client, world, item, Some(found));
    Ok(top)
}
