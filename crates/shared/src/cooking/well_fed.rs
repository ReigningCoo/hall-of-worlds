//! Well Fed (owner, 2026-11-21): eating a dish restores Health and/or Mana
//! for 20 minutes at Poor, 10 more a quality step. Protein, fat and nuts
//! lean it towards Health, starch and sweetness towards Mana, the rest
//! halfway. Its foods' effects come with it: vegetables' cooked effects (raw
//! ones if nothing was heated), fruits', fungi's food effects and herbs'
//! first. One Well Fed at a time (`bonus::Source::WellFed`).

use super::mix::Foods;
use super::{Ingredients, Portion, Role, Source};
use crate::bonus::{self, Bonus, Effect, EndsOn};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::{Fruits, bonus_of};
use crate::fungus::Fungi;
use crate::herb::{Herbs, Stat};
use crate::item::Quality;
use crate::vegetable::Vegetables;
use crate::vitals::Vital;

/// Health and Mana restored a second in all, in thousandths of a point.
/// First guess.
pub const WELL_FED_RESTORE: u32 = 250;
/// How long it lasts at Poor, and how much longer each quality step, in
/// minutes.
pub const WELL_FED_MINUTES: u64 = 20;
pub const WELL_FED_STEP_MINUTES: u64 = 10;
/// The strongest an effect gets however many foods give it, either way.
pub const MOST_CHANGE: i8 = 3;

const MINUTE: u64 = 60 * ACTION_TICKS_PER_SECOND as u64;

/// What eating a dish restores and changes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WellFed {
    /// Health restored a second, in thousandths of a point.
    pub health: u32,
    pub mana: u32,
    /// Its foods' effects as bonuses.
    pub bonuses: Vec<Bonus>,
}

impl WellFed {
    /// What a dish of these portions gives; `cooked` if any step heated it.
    pub fn of(portions: &[Portion], cooked: bool, ingredients: &Ingredients) -> Self {
        let foods = Foods::of(portions, ingredients);
        let (mut health, mut mana) = (0u64, 0u64);
        for &(def, grams) in &foods.0 {
            for &role in &def.roles {
                let weight = u64::from(grams) * if def.main == Some(role) { 2 } else { 1 };
                match role {
                    Role::Protein | Role::Fat | Role::Nut => health += 2 * weight,
                    Role::Starch | Role::Sweet => mana += 2 * weight,
                    _ => (health, mana) = (health + weight, mana + weight),
                }
            }
        }
        let all = health + mana;
        let health = match all {
            0 => 0,
            _ => (u64::from(WELL_FED_RESTORE) * health).div_ceil(all) as u32,
        };
        let mana = if all == 0 {
            0
        } else {
            WELL_FED_RESTORE - health
        };
        let bonuses = effects(&foods, cooked);
        Self {
            health,
            mana,
            bonuses,
        }
    }

    /// Its effect for a dish of `quality`, from `now`: nothing ends it
    /// early.
    pub fn effect(&self, quality: Quality, now: u64) -> Effect {
        let mut bonuses = Vec::new();
        for (vital, amount) in [(Vital::Health, self.health), (Vital::Mana, self.mana)] {
            if amount > 0 {
                bonuses.push(Bonus::flat(bonus::Stat::Restore(vital), amount as i32));
            }
        }
        bonuses.extend(self.bonuses.iter().cloned());
        Effect {
            source: bonus::Source::WellFed,
            bonuses,
            start: now,
            end: Some(now + well_fed_ticks(quality)),
            ends_on: EndsOn::NOTHING,
        }
    }
}

/// How long Well Fed lasts for a dish of `quality`, in action ticks.
pub fn well_fed_ticks(quality: Quality) -> u64 {
    (WELL_FED_MINUTES + WELL_FED_STEP_MINUTES * quality as u64) * MINUTE
}

/// The foods' effects added up by stat, each at most `MOST_CHANGE` either
/// way, in `Stat::ALL`'s order.
fn effects(foods: &Foods, cooked: bool) -> Vec<Bonus> {
    let mut changes = [0i32; Stat::ALL.len()];
    let mut add = |stat: Stat, change: i8| {
        if let Some(at) = Stat::ALL.iter().position(|&s| s == stat) {
            changes[at] += i32::from(change);
        }
    };
    for &(def, _) in &foods.0 {
        match def.source {
            Source::Herb => {
                let herb = Herbs::built_in_ref().get(&def.key);
                if let Some(first) = herb.and_then(|h| h.effects.first()) {
                    add(first.stat, first.change);
                }
            }
            Source::Fruit => {
                for e in Fruits::built_in()
                    .get(&def.key)
                    .map_or(&[][..], |f| &f.effects)
                {
                    add(e.stat, e.change);
                }
            }
            Source::Vegetable => {
                for e in Vegetables::built_in()
                    .get(&def.key)
                    .map_or(&[][..], |v| &v.effects)
                {
                    if if cooked {
                        e.when.cooked()
                    } else {
                        e.when.raw()
                    } {
                        add(e.stat, e.change);
                    }
                }
            }
            Source::Fungus => {
                for e in Fungi::built_in().get(&def.key).map_or(&[][..], |f| &f.food) {
                    add(e.stat, e.change);
                }
            }
            _ => {}
        }
    }
    let most = i32::from(MOST_CHANGE);
    (Stat::ALL.iter().zip(changes))
        .filter(|(_, change)| *change != 0)
        .filter_map(|(&stat, change)| bonus_of(stat, change.clamp(-most, most) as i8))
        .collect()
}
