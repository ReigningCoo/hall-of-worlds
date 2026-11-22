//! Decor's fixed lights: sconces, chandeliers and candles on tables, always
//! lit. How far each reaches is a first guess for the owner to tune.

use crate::dungeon::Level;
use crate::furniture::Furniture;

/// How far a sconce's light reaches, in cells.
pub const SCONCE_LIGHT_CELLS: u32 = 4;

/// How far a chandelier's light reaches, in cells.
pub const CHANDELIER_LIGHT_CELLS: u32 = 6;

/// How far candles on a table light, in cells.
pub const CANDLE_LIGHT_CELLS: u32 = 3;

/// A light that stays where it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedLight {
    /// The cell it shines from.
    pub cell: (i64, i64),
    /// How far it reaches, in cells.
    pub cells: u32,
}

/// Every fixed light decor and candles give on `level`, in a set order.
pub fn fixed_lights(level: &Level) -> Vec<FixedLight> {
    let decor = (level.decor())
        .filter_map(|decor| decor.light())
        .map(|(cell, cells)| FixedLight { cell, cells });
    let candles = (level.furniture_on_top())
        .filter(|piece| matches!(piece.furniture, Furniture::Candles { .. }))
        .map(|piece| FixedLight {
            cell: piece.at,
            cells: CANDLE_LIGHT_CELLS,
        });
    decor.chain(candles).collect()
}
