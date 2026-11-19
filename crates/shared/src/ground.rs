//! What can be told of something on the ground from afar: how full its bag looks,
//! the cask standing there, or the logs lying there.
//! What is in it is only told to someone within reach of it. See
//! `docs/context/shared/ground.md`.

use serde::{Deserialize, Serialize};

use crate::item::Item;

mod reach;

pub use reach::{ARM_REACH, in_reach, nearest_in_reach, reach};

/// How full a bag on the ground looks, by the weight of all in it, or the
/// biggest cask lying loose in it. Saved in what characters remember.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Load {
    /// Almost flat.
    Flat,
    Slack,
    Half,
    Full,
    Bulging,
    /// A cask standing there (`shared::cask`), whatever else is with it.
    Cask(crate::cask::CaskLook),
    /// Logs lying there (`shared::woodcutting`), whatever else is with them
    /// but a cask: the first one's wood, and how many show, 1 to `MOST_LOGS`.
    Logs {
        wood: crate::material::Material,
        count: u8,
    },
}

/// The most logs a pile shows: three or more look alike (owner).
pub const MOST_LOGS: u8 = 3;

impl Load {
    /// Every bag's look (casks have their own).
    pub const ALL: [Self; 5] = [
        Self::Flat,
        Self::Slack,
        Self::Half,
        Self::Full,
        Self::Bulging,
    ];

    /// How full a bag this heavy looks, in grams. First guesses.
    pub fn of_weight(grams: u32) -> Self {
        match grams {
            0..200 => Self::Flat,
            200..1_000 => Self::Slack,
            1_000..5_000 => Self::Half,
            5_000..15_000 => Self::Full,
            _ => Self::Bulging,
        }
    }

    /// How full a bag holding these looks, or the biggest cask among them,
    /// or the logs among them.
    pub fn of(items: &[Item]) -> Self {
        let cask = items
            .iter()
            .filter_map(Item::cask)
            .max_by_key(|cask| cask.size);
        if let Some(cask) = cask {
            return Self::Cask(cask.look());
        }
        let mut logs = items
            .iter()
            .filter_map(|item| Some((item.log()?, item.count())));
        if let Some((first, count)) = logs.next() {
            let count = logs.fold(count, |all, (_, count)| all.saturating_add(count));
            return Self::Logs {
                wood: first.wood,
                count: count.min(u32::from(MOST_LOGS)) as u8,
            };
        }
        let grams = items.iter().map(Item::total_weight).sum();
        Self::of_weight(grams)
    }
}

#[cfg(test)]
mod tests;
