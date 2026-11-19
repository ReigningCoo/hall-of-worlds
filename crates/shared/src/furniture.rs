//! Furniture in sites' rooms: tables, chairs and benches of wood or stone.
//!
//! Pieces are kept on `dungeon::Level` (as doors are) and sent with its cells.
//! They never block: crossing a table takes `TABLE_COST_PER_MILLE` of the time,
//! a seat `SEAT_COST_PER_MILLE`, so routes go round them (`movement::LevelWay`).
//! Material and style are only for models, never named to players. Placed by
//! `furnish` at the end of `site::Site::level`, from hashes of the site's seed,
//! so generators and their versions are untouched.

// Split into files (`cargo xtask lint` keeps files small).
mod material;
mod piece;
pub(crate) mod place;
mod shape;
mod style;

pub use material::{Material, Stone, Wood};
pub use piece::{Furniture, Piece, SEAT_COST_PER_MILLE, TABLE_COST_PER_MILLE};
pub use place::{Setting, furnish};
pub use shape::footprint;
pub use style::{BenchStyle, ChairStyle, TableStyle};

#[cfg(test)]
mod tests;
