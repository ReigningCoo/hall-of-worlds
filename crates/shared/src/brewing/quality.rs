//! A potion's quality from its doses (decision 16), how long one lasts by its
//! quality (decision 18), and an ointment used on oneself (decision 8).

use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::Quality;
use crate::potion::{Contents, Dose};

/// How long a Common potion's attributes last, in seconds.
pub const COMMON_SECONDS: u32 = 120;
/// How much longer each quality above Common lasts, and how much shorter Poor
/// is, in seconds.
pub const SECONDS_PER_QUALITY: u32 = 30;
/// An ointment on the skin gives its doses divided by this.
pub const ON_SKIN_SHARE: i16 = 4;

/// How good a potion (or ointment) with these doses is: the strongest dose's
/// steps less the next strongest's (either way), divided by 3, so purer is
/// better: 2 or less Poor, 3–4 Common, 5–6 Uncommon, 7–8 Rare, 9–10 Epic, 11
/// or more Legendary; never Artifact.
pub fn quality(contents: &Contents) -> Quality {
    let mut sizes: Vec<i32> = contents
        .doses()
        .iter()
        .map(|dose| i32::from(dose.step).abs())
        .collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let strongest = sizes.first().copied().unwrap_or(0);
    let next = sizes.get(1).copied().unwrap_or(0);
    match (strongest - next) / 3 {
        ..=2 => Quality::Poor,
        3..=4 => Quality::Common,
        5..=6 => Quality::Uncommon,
        7..=8 => Quality::Rare,
        9..=10 => Quality::Epic,
        _ => Quality::Legendary,
    }
}

/// How long a potion of this quality changes attributes, in seconds: two
/// minutes, 30 s more for each quality above Common, 30 s less for Poor (Poor
/// 90 s to Legendary 4 minutes; Artifact, which brewing never makes, 4½).
pub fn duration_seconds(quality: Quality) -> u32 {
    let steps = Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1) as u32;
    COMMON_SECONDS + steps * SECONDS_PER_QUALITY - SECONDS_PER_QUALITY
}

/// `duration_seconds` in action ticks, as effects' ends are kept.
pub fn duration_ticks(quality: Quality) -> u64 {
    u64::from(duration_seconds(quality)) * ACTION_TICKS_PER_SECOND as u64
}

/// What an ointment used on oneself gives: a quarter of each dose, rounded
/// towards 0, those that come to 0 dropped. It lasts as a potion of the
/// ointment's own `quality` would.
pub fn on_skin(ointment: &Contents) -> Contents {
    Contents::new(ointment.doses().iter().map(|dose| Dose {
        step: dose.step / ON_SKIN_SHARE,
        ..*dose
    }))
}
