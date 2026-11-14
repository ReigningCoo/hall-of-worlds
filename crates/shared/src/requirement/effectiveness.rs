//! How well an item works for someone: a factor in thousandths, from nothing at
//! half of what it asks to full at all of it. Nothing is ever refused.

use super::kind::{Requirement, item_requirements};
use crate::bonus::{Amount, Bonus, item_bonuses};
use crate::item::Item;
use crate::sheet::Attributes;

/// The factor when everything asked is met: thousandths, so 1000 is 100%.
pub const FULL: u32 = 1000;

/// How well something asking `min` works with `value`: 0 at half of `min` or
/// less, rising in a straight line to `FULL` at `min` or more.
pub fn factor_for(min: u16, value: u16) -> u32 {
    if min == 0 {
        return FULL;
    }
    // (value − min/2) ÷ (min/2), worked in doubles so halves don't round.
    let over_half = (2 * i64::from(value) - i64::from(min)).max(0);
    (over_half * i64::from(FULL) / i64::from(min)).min(i64::from(FULL)) as u32
}

/// How well something with these requirements works with these attributes: the
/// lowest of the attributes' factors (skills and kinds don't change it).
pub fn effectiveness(requirements: &[Requirement], attributes: &Attributes) -> u32 {
    requirements
        .iter()
        .filter_map(|requirement| match *requirement {
            Requirement::Attribute { attribute, min } => {
                Some(factor_for(min, attributes.get(attribute)))
            }
            Requirement::Skill { .. } | Requirement::Kinds(_) => None,
        })
        .min()
        .unwrap_or(FULL)
}

/// What this item gives while worn by someone with these attributes (their own
/// with effects, never other worn items', so nothing goes round in a loop): its
/// good bonuses times its effectiveness, its bad ones always in full.
pub fn effective_bonuses(item: &Item, attributes: &Attributes) -> Vec<Bonus> {
    let factor = effectiveness(&item_requirements(item), attributes);
    let mut bonuses = item_bonuses(item);
    if factor < FULL {
        for bonus in &mut bonuses {
            bonus.amount = scaled(bonus.amount, factor);
        }
    }
    bonuses
}

/// A good amount times the factor; a bad one as it is.
fn scaled(amount: Amount, factor: u32) -> Amount {
    let times = |milli: i32| {
        if milli > 0 {
            (i64::from(milli) * i64::from(factor) / i64::from(FULL)) as i32
        } else {
            milli
        }
    };
    match amount {
        Amount::Flat(milli) => Amount::Flat(times(milli)),
        Amount::Percent(milli) => Amount::Percent(times(milli)),
    }
}
