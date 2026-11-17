//! Herbs growing wild: clumps of patches on open land, chosen by each world
//! map point's biome and warmth (`herb::wild_herbs`), the commoner herbs more
//! often. Each point is rolled on its own, and its clumps stay inside its own
//! cells, so a block's patches depend only on where it is.

use super::{BlockPos, ChunkPlan, SCALE, Wilderness};
use crate::dungeon::{Cell, Level};
use crate::herb::{Abundance, HerbPatch, Herbs, choose, wild_herbs};
use crate::mob::Habitat;
use crate::rng::{Rng, Seed};
use std::collections::BTreeSet;

/// Tries for a clump on each world map point, and the chance of each, in
/// percent: about two clumps every three points. First guesses.
pub const CLUMP_TRIES: i64 = 2;
pub const CLUMP_PERCENT: u64 = 35;

/// The most patches in a clump.
pub const CLUMP_MOST: u64 = 5;

/// How far a clump's first patch is from its point's edge, in cells.
const CLUMP_MARGIN: i64 = 3;

impl Wilderness {
    /// What the land is like at world map point `(px, py)` (`px` in
    /// `0..points`): its biome, height and temperature, rounded to whole metres
    /// and degrees.
    pub fn habitat_at(&self, px: i64, py: i64) -> Habitat {
        let height = self.surface.sample(px, py).height;
        let climate = self.climate.sample(px, py, height);
        Habitat {
            biome: climate.biome(),
            metres: self.surface.metres(height),
            celsius: climate.celsius().round() as i32,
        }
    }

    /// The herb patches growing wild in `block`, whose cells are `cells`
    /// (`block(block, plan)`), with `herbs` the server's table: only on open
    /// ground and grass, never in a cutout, on its border or in a gate's
    /// clearing.
    pub fn herb_patches(
        &self,
        block: BlockPos,
        plan: &ChunkPlan,
        cells: &Level,
        herbs: &Herbs,
    ) -> Vec<((i64, i64), HerbPatch)> {
        let (min_x, min_y, width, height) = self.block_bounds(block);
        let free = |x: i64, y: i64| {
            matches!(cells.cell(x, y), Cell::Ground | Cell::Grass)
                && !plan.cutout().is_some_and(|cutout| cutout.contains(x, y))
                && !plan.keeps_clear(x, y)
                && !plan
                    .gates()
                    .iter()
                    .any(|gate| gate.contains(self.wrap(x), y))
        };
        let seeds = Seeds {
            clumps: self.seed.derive("herb clumps"),
            phases: self.seed.derive("herb phases"),
        };
        let mut taken = BTreeSet::new();
        let mut patches = Vec::new();
        let points_y = min_y.div_euclid(SCALE)..(min_y + height).div_euclid(SCALE);
        for py in points_y {
            for px in min_x.div_euclid(SCALE)..(min_x + width).div_euclid(SCALE) {
                let suited = wild_herbs(herbs, &self.habitat_at(px, py));
                for attempt in 0..CLUMP_TRIES {
                    let mut rng = Seed::new(seeds.clumps.at(&[px, py, attempt])).rng();
                    if suited.is_empty() || rng.range_u64(0, 100) >= CLUMP_PERCENT {
                        continue;
                    }
                    let Some(def) = choose(&suited, Abundance::wild_weight, &mut rng) else {
                        continue;
                    };
                    for (x, y) in clump(&mut rng, (px, py), &|x, y| {
                        free(x, y) && !taken.contains(&(x, y))
                    }) {
                        taken.insert((x, y));
                        let patch = HerbPatch {
                            herb: def.key.clone(),
                            phase: seeds.phases.at(&[x, y]) as u32,
                            picked: None,
                        };
                        patches.push(((x, y), patch));
                    }
                }
            }
        }
        patches
    }
}

/// A clump's cells in point `(px, py)`: a short wander from a first cell
/// away from the point's edge, taking each cell `free` says it may.
fn clump(rng: &mut Rng, (px, py): (i64, i64), free: &dyn Fn(i64, i64) -> bool) -> Vec<(i64, i64)> {
    let span = (SCALE - 2 * CLUMP_MARGIN) as u64;
    let mut at = (
        px * SCALE + CLUMP_MARGIN + rng.range_u64(0, span) as i64,
        py * SCALE + CLUMP_MARGIN + rng.range_u64(0, span) as i64,
    );
    let size = rng.range_u64(1, CLUMP_MOST + 1) as usize;
    let within = |x: i64, y: i64| x.div_euclid(SCALE) == px && y.div_euclid(SCALE) == py;
    let mut clump = Vec::new();
    for _ in 0..size * 3 {
        if clump.len() >= size {
            break;
        }
        if free(at.0, at.1) && !clump.contains(&at) {
            clump.push(at);
        }
        let (dx, dy) = STEPS[rng.range_u64(0, STEPS.len() as u64) as usize];
        if within(at.0 + dx, at.1 + dy) {
            at = (at.0 + dx, at.1 + dy);
        }
    }
    clump
}

/// The seeds patches are rolled from.
struct Seeds {
    clumps: Seed,
    phases: Seed,
}

/// The ways a clump wanders from one patch to the next.
const STEPS: [(i64, i64); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[cfg(test)]
mod tests;
