//! How long fermenting takes (owner: real durations in game days, ale about
//! 24, cider and perry about 36, wine about 90; longer in bigger casks,
//! shorter with better and more yeast; yeasts stack).

use crate::cask::CaskSize;
use crate::item::Quality;

use super::DrinkKind;

/// One game day: one real hour, in seconds.
pub const GAME_DAY_SECS: u64 = 3600;

/// The millilitres one yeast starts at its usual pace: a Pin's 20 L.
pub const PITCH_ML: u64 = 20_000;

/// The most yeast counts for, in percent of one yeast to `PITCH_ML`: four
/// times as much, which ferments in two fifths of the time.
const MOST_PITCH: u64 = 400;

/// How long `kind` takes in a Pin with one Common yeast, in real seconds.
pub fn base_secs(kind: DrinkKind) -> u64 {
    let days = match kind {
        DrinkKind::Ale => 24,
        DrinkKind::Cider | DrinkKind::Perry => 36,
        DrinkKind::Wine | DrinkKind::FruitWine => 90,
    };
    days * GAME_DAY_SECS
}

/// Percent of the time for a cask of `size`: a tenth longer for each size
/// past the Pin (a Tun 170%).
fn size_percent(size: CaskSize) -> u64 {
    let step = CaskSize::ALL.iter().position(|&s| s == size).unwrap_or(0);
    100 + 10 * step as u64
}

/// Percent of the time for yeast of `quality`: Poor 125, Common 100, then a
/// tenth less a step to Artifact's 50.
fn quality_percent(quality: Quality) -> u64 {
    match quality {
        Quality::Poor => 125,
        Quality::Common => 100,
        Quality::Uncommon => 90,
        Quality::Rare => 80,
        Quality::Epic => 70,
        Quality::Legendary => 60,
        Quality::Artifact => 50,
    }
}

/// Percent of the time for `yeasts` yeasts in `ml`: 100 at one to
/// `PITCH_ML`, 200 with next to none, 40 at `MOST_PITCH` or more.
fn pitch_percent(yeasts: usize, ml: u32) -> u64 {
    let pitch = yeasts as u64 * PITCH_ML * 100 / u64::from(ml.max(1));
    10_000 / (50 + pitch.min(MOST_PITCH) / 2)
}

/// How long `ml` of `kind` takes to ferment in a cask of `size` with
/// `yeasts` (each one item; their qualities averaged), in real seconds.
/// No yeast: wild fermenting, timed as one Poor yeast.
pub fn ferment_secs(kind: DrinkKind, size: CaskSize, ml: u32, yeasts: &[Quality]) -> u64 {
    let (count, quality) = match yeasts.len() {
        0 => (1, quality_percent(Quality::Poor)),
        n => {
            let sum: u64 = yeasts.iter().map(|&q| quality_percent(q)).sum();
            (n, sum / n as u64)
        }
    };
    base_secs(kind) * size_percent(size) * quality * pitch_percent(count, ml) / 1_000_000
}
