//! A new character's first way out of the inn (owner, 2026-11-23): a place on
//! the Hall's walkway rolled from their id, as their world map start is
//! (`world_map::arrival`), kept as their way back (`inn_return`) like any
//! other. `HOW_DEBUG_INN_EXIT` puts it somewhere chosen instead, and the debug
//! API's `server.inn_exit` sets or rerolls it for a player in the inn.

use shared::hall::{self, HALL_LENGTH, WALKWAY_ROWS};
use shared::protocol::CharacterId;
use shared::rng::Seed;
use shared::spot::Spot;

/// `HOW_DEBUG_INN_EXIT=x[,y]`: every new character's first way out of the
/// inn is this Hall tile (`y` the walkway's row, 0 to 2, the middle if left
/// out), to test from a known place.
pub(crate) const VARIABLE: &str = "HOW_DEBUG_INN_EXIT";

/// Where a new character first comes out of the inn into the Hall.
pub(crate) fn first_way_back(character: &CharacterId) -> Spot {
    let chosen = std::env::var(VARIABLE)
        .ok()
        .and_then(|text| tile_from(&text));
    let tile = chosen.unwrap_or_else(|| rolled(character.0.seed().derive("inn first exit")));
    Spot::middle_of(tile)
}

/// A tile anywhere along the walkway, from a seed.
pub(crate) fn rolled(seed: Seed) -> (i64, i64) {
    let mut rng = seed.rng();
    let x = rng.range_u64(0, HALL_LENGTH as u64) as i64;
    let y = rng.range_u64(0, WALKWAY_ROWS as u64) as i64;
    (x, y)
}

/// A walkway tile from `x` or `x,y` (`y` kept on the walkway); `None` if it
/// isn't one.
pub(crate) fn tile_from(text: &str) -> Option<(i64, i64)> {
    let mut parts = text.split(',').map(|part| part.trim().parse::<i64>());
    let x = parts.next()?.ok()?;
    let y = match parts.next() {
        Some(y) => y.ok()?,
        None => WALKWAY_ROWS / 2,
    };
    if parts.next().is_some() {
        return None;
    }
    Some((hall::wrap_x(x), y.clamp(0, WALKWAY_ROWS - 1)))
}

#[cfg(test)]
mod tests;
