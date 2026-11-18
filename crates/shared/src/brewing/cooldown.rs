//! Drinking's cooldown (brewing decision 20): 30 s before a potion of the
//! same combination of stats and signs can be drunk again, whatever its
//! strength; other combinations can be drunk meanwhile. Kept on the sheet, in
//! action ticks, so it runs on in real time, offline too.

use serde::{Deserialize, Serialize};

use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::Stat;
use crate::potion::Contents;

/// How long before the same combination can be drunk again, in seconds.
pub const DRINK_COOLDOWN_SECONDS: u64 = 30;

/// The cooldown in action ticks.
pub const DRINK_COOLDOWN_TICKS: u64 = DRINK_COOLDOWN_SECONDS * ACTION_TICKS_PER_SECOND as u64;

/// Which stats a potion moves, and which way: a bit per stat and sign.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Combination(u32);

impl Combination {
    /// The combination of these contents' doses.
    pub fn of(contents: &Contents) -> Self {
        let mut bits = 0;
        for dose in contents.doses() {
            let place = Stat::ALL.iter().position(|&stat| stat == dose.stat);
            let harmful = u32::from(dose.step < 0);
            bits |= 1 << (place.unwrap_or(0) as u32 * 2 + harmful);
        }
        Self(bits)
    }
}

/// The combinations drunk lately, each with the action tick it can be drunk
/// again at. None in older saves.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cooldowns(Vec<(Combination, u64)>);

impl Cooldowns {
    /// The action tick this combination can be drunk again at, if it can't now.
    pub fn ready_at(&self, combination: Combination, now: u64) -> Option<u64> {
        self.0
            .iter()
            .find(|&&(it, ready)| it == combination && now < ready)
            .map(|&(_, ready)| ready)
    }

    /// Whether this combination can be drunk now.
    pub fn ready(&self, combination: Combination, now: u64) -> bool {
        self.ready_at(combination, now).is_none()
    }

    /// Notes this combination drunk at `now`, forgetting cooldowns that are over.
    pub fn drank(&mut self, combination: Combination, now: u64) {
        self.0
            .retain(|&(it, ready)| it != combination && now < ready);
        self.0.push((combination, now + DRINK_COOLDOWN_TICKS));
    }
}
