//! Opened casks turning to vinegar (owner: a sealed cask never spoils; once
//! opened a best-before runs, longer for stronger drinks and none over 20%,
//! sped by added vinegar, with a notice near the end), and opened juice
//! fermenting wild into a Poor cider (or fruit wine) instead.

use crate::fruit::FruitDef;

use super::{Fermented, GAME_DAY_SECS, Mash, WILD_TOLERANCE, ferment};

/// Drinks this strong (tenths of a percent: 20%) or stronger keep for ever.
pub const KEEPS_FROM: u32 = 200;

/// How long opened juice keeps before it starts fermenting wild: 2 game
/// days, in real seconds.
pub const JUICE_TURNS_SECS: u64 = 2 * GAME_DAY_SECS;

/// The most vinegar can speed spoiling: to a tenth of the time.
const FASTEST: u64 = 100;

/// How much of `total_ml` is vinegar, in thousandths.
pub fn vinegar_share(vinegar_ml: u32, total_ml: u32) -> u32 {
    let share = u64::from(vinegar_ml) * 1000 / u64::from(total_ml.max(1));
    share.min(1000) as u32
}

/// How long an opened drink of `strength` (tenths of a percent) keeps before
/// it is vinegar, in real seconds: 6 game days plus a quarter of the
/// strength's square in percent (5%: 12 days; 12%: 42; 18%: 87); none at
/// `KEEPS_FROM` or stronger. `vinegar` (thousandths of it, `vinegar_share`)
/// takes 4 thousandths off the time for each, down to a tenth.
pub fn best_before_secs(strength: u32, vinegar: u32) -> Option<u64> {
    if strength >= KEEPS_FROM {
        return None;
    }
    let strength = u64::from(strength);
    let secs = 6 * GAME_DAY_SECS + strength * strength * GAME_DAY_SECS / 400;
    let left = 1000u64.saturating_sub(4 * u64::from(vinegar)).max(FASTEST);
    Some(secs * left / 1000)
}

/// Whether the player should be told an opened drink is near its end: in the
/// last tenth of `best_before` (seconds), once `opened_for` seconds have
/// passed, and not yet past it.
pub fn notice_due(best_before: u64, opened_for: u64) -> bool {
    opened_for < best_before && opened_for >= best_before - best_before / 10
}

/// What `ml` of `def`'s opened juice ferments into once left
/// `JUICE_TURNS_SECS`: wild yeast up to `WILD_TOLERANCE`, always Poor
/// (`drink_quality` with no yeast). Apple juice turns to a 5.8% cider.
pub fn wild_cider(def: &FruitDef, ml: u32) -> Option<Fermented> {
    ferment(&Mash::juice(def, ml)?, WILD_TOLERANCE)
}
