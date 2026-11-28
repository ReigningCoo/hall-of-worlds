//! Who lives in the wilderness: a few creatures to each block, by its land
//! (the livestock table's wild forms and feral herds, wolves, highland cattle:
//! `wild_shares`), from the secret seed and the block's position alone, so a
//! block let go and made again has them back as they were.

use super::herd::{Age, Herd};
use super::wild_looks::wild_looks;
use super::wild_shares::{Around, Form, Resident, shares};
use super::{Spawn, looks, roll_coins, roll_gear};
use crate::alignment::Alignment;
use crate::behaviour::Band;
use crate::climate::Biome;
use crate::dungeon::{Cell, Level};
use crate::gender::Gender;
use crate::id::Id;
use crate::livestock::{Livestock, Place};
use crate::rng::Seed;
use crate::sheet::{Attributes, Build};
use crate::wilderness::BlockPos;

#[cfg(test)]
#[path = "wild_tests.rs"]
mod tests;

/// Marks these hashes as being for wild mobs.
pub(super) const PURPOSE: i64 = 0x0077_696c;

/// In this many blocks out of 100, nothing lives.
const EMPTY_PERCENT: u64 = 45;

/// In desert blocks, this many out of 100 are empty: little lives there.
const DESERT_EMPTY_PERCENT: u64 = 75;

/// The most grown ones of a pack or herd in one block.
const MOST_TOGETHER: u64 = 3;

/// How many herds of kept animals' kinds out of 100 have young with them.
const YOUNG_PERCENT: u64 = 40;

/// The most young with a herd.
const MOST_YOUNG: u64 = 2;

/// Which of a block's rolls is its band's number.
const BAND_ROLL: i64 = 90;

/// How far a pack's members start from its den, in cells each way.
const SPREAD: i64 = 3;

/// How many places are tried for each.
const TRIES: i64 = 16;

/// What a wild block's land is like at its middle, for who lives there: its
/// biome, its height above the sea in whole metres (on surfaces that aren't
/// in metres, the height scale's own metres) and its temperature in whole °C.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Habitat {
    pub biome: Biome,
    pub metres: i32,
    pub celsius: i32,
}

impl Habitat {
    /// Low land of a biome at a middling temperature for it, where nothing
    /// more is known (the development area).
    pub fn typical(biome: Biome) -> Self {
        let celsius = match biome {
            Biome::Tundra => -5,
            Biome::BorealForest => 3,
            Biome::Grassland | Biome::Woodland => 10,
            Biome::SeasonalForest | Biome::TemperateRainforest => 14,
            Biome::Desert | Biome::Savanna | Biome::TropicalRainforest => 26,
        };
        Habitat {
            biome,
            metres: 0,
            celsius,
        }
    }
}

/// A block's creatures (`level`: its cells; `land`: what it is like at its
/// middle): none, or a pack or herd of one kind round a den on open ground,
/// chosen by `wild_shares::shares`. A herd of a kept animal's kind sometimes
/// has young (`YOUNG_PERCENT`), born in one season, their mother first.
/// Each one's number is its own across the wilderness (from a hash), and its
/// id `wild_mob_id`.
pub fn wild_spawns(seed: Seed, block: BlockPos, level: &Level, land: Habitat) -> Vec<Spawn> {
    let roll = |what: i64| seed.at(&[PURPOSE, block.x, block.y, what]);
    let lives_here = shares(land, around(level));
    let empty = if land.biome == Biome::Desert {
        DESERT_EMPTY_PERCENT
    } else {
        EMPTY_PERCENT
    };
    if lives_here.is_empty() || roll(0) % 100 < empty {
        return Vec::new();
    }
    let resident = pick(roll(1), &lives_here);
    let kind = resident.kind;
    let Some(den) = den(level, resident.place, &roll) else {
        return Vec::new();
    };
    let stages = Livestock::built_in().by_kind(kind).map(|def| &def.stages);
    let grown = 1 + roll(100) % MOST_TOGETHER;
    let young = if stages.is_some() && roll(101) % 100 < YOUNG_PERCENT {
        1 + roll(102) % MOST_YOUNG
    } else {
        0
    };
    let herd = Herd { seed, block };
    // The whole pack or herd is one band.
    let band = Band(roll(BAND_ROLL));
    let mut mobs: Vec<Spawn> = Vec::new();
    for member in 0..(grown + young) as i64 {
        let own = |what: i64| roll(1000 + member * 100 + what);
        let age = match young {
            0 => Age::Any,
            _ if member < grown as i64 => Age::Grown,
            _ => Age::Young,
        };
        let Some(number) = herd.number(member, age, stages) else {
            continue;
        };
        let spot = (0..TRIES).find_map(|attempt| {
            let span = (2 * SPREAD + 1) as u64;
            let x = den.0 + (own(attempt * 2) % span) as i64 - SPREAD;
            let y = den.1 + (own(attempt * 2 + 1) % span) as i64 - SPREAD;
            let free = mobs.iter().all(|mob| mob.position != (x, y));
            (level.contains(x, y) && open(level, x, y) && free).then_some((x, y))
        });
        let Some(position) = spot else {
            continue;
        };
        let gender = match Gender::rolled(own(61)) {
            // A herd with young is led by their mother.
            _ if young > 0 && member == 0 => Gender::Female,
            // Nothing gelds wild animals.
            Gender::Neutral if resident.form == Form::Wild => {
                [Gender::Male, Gender::Female][(own(63) % 2) as usize]
            }
            gender => gender,
        };
        let mut appearance = looks(&own, kind, gender).fitted(kind, gender);
        if resident.form == Form::Wild {
            appearance = wild_looks(kind, gender, appearance);
        }
        mobs.push(Spawn {
            number,
            kind,
            position,
            appearance,
            gender,
            alignment: Alignment::rolled(kind, own(62)),
            attributes: Attributes::for_kind(
                kind,
                gender,
                &Build::rolled(&mut Seed::new(own(60)).rng()),
            ),
            gear: roll_gear(&own, kind),
            coins: roll_coins(&own, kind),
            penned: false,
            band,
            family: None,
        });
    }
    mobs
}

/// Whether a cell is open ground a creature may start on.
fn open(level: &Level, x: i64, y: i64) -> bool {
    level.is_walkable(x, y) && matches!(level.cell(x, y), Cell::Ground | Cell::Grass)
}

/// Whether a cell is water (the sea, shallows, a river).
fn wet(level: &Level, x: i64, y: i64) -> bool {
    level.contains(x, y) && matches!(level.cell(x, y), Cell::Water | Cell::Shallows)
}

/// What a block's cells have in them that some animals need.
fn around(level: &Level) -> Around {
    let (min_x, min_y, width, height) = level.bounds();
    let mut around = Around::default();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            around.water |= wet(level, x, y);
            around.people |= level.cell(x, y).is_road();
        }
    }
    around
}

/// The resident a roll lands on among the weights.
fn pick(rolled: u64, lives_here: &[(Resident, u64)]) -> Resident {
    let total: u64 = lives_here.iter().map(|&(_, share)| share).sum();
    let mut left = rolled % total.max(1);
    lives_here
        .iter()
        .find(|&&(_, share)| {
            let here = left < share;
            left = left.saturating_sub(share);
            here
        })
        .map_or(lives_here[0].0, |&(resident, _)| resident)
}

/// Where a herd gathers: open ground, beside water for water birds.
fn den(level: &Level, place: Place, roll: &impl Fn(i64) -> u64) -> Option<(i64, i64)> {
    let (min_x, min_y, width, height) = level.bounds();
    let by_water = |x: i64, y: i64| {
        (-SPREAD..=SPREAD).any(|dy| (-SPREAD..=SPREAD).any(|dx| wet(level, x + dx, y + dy)))
    };
    (0..TRIES).find_map(|attempt| {
        let x = min_x + (roll(2 + attempt * 2) % width as u64) as i64;
        let y = min_y + (roll(3 + attempt * 2) % height as u64) as i64;
        let suits = place != Place::ByWater || by_water(x, y);
        (open(level, x, y) && suits).then_some((x, y))
    })
}

/// The id of the wild mob with this number: always the same, so nothing needs
/// saving for a mob nobody has done anything to.
pub fn wild_mob_id(seed: Seed, number: u32) -> Id {
    let hash = seed.at(&[PURPOSE, i64::from(number)]);
    let bytes = [hash.to_le_bytes(), u64::from(number).to_le_bytes()].concat();
    Id(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, &bytes))
}
