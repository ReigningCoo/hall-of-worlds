//! The quality picking a herb gives (owner: Poor to Uncommon mostly, anything
//! better far rarer than other items), better with Foraging.

use crate::item::{ODDS_OUT_OF, Quality};
use crate::rng::Rng;
use crate::skill::{SkillId, Skills, VALUE_SCALE};

/// The skill that makes picked herbs better.
pub const FORAGING: &str = "gathering/foraging";

/// A skill's value at 100%, in hundredths (`Skills::value`).
pub(super) const FULL: u32 = 100 * VALUE_SCALE;

/// How likely each quality is, worst first, out of `ODDS_OUT_OF`, for a picker
/// with no Foraging: Rare and better are 2 in 100, against 8 for other items
/// (`Quality::odds`). First guesses.
pub const HERB_ODDS: [u64; 7] = [4000, 4500, 1300, 150, 40, 9, 1];

/// What full Foraging moves from Poor to the rest; in between, in proportion.
/// Even then Rare and better are 7 in 100.
const SKILLED_SHIFT: [i64; 7] = [-3000, 1000, 1500, 350, 100, 40, 10];

/// The odds of each quality, worst first, for a picker with this much Foraging
/// (hundredths of a percent, `skill_value`).
pub fn pick_odds(foraging: u32) -> [u64; 7] {
    let skill = i64::from(foraging.min(FULL));
    let mut odds = HERB_ODDS;
    for (odds, shift) in odds.iter_mut().zip(SKILLED_SHIFT).skip(1) {
        *odds = (*odds as i64 + shift * skill / i64::from(FULL)) as u64;
    }
    // Poor has what is left, so rounding never loses any.
    odds[0] = ODDS_OUT_OF - odds[1..].iter().sum::<u64>();
    odds
}

/// The quality of a herb picked by someone with this much Foraging.
pub fn pick_quality(rng: &mut Rng, foraging: u32) -> Quality {
    let odds = pick_odds(foraging);
    let mut left = rng.range_u64(0, ODDS_OUT_OF);
    for (quality, odds) in Quality::ALL.into_iter().zip(odds) {
        if left < odds {
            return quality;
        }
        left -= odds;
    }
    Quality::Common
}

/// How good someone is at the skill at `path`, as a check counts it (its value
/// and every value above it), at most 100%: hundredths of a percent.
pub fn skill_value(skills: &Skills, path: &str) -> u32 {
    SkillId::find(path).map_or(0, |id| skills.total(id).min(FULL))
}
