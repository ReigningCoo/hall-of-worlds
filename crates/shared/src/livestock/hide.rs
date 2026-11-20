//! `Hide`: an animal's skin taken when it is butchered, and what curing makes
//! of it (owner, decisions 7 and 8).

use serde::{Deserialize, Serialize};

use crate::item::Quality;

/// A raw hide rots after 7 hours (owner, decision 8): of playtime while
/// carried, of real time on the ground (like casks' clocks). Salting it with
/// the game's Salt stops the clock until it is cured.
pub const HIDE_ROTS_AFTER_SECONDS: u64 = 7 * 60 * 60;

/// What curing makes of a hide.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cure {
    /// Scraped and tanned.
    Leather,
    /// Cured with its fur or wool on: every furry hide (decision 7).
    Pelt,
}

/// A young animal's finer skin (calfskin, lambskin, kidskin).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct YoungHide {
    pub name: String,
    pub lowest: Quality,
    pub highest: Quality,
}

/// An animal's hide.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hide {
    /// The raw hide ("cowhide").
    pub name: String,
    pub cure: Cure,
    /// What curing makes ("cow leather", "sheep pelt").
    pub cured: String,
    /// The qualities it comes in.
    pub lowest: Quality,
    pub highest: Quality,
    #[serde(default)]
    pub young: Option<YoungHide>,
    /// Its uses, and any exception (shell cordovan).
    #[serde(default)]
    pub note: String,
}

impl Hide {
    pub(super) fn check(&self) -> Result<(), String> {
        let named = |name: &String| !name.trim().is_empty();
        if !named(&self.name) || !named(&self.cured) {
            return Err("a hide has no name".to_owned());
        }
        let in_order = |low: Quality, high: Quality| low <= high && high <= Quality::Legendary;
        if !in_order(self.lowest, self.highest) {
            return Err(format!("{}: its qualities are out of order", self.name));
        }
        if let Some(young) = &self.young
            && (!named(&young.name) || !in_order(young.lowest, young.highest))
        {
            return Err(format!("{}: its young hide is wrong", self.name));
        }
        Ok(())
    }
}
