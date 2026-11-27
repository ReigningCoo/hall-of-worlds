//! A set's bonuses: one for each number of its pieces worn from 2 up, and
//! one more for the whole set, so a full set of N gives N (owner: at least
//! one a piece). Each a different stat from the same list quality bonuses
//! roll from (`bonus::GEAR_STATS`), suiting where any piece is worn, or a
//! skill a piece suits (`bonus::gear_skills`); stronger than a quality
//! bonus (`SET_BONUS_FACTOR`).

use super::roll;
use crate::bonus::{Amount, Bonus, GEAR_STATS, SKILL_LEVEL, Stat, gear_skills, places_of};
use crate::id::Id;
use crate::item::{Item, ItemKind};

/// A set bonus is this many times a quality bonus's usual size (first
/// guess: +10% of an attribute).
pub const SET_BONUS_FACTOR: i32 = 2;

/// One of a set's bonuses, and how many of its pieces must be worn for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBonus {
    pub worn: u8,
    pub bonus: Bonus,
}

/// Set `number`'s bonuses, fewest pieces worn first.
pub(super) fn roll_bonuses(number: u16, pieces: &[ItemKind]) -> Vec<SetBonus> {
    let mut left = candidates(pieces);
    let mut rng = roll::seed(number, "bonuses").rng();
    // One at each count from 2 to all of them, and the last count again.
    let counts = (2..=pieces.len()).chain([pieces.len()]);
    let mut bonuses = Vec::new();
    for worn in counts {
        if left.is_empty() || pieces.len() < 2 {
            break;
        }
        let at = rng.range_u64(0, left.len() as u64) as usize;
        bonuses.push(SetBonus {
            worn: worn as u8,
            bonus: stronger(left.remove(at)),
        });
    }
    bonuses
}

/// Every bonus the set could roll, in a fixed order: the gear stats suiting
/// any piece, then the skills any piece suits, each once.
fn candidates(pieces: &[ItemKind]) -> Vec<Bonus> {
    let places = pieces.iter().fold(0, |places, &kind| {
        places | places_of(&roll::locations(kind))
    });
    let stats = GEAR_STATS.iter().filter(|gear| gear.places & places != 0);
    let mut bonuses: Vec<Bonus> = stats
        .map(|gear| Bonus {
            stat: gear.stat,
            amount: gear.amount,
        })
        .collect();
    for &kind in pieces {
        // A stand-in of the kind: a weapon's own skills need its blade, so
        // only armour's and footwear's come from a kind alone.
        for skill in gear_skills(&Item::new(Id::NOBODY, kind)) {
            let bonus = Bonus::flat(Stat::Skill(skill), SKILL_LEVEL);
            if !bonuses.contains(&bonus) {
                bonuses.push(bonus);
            }
        }
    }
    bonuses
}

/// A bonus at a set bonus's size.
fn stronger(bonus: Bonus) -> Bonus {
    let amount = match bonus.amount {
        Amount::Flat(milli) => Amount::Flat(milli.saturating_mul(SET_BONUS_FACTOR)),
        Amount::Percent(milli) => Amount::Percent(milli.saturating_mul(SET_BONUS_FACTOR)),
    };
    Bonus { amount, ..bonus }
}
