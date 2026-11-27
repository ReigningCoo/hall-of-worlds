//! Carrying: how much a creature can carry, how weighed down that makes it, and
//! how much that slows it.

use serde::{Deserialize, Serialize};

use super::Sheet;
use crate::bonus::Stat;
use crate::creature::Size;
use crate::energy::NORMAL_SPEED;
use crate::item::Inventory;
use crate::movement::{FULL, MOVEMENT_SKILLS};

/// How weighed down someone is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Load {
    Light,
    Burdened,
    Overloaded,
}

impl Load {
    /// How much of its speed something keeps, as a percentage.
    pub fn speed_percent(self) -> u32 {
        match self {
            Self::Light => 100,
            Self::Burdened => 75,
            Self::Overloaded => 40,
        }
    }
}

impl Sheet {
    /// How much it can carry before it is overloaded, in grams: by strength, and
    /// more the bigger it is. A person of median strength (100) carries 30 kg,
    /// and Weight Training adds its value as a share of that: twice as much at
    /// 100% (owner).
    pub fn capacity(&self, size: Size) -> u32 {
        const GRAMS_PER_STRENGTH_AND_SIZE: u32 = 100;
        let base =
            self.attributes.strength as u32 * (size as u32 + 1) * GRAMS_PER_STRENGTH_AND_SIZE;
        let trained = self.skills.value(MOVEMENT_SKILLS.weight_training).min(FULL);
        (u64::from(base) * u64::from(FULL + trained) / u64::from(FULL)) as u32
    }

    /// How weighed down this makes it: burdened past half what it can carry.
    pub fn load(&self, carrying: &Inventory) -> Load {
        let capacity = self.capacity(carrying.creature.def().size);
        let weight = carrying.total_weight();
        if weight > capacity {
            Load::Overloaded
        } else if weight * 2 > capacity {
            Load::Burdened
        } else {
            Load::Light
        }
    }

    /// How much energy it gains per tick: its kind's speed, slowed by what it
    /// carries and by drink (`alcohol::Stage::speed_percent`), the two
    /// stacking fully (owner, 2026-11-18), and by spells' chill
    /// (`Sheet::spell_speed_percent`), and by gear's Speed (quality bonuses).
    pub fn speed(&self, carrying: &Inventory) -> u32 {
        let kind = carrying.creature.def().speed;
        let drink = self
            .alcohol_stage()
            .map_or(100, crate::alcohol::Stage::speed_percent);
        let spells = self.spell_speed_percent();
        let gear = carrying.worn_totals().per_mille(Stat::Speed);
        let speed =
            u64::from(NORMAL_SPEED * kind / 100 * self.load(carrying).speed_percent() / 100)
                * u64::from(drink)
                / 100
                * u64::from(spells)
                / 100
                * gear
                / 1000;
        speed.clamp(1, u64::from(u32::MAX)) as u32
    }
}
