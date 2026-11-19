//! Taking something whole out of a bag in the player's reach, to use it up
//! (a log split into kindling, `connections/cut_kindling.rs`).

use shared::id::Id;
use shared::item::Item;

use super::bags::{lying, take_up};
use crate::ServerWorld;
use crate::connections::Client;

/// Takes `item` (all of its stack) out of its bag in the player's reach.
pub(crate) fn take_lying(
    client: &Client,
    world: &mut ServerWorld,
    item: Id,
) -> Result<Item, &'static str> {
    lying(client, world, item)?;
    take_up(client, world, item).ok_or("that isn't in reach")
}
