//! `GEAR_STATS`: every stat a quality bonus on gear can be, what each gives
//! at its usual size, and which places it suits. One list, so anything new
//! that potions or spells give is added here too (owner, 2026-11-27).

use super::MILLI;
use super::stat::{Amount, PER_CENT, Stat};
use crate::creature::Location;
use crate::sheet::Attribute::{
    self, Agility, Aura, Empathy, Intellect, Perception, Presence, Strength, Toughness, Will,
};
use crate::vitals::Vital::{Health, Mana};

/// Places a gear stat suits, as bits: an item may roll it when any place it
/// is worn has one of its bits (`places_of`).
pub type Places = u16;

pub const HEAD: Places = 1;
pub const NECK: Places = 1 << 1;
pub const SHOULDERS: Places = 1 << 2;
pub const ARMS: Places = 1 << 3;
pub const TORSO: Places = 1 << 4;
pub const WAIST: Places = 1 << 5;
pub const HANDS: Places = 1 << 6;
pub const FINGERS: Places = 1 << 7;
pub const LEGS: Places = 1 << 8;
pub const FEET: Places = 1 << 9;
pub const BACK: Places = 1 << 10;
/// Held to fight with: weapons.
pub const HELD: Places = 1 << 11;

/// One stat gear can roll, and how much a bonus of it usually gives, in the
/// good direction (less harm taken is good): uniques get twice as much.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GearStat {
    pub stat: Stat,
    pub amount: Amount,
    pub places: Places,
}

/// +5% of an attribute: the usual size of a percentage bonus (first guess).
pub const PERCENT_BONUS: i32 = 5 * PER_CENT;
/// +20 most Health or Mana (first guess).
pub const MOST_BONUS: i32 = 20 * MILLI as i32;
/// +0.2 Health or Mana a second while refilling (first guess).
pub const RESTORE_BONUS: i32 = 200;
/// +1 cell of Darkvision or light (first guess).
pub const CELL_BONUS: i32 = MILLI as i32;
/// +1 level of a skill: one percentage point, a value of 1.00, in thousandths
/// of its hundredths (`skill::VALUE_SCALE`) (first guess).
pub const SKILL_LEVEL: i32 = 100 * MILLI as i32;

const fn attribute(attribute: Attribute, places: Places) -> GearStat {
    percent(Stat::Attribute(attribute), places)
}

const fn percent(stat: Stat, places: Places) -> GearStat {
    GearStat {
        stat,
        amount: Amount::Percent(PERCENT_BONUS),
        places,
    }
}

const fn flat(stat: Stat, milli: i32, places: Places) -> GearStat {
    GearStat {
        stat,
        amount: Amount::Flat(milli),
        places,
    }
}

/// Every stat gear can roll but skills, which come from what the item is
/// (`gear_skills`). Wet, chill, Calm and Levitate are states, not numbers,
/// so never here. Order matters: rolls pick by place in this list, so new
/// ones go at the end.
pub const GEAR_STATS: &[GearStat] = &[
    attribute(Strength, HELD | ARMS | HANDS | TORSO | WAIST),
    attribute(Agility, HELD | HANDS | LEGS | FEET | WAIST),
    attribute(Toughness, TORSO | SHOULDERS | LEGS | NECK | WAIST | FEET),
    attribute(Intellect, HEAD | FINGERS | NECK),
    attribute(Perception, HEAD | HELD | FINGERS | NECK),
    attribute(Will, HEAD | NECK | TORSO | FINGERS),
    attribute(Presence, HEAD | SHOULDERS | NECK | FINGERS | BACK),
    attribute(Empathy, NECK | HANDS | FINGERS | BACK),
    attribute(Aura, FINGERS | NECK | HEAD | BACK),
    flat(
        Stat::Most(Health),
        MOST_BONUS,
        TORSO | SHOULDERS | WAIST | LEGS | NECK | FINGERS,
    ),
    flat(
        Stat::Most(Mana),
        MOST_BONUS,
        HEAD | FINGERS | NECK | BACK | ARMS,
    ),
    flat(Stat::Restore(Health), RESTORE_BONUS, TORSO | NECK | FINGERS),
    flat(Stat::Restore(Mana), RESTORE_BONUS, HEAD | FINGERS | BACK),
    flat(Stat::Darkvision, CELL_BONUS, HEAD | FINGERS),
    flat(Stat::Light, CELL_BONUS, HEAD | FINGERS | HELD),
    percent(Stat::Speed, FEET | LEGS | HELD),
    percent(
        Stat::Defence,
        HELD | ARMS | HANDS | SHOULDERS | TORSO | HEAD | FEET,
    ),
    percent(Stat::BlowHarm, HELD | HANDS | ARMS),
    GearStat {
        stat: Stat::HarmTaken,
        amount: Amount::Percent(-PERCENT_BONUS),
        places: TORSO | HEAD | SHOULDERS | LEGS | WAIST | NECK | FEET | ARMS,
    },
    percent(Stat::Resistance, FINGERS | NECK | BACK | HEAD | TORSO),
];

/// The places an item worn at these locations counts as. Beasts' legs are
/// legs, and their wings and tails their back.
pub fn places_of(locations: &[Location]) -> Places {
    locations.iter().fold(0, |places, location| {
        places
            | match location {
                Location::Head => HEAD,
                Location::Neck => NECK,
                Location::Shoulders => SHOULDERS,
                Location::Arms => ARMS,
                Location::Torso => TORSO,
                Location::Waist => WAIST,
                Location::Hands => HANDS,
                Location::Fingers => FINGERS,
                Location::Legs | Location::Forelegs | Location::Hindlegs => LEGS,
                Location::Feet => FEET,
                Location::Back | Location::Wings | Location::Tail => BACK,
                Location::Held => HELD,
            }
    })
}
