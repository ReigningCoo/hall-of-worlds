//! What a herbalist grows: each bed (a run of `Cell::HerbBed` joined side by
//! side, one strip of a hut's patch) one herb suited to the place, chosen from
//! the site's seed and the bed's first cell.

use std::collections::BTreeSet;

use super::place::{bed_herbs, choose};
use super::{Abundance, HerbPatch, Herbs};
use crate::dungeon::{Cell, Level};
use crate::mob::Habitat;
use crate::rng::Seed;

/// Plants every bed of `level` with a herb that suits `habitat`, from `seed`
/// (the site's). Each patch's phase comes from its cell, so a bed's patches
/// grow at different times.
pub fn plant_beds(level: &mut Level, herbs: &Herbs, habitat: &Habitat, seed: Seed) {
    let suited = bed_herbs(herbs, habitat);
    let (min_x, min_y, width, height) = level.bounds();
    let mut planted = BTreeSet::new();
    let phases = seed.derive("herb bed phase");
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.cell(x, y) != Cell::HerbBed || planted.contains(&(x, y)) {
                continue;
            }
            let bed = bed_at(level, (x, y));
            let mut rng = Seed::new(seed.derive("herb bed").at(&[x, y])).rng();
            let Some(def) = choose(&suited, Abundance::bed_weight, &mut rng) else {
                return;
            };
            for &(bx, by) in &bed {
                let patch = HerbPatch {
                    herb: def.key.clone(),
                    phase: phases.at(&[bx, by]) as u32,
                    picked: None,
                };
                level.set_herb(bx, by, patch);
            }
            planted.extend(bed);
        }
    }
}

/// The bed `start` is in: every `HerbBed` joined to it side by side.
fn bed_at(level: &Level, start: (i64, i64)) -> BTreeSet<(i64, i64)> {
    let mut bed = BTreeSet::from([start]);
    let mut open = vec![start];
    while let Some((x, y)) = open.pop() {
        for next in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if level.cell(next.0, next.1) == Cell::HerbBed && bed.insert(next) {
                open.push(next);
            }
        }
    }
    bed
}

#[cfg(test)]
mod tests;
