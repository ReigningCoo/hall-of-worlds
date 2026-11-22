//! Decor (tavern furniture plan PR 5): what hangs on walls, chandeliers and
//! rugs, kept on `dungeon::Level` beside its furniture and sent with its
//! cells. None of it blocks or slows anyone.
//!
//! Owner's rule: nothing hangs from the ceiling, as it confuses the player and
//! the camera; chandeliers are the only exception. Things on walls hang on a
//! wall cell's face (`Decor::Wall`: the wall and the side it faces), never on
//! floor. Sconces, chandeliers and candles on tables (`furniture::Candles`)
//! are fixed lights, always lit (a first guess: they are only put in lived-in
//! places); they light the place, never a player (no light without a lantern
//! still holds). Placed by code: Place Furniture now, castles, towers and the
//! starting inn later.

// Split into files (`cargo xtask lint` keeps files small).
mod chandelier;
mod hanging;
mod light;
mod piece;
mod rug;

pub use chandelier::ChandelierStyle;
pub use hanging::Hanging;
pub use light::{
    CANDLE_LIGHT_CELLS, CHANDELIER_LIGHT_CELLS, FixedLight, SCONCE_LIGHT_CELLS, fixed_lights,
};
pub use piece::{Decor, DecorSlot};
pub use rug::{RUG_MOST, Rug, RugColour, RugPattern};

#[cfg(test)]
mod tests;
