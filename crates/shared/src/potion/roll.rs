//! Contents rolled from a seed: for Give Item (dev tools) until brewing makes
//! potions. Both sides roll the same from the same seed.

use super::{Contents, Dose};
use crate::herb::Stat;
use crate::rng::Rng;

/// The most effects Give Item can ask a potion to have.
pub const MOST_EFFECTS: u8 = 4;

/// `effects` doses (at most `MOST_EFFECTS`) on different stats, as brewing's
/// rule would leave them (`research-herbs.md`): the first 4 to 8 steps, the
/// rest 2 to 6, each helpful two times in three.
pub fn roll_contents(rng: &mut Rng, effects: u8) -> Contents {
    let mut stats = Stat::ALL.to_vec();
    let mut doses = Vec::new();
    for n in 0..effects.min(MOST_EFFECTS) {
        let at = rng.range_u64(0, stats.len() as u64) as usize;
        let stat = stats.remove(at);
        let least = if n == 0 { 4 } else { 2 };
        let size = rng.range_u64(least, least + 5) as i16;
        let helpful = rng.range_u64(0, 3) < 2;
        let step = if helpful { size } else { -size };
        doses.push(Dose { stat, step });
    }
    Contents::new(doses)
}
