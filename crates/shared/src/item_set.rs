//! Item sets (owner, 2026-11-27): a fixed pool of sets, each made from its
//! number alone: a placeholder name, one quality for every piece, 2 to 7
//! pieces worn in different places (two rings may be), and bonuses for
//! wearing 2 or more of its pieces at once. Planets pick their set item from
//! the pool (`quest::set_item_of`), so the same set turns up on many planets.
//! Nothing but which set and piece is saved on an item (`Item::set`).

mod bonuses;
mod piece;
mod roll;
mod saved;
mod set;
mod worn;

pub use bonuses::{SET_BONUS_FACTOR, SetBonus};
pub use piece::SetPiece;
pub(crate) use saved::SavedSet;
pub use set::ItemSet;
pub use worn::{set_bonuses, worn_pieces};

/// How many sets there are (first guess): every planet picks from these.
pub const POOL: u16 = 1024;
/// The fewest pieces a set has.
pub const MIN_PIECES: usize = 2;
/// The most pieces a set has.
pub const MAX_PIECES: usize = 7;

#[cfg(test)]
mod tests;
