//! Kept animals (livestock plan PR 8): who lives in a farm's or town's pens
//! and pastures, by what the livestock table keeps in the land's biome.
//! Grazers in pasture, pigs in a sty, poultry in a yard (pigeons with its
//! dovecote), ducks and geese by a farm's pond. Ages come from their ids
//! (`livestock::stage_of`), so they are mixed; genders are rolled as any
//! mob's, Neutral ones looking castrated (oxen, wethers, geldings, capons).

use super::{Habitat, Spawn, spawn_at, spawns};
use crate::climate::Biome;
use crate::creature::CreatureKind::{self, *};
use crate::dungeon::{Cell, Level};
use crate::livestock::Livestock;
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::{FieldLook, PenKind, SiteLevel};

/// Grazers, which go in pasture.
const GRAZERS: [CreatureKind; 14] = [
    Cattle,
    Coo,
    WaterBuffalo,
    Yak,
    Sheep,
    Goat,
    Horse,
    Donkey,
    Mule,
    Dromedary,
    BactrianCamel,
    Llama,
    Alpaca,
    Reindeer,
];
/// Poultry for a yard; chickens much the likeliest.
const POULTRY: [CreatureKind; 4] = [Chicken, Turkey, Guineafowl, Quail];
/// Water birds, by a pond.
const WATER_BIRDS: [CreatureKind; 3] = [Duck, MuscovyDuck, Goose];
/// A sty's beasts where pigs aren't kept, and a town yard's odd one.
const SMALL_STOCK: [CreatureKind; 2] = [Goat, Sheep];

/// Where a town's kept animals' numbers start, past its beggars'.
const TOWN_NUMBERS: u32 = 1000;
/// One grazer for this many cells of pasture, and how many a field may have.
const CELLS_A_GRAZER: i64 = 16;
const GRAZERS_A_FIELD: (u64, u64) = (2, 6);
/// How far from water ducks and geese start.
const BY_WATER: i64 = 2;

/// Who is on level `index` of a site to begin with: `spawns`, with kept
/// animals on a farm's ground floor (instead of its old free-roaming ones,
/// which a ruined farm, with no pens, keeps) and in a town's yards.
pub fn site_spawns(
    seed: Seed,
    index: u32,
    site: PoiKind,
    level: &SiteLevel,
    stairs: &[(i64, i64)],
    land: Habitat,
) -> Vec<Spawn> {
    let free = || spawns(seed, index, site, &level.level, stairs);
    if index != 0 {
        return free();
    }
    if site == PoiKind::Farm {
        let kept = kept_spawns(seed, level, land, 0, true);
        return if kept.is_empty() { free() } else { kept };
    }
    if site.is_town() {
        let mut mobs = free();
        mobs.extend(kept_spawns(seed, level, land, TOWN_NUMBERS, false));
        return mobs;
    }
    free()
}

/// A kind kept in this biome, weighted: three times for its home.
fn kept_weight(kind: CreatureKind, biome: Biome) -> u64 {
    let table = Livestock::built_in();
    let Some(def) = table.iter().find(|def| def.kind == Some(kind)) else {
        return 0;
    };
    match def.kept.iter().position(|&kept| kept == biome) {
        Some(0) => 3,
        Some(_) => 1,
        None => 0,
    }
}

/// One of `kinds` kept in `biome`, by `roll`; `None` if none is.
fn choose(
    kinds: &[CreatureKind],
    biome: Biome,
    roll: u64,
    favour: CreatureKind,
) -> Option<CreatureKind> {
    let weights: Vec<(CreatureKind, u64)> = kinds
        .iter()
        .map(|&kind| {
            let times = if kind == favour { 6 } else { 1 };
            (kind, kept_weight(kind, biome) * times)
        })
        .filter(|&(_, weight)| weight > 0)
        .collect();
    let total: u64 = weights.iter().map(|&(_, weight)| weight).sum();
    let mut left = roll.checked_rem(total)?;
    weights.into_iter().find_map(|(kind, weight)| {
        let here = left < weight;
        left = left.saturating_sub(weight);
        here.then_some(kind)
    })
}

/// The kept animals in a level's pens and pastures (and by a farm's pond),
/// numbered from `first`, from their own seed.
fn kept_spawns(seed: Seed, level: &SiteLevel, land: Habitat, first: u32, farm: bool) -> Vec<Spawn> {
    let seed = seed.derive("kept animals");
    let mut mobs: Vec<Spawn> = Vec::new();
    let mut number = first;
    let mut add = |mobs: &mut Vec<Spawn>, kind, cells: &[(i64, i64)], count: u64, roll: u64| {
        let mut cells = cells.to_vec();
        cells.retain(|cell| mobs.iter().all(|mob| mob.position != *cell));
        for pick in 0..count.min(cells.len() as u64) {
            let at = (Seed::new(roll).at(&[pick as i64]) % cells.len() as u64) as usize;
            let position = cells.swap_remove(at);
            let spawn = spawn_at(seed, kind, number, position);
            mobs.push(Spawn {
                penned: true,
                ..spawn
            });
            number += 1;
        }
    };
    for (index, field) in level.fields.iter().enumerate() {
        let roll = |what: i64| seed.at(&[index as i64, what]);
        let cells = cells_of(&level.level, field);
        let biome = land.biome;
        match field.pen.map(|pen| pen.kind) {
            None if field.livestock => {
                let Some(kind) = choose(&GRAZERS, biome, roll(0), Cattle) else {
                    continue;
                };
                let room = (cells.len() as i64 / CELLS_A_GRAZER) as u64;
                let most = room.clamp(GRAZERS_A_FIELD.0, GRAZERS_A_FIELD.1);
                let count = GRAZERS_A_FIELD.0 + roll(1) % (most - 1);
                add(&mut mobs, kind, &cells, count, roll(2));
            }
            None => {}
            Some(PenKind::Sty) => {
                let kind = choose(&[Pig], biome, roll(0), Pig)
                    .or_else(|| choose(&SMALL_STOCK, biome, roll(0), Goat));
                if let Some(kind) = kind {
                    add(&mut mobs, kind, &cells, 1 + roll(1) % 3, roll(2));
                }
            }
            Some(PenKind::Yard) => {
                // A town's yard: now and then a goat or a pig instead of hens.
                let beast = !farm && roll(5) % 100 < 25;
                let kind = if beast {
                    choose(&[Pig, Goat], biome, roll(0), Goat)
                } else {
                    choose(&POULTRY, biome, roll(0), Chicken)
                };
                if let Some(kind) = kind {
                    let count = if beast {
                        1 + roll(1) % 2
                    } else {
                        3 + roll(1) % 4
                    };
                    add(&mut mobs, kind, &cells, count, roll(2));
                }
                if field.pen.is_some_and(|pen| pen.dovecote.is_some()) {
                    add(&mut mobs, Pigeon, &cells, 2 + roll(3) % 3, roll(4));
                }
            }
        }
    }
    if farm {
        let water = by_water(&level.level);
        let roll = |what: i64| seed.at(&[-1, what]);
        if !water.is_empty()
            && let Some(kind) = choose(&WATER_BIRDS, land.biome, roll(0), Duck)
        {
            add(&mut mobs, kind, &water, 2 + roll(1) % 4, roll(2));
        }
    }
    mobs
}

/// A field's cells a creature may start on: walkable and clear of a pen's
/// shelter and dovecote post.
fn cells_of(level: &Level, field: &FieldLook) -> Vec<(i64, i64)> {
    let r = field.rect;
    let built = |x: i64, y: i64| {
        field
            .pen
            .is_some_and(|pen| pen.shelter.contains(x, y) || pen.dovecote == Some((x, y)))
    };
    (r.y..r.y + r.height)
        .flat_map(|y| (r.x..r.x + r.width).map(move |x| (x, y)))
        .filter(|&(x, y)| level.is_walkable(x, y) && !built(x, y))
        .collect()
}

/// Bare ground within `BY_WATER` of a pond's water, for ducks and geese.
fn by_water(level: &Level) -> Vec<(i64, i64)> {
    let (min_x, min_y, width, height) = level.bounds();
    let wet = |x: i64, y: i64| level.contains(x, y) && level.cell(x, y) == Cell::Water;
    (min_y..min_y + height)
        .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
        .filter(|&(x, y)| level.cell(x, y) == Cell::Ground && level.is_walkable(x, y))
        .filter(|&(x, y)| {
            (-BY_WATER..=BY_WATER).any(|dy| (-BY_WATER..=BY_WATER).any(|dx| wet(x + dx, y + dy)))
        })
        .collect()
}

#[cfg(test)]
mod tests;
