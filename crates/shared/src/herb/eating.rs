//! Eating a herb: its raw dose (`raw_dose`), and for a herb that bears fruit
//! (bilberry, elder, hawthorn, juniper, bearberry, wintergreen) that fruit's
//! healing and effects too (`fruit::eating`).

use super::{HerbDef, raw_dose};
use crate::bonus::Effect;
use crate::fruit::{self, Fruits};
use crate::item::Quality;

/// Whether a herb bearing fruit still gives its raw dose besides the fruit's
/// healing and effects (owner, 2026-11-17: they "have fruit effects too", so
/// both). The one place to change if that changes.
pub const RAW_DOSE_WITH_FRUIT: bool = true;

/// What eating one of `def` at this quality does from `now`, for a user with
/// this much Herbalism (`skill_value`).
pub fn eating_herb(def: &HerbDef, quality: Quality, herbalism: u32, now: u64) -> Vec<Effect> {
    let fruit = (def.fruit.as_deref()).and_then(|key| Fruits::built_in().get(key));
    let mut effects = Vec::new();
    if fruit.is_none() || RAW_DOSE_WITH_FRUIT {
        effects.extend(raw_dose(def, quality, herbalism, now));
    }
    if let Some(fruit) = fruit {
        effects.extend(fruit::eating(fruit, now));
    }
    effects
}
