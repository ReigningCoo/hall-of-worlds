//! How an intermediate changes as it rests (owner, 2026-11-21): stable
//! (dried noodles), proving, better and then worse (dough), or turning into
//! another (fresh noodles drying, warm milk setting into yoghurt).

use super::Rest;
use crate::item::Quality;

const HOUR: u32 = 3600;

/// An intermediate's curve, in seconds of rest at the usual pace
/// (`warmth_percent` 100).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curve {
    /// Nothing changes with time.
    Stable,
    /// One step up from `better_from` until `best_until`, as made again
    /// until `worse_from`, then one step down each `worse_every` after it,
    /// to Poor at worst.
    Proves {
        better_from: u32,
        best_until: u32,
        worse_from: u32,
        worse_every: u32,
    },
    /// It becomes `into` after `after` (a culture's pace changing it).
    Turns { after: u32, into: Rest },
}

impl Curve {
    /// Dough proving: better from 1 h to 4 h, over-proved from 12 h.
    pub const DOUGH: Self = Self::Proves {
        better_from: HOUR,
        best_until: 4 * HOUR,
        worse_from: 12 * HOUR,
        worse_every: 8 * HOUR,
    };

    /// Whether it changes at all.
    pub fn changes(self) -> bool {
        self != Self::Stable
    }

    /// The quality of something made at `base` after `secs` of rest.
    pub fn quality(self, base: Quality, secs: u32) -> Quality {
        match self {
            Self::Proves {
                better_from,
                best_until,
                worse_from,
                worse_every,
            } => {
                if (better_from..best_until).contains(&secs) {
                    return better(base);
                }
                if secs < worse_from {
                    return base;
                }
                let steps = 1 + (secs - worse_from) / worse_every.max(1);
                (0..steps).fold(base, |quality, _| quality.worse())
            }
            Self::Stable | Self::Turns { .. } => base,
        }
    }
}

/// A step better, at most Legendary (Artifacts are never made).
pub(super) fn better(quality: Quality) -> Quality {
    match quality {
        Quality::Legendary | Quality::Artifact => quality,
        _ => Quality::ALL[quality as usize + 1],
    }
}
