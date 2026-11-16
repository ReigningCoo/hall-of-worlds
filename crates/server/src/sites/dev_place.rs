//! Developer tools for Load in Debug Info (`connections::go_to_place`): into
//! the wilderness anywhere, and onto a spot of a site level or the nearest
//! with room.

use shared::spot::Spot;

use super::wild::{enter_wilderness, nearest_walkable};
use super::{leave, teleport};
use crate::ServerWorld;
use crate::connections::Client;

/// Puts the player in the wilderness at `at` (or the walkable cell nearest),
/// out of wherever they are.
pub(crate) fn dev_wilderness(client: &mut Client, world: &mut ServerWorld, at: Spot) -> bool {
    leave(client, world);
    client.visit = None;
    enter_wilderness(client, world, at)
}

/// On a site level (not the wilderness, which finds its own footing), puts the
/// player at `at`, or in the middle of the walkable cell nearest it if there is
/// no room there. Nothing if they are there already, or not on a level.
pub(crate) fn land_near(client: &mut Client, at: Spot) -> bool {
    let Some(visit) = &client.visit else {
        return true;
    };
    if visit.is_wilderness() || visit.position == at {
        return true;
    }
    let nearest = Spot::middle_of(nearest_walkable(&visit.level.level, at.cell()));
    if !teleport(client, at) {
        return false;
    }
    match &client.visit {
        Some(visit) if visit.position != at && visit.position != nearest => {
            teleport(client, nearest)
        }
        _ => true,
    }
}
