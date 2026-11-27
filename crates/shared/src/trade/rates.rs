//! What a shop pays and asks, as a percentage of an item's value: 50% and
//! 150%, each moved towards 100% by town standing (up to 25%) and the
//! Trading skill (up to 25%), added (owner). With both full a shop pays
//! and asks the value itself.

use serde::{Deserialize, Serialize};

use crate::quest::Standing;

/// What a shop pays before standing and skill, as a percentage of value.
pub const BUYS_AT: u32 = 50;

/// What a shop asks before standing and skill.
pub const SELLS_AT: u32 = 150;

/// The most town standing moves both, in percent.
pub const MOST_STANDING: u32 = 25;

/// The most the Trading skill moves both, in percent.
pub const MOST_SKILL: u32 = 25;

/// A shop's rates for one character, in percent of value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rates {
    /// What it pays for what it is sold.
    pub buys: u32,
    /// What it asks for what it sells.
    pub sells: u32,
}

impl Rates {
    /// The rates for a character of this standing in the shop's town, with
    /// this much Trading (a whole percentage, `Skills::percent`).
    pub fn for_shop(standing: Standing, trading: u32) -> Self {
        let off = standing_percent(standing) + skill_percent(trading);
        Self {
            buys: BUYS_AT + off,
            sells: SELLS_AT - off,
        }
    }
}

impl Default for Rates {
    /// A stranger's with no Trading.
    fn default() -> Self {
        Self::for_shop(Standing::Stranger, 0)
    }
}

/// What standing takes off: none for a stranger, the most for a champion,
/// evenly between (Claude's, to balance).
pub fn standing_percent(standing: Standing) -> u32 {
    let tier = Standing::ALL.iter().position(|&one| one == standing);
    let top = Standing::ALL.len() as u32 - 1;
    tier.unwrap_or(0) as u32 * MOST_STANDING / top
}

/// What the Trading skill takes off: in step with it, the most at 100%.
pub fn skill_percent(trading: u32) -> u32 {
    trading.min(100) * MOST_SKILL / 100
}
