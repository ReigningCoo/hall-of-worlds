//! Wine racks and spirit racks (tavern furniture plan PR 2, owner's design):
//! furniture with places (`furniture::Held`) for bottles, 1 to 3 cells long.
//! A wine rack stands full height against a wall, wine bottles lying on their
//! sides in a diamond lattice; a spirit rack is three stepped shelves of
//! upright bottles rising towards the back, standing on a table (or a bar
//! counter, later): the first furniture standing on furniture
//! (`Furniture::stands_on_furniture`). Sealed drinks on either age
//! `cask_rack::RACK_AGEING_PERCENT` as fast, by world time (`ferment`'s glass
//! clock). Kept by the server as cask racks are. See
//! `docs/context/shared/bottle_rack.md`.

// Split into files (`cargo xtask lint` keeps files small).
mod bottles;
mod rack;

pub use bottles::RackBottles;
pub use rack::{BottlePlace, BottleRack, BottleRackKind, SPIRIT_SHELVES, WINE_ROWS};

#[cfg(test)]
mod tests;
