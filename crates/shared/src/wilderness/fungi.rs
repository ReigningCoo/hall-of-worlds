//! Fungi growing wild above ground (owner, 2026-11-19, decision 5): clumps as
//! herbs' (`herbs::clump`), a quarter as common, of fungi suiting each world
//! map point's biome and warmth (`fungus::place`); lichens as common as herbs
//! in Tundra and Boreal forest. Fungi of trees and roots grow only beside a
//! tree. Wood-feeders also grow on felled trees (`fungus::felled_patch`), kept
//! on every tree's cell and shown only while it is felled and cleared.

use std::collections::{BTreeMap, BTreeSet};

use super::herbs::clump;
use super::{BlockPos, CLUMP_PERCENT, CLUMP_TRIES, ChunkPlan, SCALE, Wilderness};
use crate::climate::Biome;
use crate::dungeon::{Cell, Level};
use crate::fungus::{Fungi, FungusDef, FungusPatch, Niche, Spot, choose, felled_patch, suited};
use crate::mob::Habitat;
use crate::rng::Seed;

/// The chance of a clump on each world map point, in percent: one try, so a
/// quarter of herbs' (`CLUMP_TRIES` × `CLUMP_PERCENT`).
pub const FUNGUS_CLUMP_PERCENT: u64 = CLUMP_TRIES as u64 * CLUMP_PERCENT / 4;

/// What open ground offers anywhere above ground.
const OPEN: [Niche; 5] = [
    Niche::Soil,
    Niche::Dung,
    Niche::Rock,
    Niche::Insects,
    Niche::Termites,
];

/// What a cell beside a tree offers too.
const BY_TREES: [Niche; 3] = [Niche::Trees, Niche::Roots, Niche::Wood];

impl Wilderness {
    /// The fungus patches growing wild in `block`, whose cells are `cells`
    /// with its herbs and vegetables already set (`block(block, plan)`): on
    /// open ground and grass free of them, never in a cutout, on its border
    /// or in a gate's clearing; and on its trees for when they are felled.
    pub fn fungus_patches(
        &self,
        block: BlockPos,
        plan: &ChunkPlan,
        cells: &Level,
        fungi: &Fungi,
    ) -> Vec<((i64, i64), FungusPatch)> {
        let (min_x, min_y, width, height) = self.block_bounds(block);
        let outside_site = |x: i64, y: i64| {
            !plan.cutout().is_some_and(|cutout| cutout.contains(x, y))
                && !plan.keeps_clear(x, y)
                && !(plan.gates().iter()).any(|gate| gate.contains(self.wrap(x), y))
        };
        let free = |x: i64, y: i64| {
            matches!(cells.cell(x, y), Cell::Ground | Cell::Grass)
                && cells.herb_at(x, y).is_none()
                && cells.vegetable_at(x, y).is_none()
                && outside_site(x, y)
        };
        let clumps = self.seed.derive("fungus clumps");
        let phases = self.seed.derive("fungus phases");
        let mut habitats = BTreeMap::new();
        let mut taken = BTreeSet::new();
        let mut patches = Vec::new();
        for py in min_y.div_euclid(SCALE)..(min_y + height).div_euclid(SCALE) {
            for px in min_x.div_euclid(SCALE)..(min_x + width).div_euclid(SCALE) {
                let habitat = self.habitat_at(px, py);
                for (attempt, lichens) in tries(habitat.biome) {
                    let mut rng = Seed::new(clumps.at(&[px, py, attempt])).rng();
                    let percent = if lichens {
                        CLUMP_PERCENT
                    } else {
                        FUNGUS_CLUMP_PERCENT
                    };
                    if rng.range_u64(0, 100) >= percent {
                        continue;
                    }
                    let options = surface_fungi(fungi, &habitat, lichens);
                    let Some(def) = choose(&options, 0, &mut rng) else {
                        continue;
                    };
                    let fits = |x: i64, y: i64| {
                        free(x, y) && !taken.contains(&(x, y)) && grows_beside(def, cells, x, y)
                    };
                    for (x, y) in clump(&mut rng, (px, py), &fits) {
                        taken.insert((x, y));
                        let patch = FungusPatch {
                            fungus: def.key.clone(),
                            phase: phases.at(&[x, y]) as u32,
                            picked: None,
                        };
                        patches.push(((x, y), patch));
                    }
                }
                habitats.insert((px, py), habitat);
            }
        }
        let felled = self.seed.derive("felled fungi");
        for y in min_y..min_y + height {
            for x in min_x..min_x + width {
                if cells.cell(x, y) != Cell::Tree || !outside_site(x, y) {
                    continue;
                }
                let point = (x.div_euclid(SCALE), y.div_euclid(SCALE));
                let habitat =
                    (habitats.entry(point)).or_insert_with(|| self.habitat_at(point.0, point.1));
                if let Some(patch) = felled_patch(fungi, habitat, felled, (x, y)) {
                    patches.push(((x, y), patch));
                }
            }
        }
        patches
    }
}

/// The tries for clumps on a point: one of any fungus, and in Tundra and
/// Boreal forest as many of lichens as herbs get.
fn tries(biome: Biome) -> Vec<(i64, bool)> {
    let mut tries = vec![(0, false)];
    if matches!(biome, Biome::Tundra | Biome::BorealForest) {
        tries.extend((1..=CLUMP_TRIES).map(|attempt| (attempt, true)));
    }
    tries
}

/// The fungi growing above ground at `habitat` (lichens only, if `lichens`).
fn surface_fungi<'a>(fungi: &'a Fungi, habitat: &Habitat, lichens: bool) -> Vec<&'a FungusDef> {
    let spot = Spot {
        niches: OPEN.iter().chain(&BY_TREES).copied().collect(),
        ..Spot::default()
    };
    let mut found = suited(fungi, &spot, habitat);
    if lichens {
        found.retain(|def| def.is_lichen());
    }
    found
}

/// Whether `def` can grow on `(x, y)`: on open ground, or, for fungi of trees
/// and roots alone, beside a tree.
fn grows_beside(def: &FungusDef, cells: &Level, x: i64, y: i64) -> bool {
    if def.niches.iter().any(|niche| OPEN.contains(niche)) {
        return true;
    }
    (-1..=1).any(|dy| (-1..=1).any(|dx| cells.cell(x + dx, y + dy) == Cell::Tree))
}

#[cfg(test)]
mod tests;
