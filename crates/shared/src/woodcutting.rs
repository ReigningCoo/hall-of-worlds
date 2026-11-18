//! Woodcutting's rules (plan: `docs/context/woodcutting.md`): the axe held,
//! the tree faced, how long felling takes and the damage it leaves, logs and
//! kindling as items, and Cut Kindling's log. The trees themselves are
//! `shared::tree`. The server decides; the client predicts with the same rules.
//! See `docs/context/shared/woodcutting.md`.

mod axe;
mod cut;
mod damage;
mod dev;
mod faced;
mod felling;
mod kindling;
mod log;

pub use axe::HeldAxe;
pub use cut::{LogAt, cut_seconds, cut_ticks, log_to_cut};
pub use damage::{DAMAGE_KEPT_MINUTES, Damage};
pub use dev::dev_wood;
pub use faced::{holds_tree, tree_faced};
pub use felling::{fell_seconds, fell_ticks};
pub use kindling::{KINDLING_GRAMS, KindlingForm, kindling_from, new_kindling};
pub use log::{LOG_CM3, LogForm, new_log};

/// Woodcutting's path in the skill tree, practised by both abilities.
pub const WOODCUTTING_SKILL: &str = "gathering/woodcutting";

/// Woodcutting practice for a tree felled: as Collect Water's 8 for each 10
/// seconds of a minute's work (Claude's default).
pub const FELL_PRACTICE: u32 = 48;

/// Woodcutting practice for a log split into kindling.
pub const KINDLING_PRACTICE: u32 = 8;

#[cfg(test)]
mod tests;
