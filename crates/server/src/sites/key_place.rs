//! Where a player is, as keys remember it (`item_form::KeyPlace`): the planet
//! with its runes, the world map point, and the named place there if any.

use shared::hall::GateCode;
use shared::item_form::KeyPlace;

use super::{Place, place_name_of};
use crate::ServerWorld;
use crate::connections::Client;

/// Where `client` is now, in `world` (theirs).
pub(crate) fn key_place(client: &Client, world: &ServerWorld) -> KeyPlace {
    if client.visit.is_none() && client.character.hall.is_some() {
        return KeyPlace::Hall;
    }
    let (poi, (x, y)) = match client.visit.as_ref().map(|visit| (&visit.place, visit)) {
        Some((Place::Site { poi, .. }, _)) => (Some(*poi), (poi.x, poi.y)),
        Some((Place::Wilderness { in_poi, .. }, visit)) => {
            let (x, y) = visit.position();
            (*in_poi, world.wilderness.wild.point_of(x, y))
        }
        None => (None, client.world_position()),
    };
    KeyPlace::Planet {
        world: world.seed.id(),
        runes: GateCode::of_planet(world.seed),
        name: poi.and_then(|poi| place_name_of(world, &poi)),
        x,
        y,
    }
}
