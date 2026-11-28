//! Whether a listener hears a noise: its reach by their hearing, muffled on
//! the way; certain within half of it, then less likely to nothing at its end,
//! rolled from a hash so both sides agree.

use super::{Noise, muffled};
use crate::creature::Acuity;
use crate::dungeon::Level;
use crate::rng::Seed;
use crate::spot::Spot;

/// What a hearing makes of a noise's reach, per mille: as the old fight
/// noise's 4, 8 and 14 cells.
pub const fn hearing_per_mille(hearing: Acuity) -> u32 {
    match hearing {
        Acuity::None => 0,
        Acuity::Poor => 500,
        Acuity::Normal => 1000,
        Acuity::Keen => 1750,
    }
}

/// Someone listening: where, how well, and their own seed for the roll (their
/// id's, `Id::seed`).
#[derive(Clone, Copy, Debug)]
pub struct Hearer {
    pub at: Spot,
    pub hearing: Acuity,
    pub seed: Seed,
}

/// How far `noise` reaches for `hearer`, in units, through what is between.
pub fn reach(level: &Level, noise: &Noise, hearer: &Hearer) -> u32 {
    let open = u64::from(noise.loudness) * u64::from(hearing_per_mille(hearer.hearing)) / 1000;
    muffled(level, noise.at, hearer.at, open as u32)
}

/// The chance `hearer` hears `noise`, per mille.
pub fn chance(level: &Level, noise: &Noise, hearer: &Hearer) -> u32 {
    let reach = u64::from(reach(level, noise, hearer));
    let apart = noise.at.distance(hearer.at);
    if apart >= reach {
        0
    } else if apart * 2 <= reach {
        1000
    } else {
        ((reach - apart) * 2000 / reach) as u32
    }
}

/// The roll for `hearer` and `noise` at `tick`, from 0 to 999.
pub fn roll(noise: &Noise, hearer: &Hearer, tick: u64) -> u32 {
    let hash =
        hearer
            .seed
            .derive("noise")
            .at(&[tick as i64, noise.at.x, noise.at.y, noise.source as i64]);
    (hash % 1000) as u32
}

/// Whether `hearer` hears `noise` made at `tick`.
pub fn heard(level: &Level, noise: &Noise, hearer: &Hearer, tick: u64) -> bool {
    let chance = chance(level, noise, hearer);
    chance > 0 && roll(noise, hearer, tick) < chance
}
