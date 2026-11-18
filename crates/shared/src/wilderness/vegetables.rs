//! Vegetables growing wild (owner, 2026-11-17), by each world map point's biome
//! (`vegetable::wild_crops`, `wild_only_vegetables`), in patches of a few cells
//! wandering as herbs' clumps do: farmed crops gone wild 1.5 times as often as
//! wild fruit (`WILD_CROP_PER_MILLE`), the commoner more often and those only
//! farmed very rarely; wild-only plants as often as herbs (`CLUMP_TRIES` tries
//! of `CLUMP_PERCENT`). Only on open land, never on a herb, in a cutout, on its
//! border or in a gate's clearing. Each point is rolled on its own.

use std::collections::BTreeSet;

use super::herbs::clump;
use super::{BlockPos, CLUMP_PERCENT, CLUMP_TRIES, ChunkPlan, SCALE, Wilderness};
use crate::dungeon::{Cell, Level};
use crate::rng::Seed;
use crate::vegetable::{
    VegetableDef, VegetablePlant, Vegetables, choose_wild, wild_crops, wild_only_vegetables,
};

/// The chance of a patch of farmed crops gone wild on each world map point, in
/// tenths of a percent: 1.5 times wild fruit's `WILD_FRUIT_PERCENT` (owner).
pub const WILD_CROP_PER_MILLE: u64 = 15 * super::WILD_FRUIT_PERCENT;

/// The most plants in a patch of farmed crops gone wild (a wild-only plant's
/// clumps are herbs', up to `CLUMP_MOST`).
pub const WILD_PATCH_MOST: u64 = 3;

impl Wilderness {
    /// The vegetable plants growing wild in `block`, whose cells and herb
    /// patches are `cells`.
    pub fn vegetable_patches(
        &self,
        block: BlockPos,
        plan: &ChunkPlan,
        cells: &Level,
    ) -> Vec<((i64, i64), VegetablePlant)> {
        let (min_x, min_y, width, height) = self.block_bounds(block);
        let free = |x: i64, y: i64| {
            matches!(cells.cell(x, y), Cell::Ground | Cell::Grass)
                && cells.herb_at(x, y).is_none()
                && !plan.cutout().is_some_and(|cutout| cutout.contains(x, y))
                && !plan.keeps_clear(x, y)
                && !(plan.gates().iter()).any(|gate| gate.contains(self.wrap(x), y))
        };
        let vegetables = Vegetables::built_in();
        let (crops, wild_only) = (
            self.seed.derive("wild crops"),
            self.seed.derive("wild vegetables"),
        );
        let phases = self.seed.derive("wild vegetable phases");
        let mut taken = BTreeSet::new();
        let mut plants = Vec::new();
        let points_y = min_y.div_euclid(SCALE)..(min_y + height).div_euclid(SCALE);
        for py in points_y {
            for px in min_x.div_euclid(SCALE)..(min_x + width).div_euclid(SCALE) {
                let biome = self.habitat_at(px, py).biome;
                let mut rng = Seed::new(crops.at(&[px, py])).rng();
                if rng.range_u64(0, 1000) < WILD_CROP_PER_MILLE
                    && let Some(def) = choose_wild(&wild_crops(vegetables, biome), &mut rng)
                {
                    let most = rng.range_u64(1, WILD_PATCH_MOST + 1) as usize;
                    let mut cells = clump(&mut rng, (px, py), &|x, y| {
                        free(x, y) && !taken.contains(&(x, y))
                    });
                    cells.truncate(most);
                    add(&mut plants, &mut taken, def, cells, phases);
                }
                let suited = wild_only_vegetables(vegetables, biome);
                for attempt in 0..CLUMP_TRIES {
                    let mut rng = Seed::new(wild_only.at(&[px, py, attempt])).rng();
                    if suited.is_empty() || rng.range_u64(0, 100) >= CLUMP_PERCENT {
                        continue;
                    }
                    let Some(def) = choose_wild(&suited, &mut rng) else {
                        continue;
                    };
                    let cells = clump(&mut rng, (px, py), &|x, y| {
                        free(x, y) && !taken.contains(&(x, y))
                    });
                    add(&mut plants, &mut taken, def, cells, phases);
                }
            }
        }
        plants
    }
}

/// A patch's plants in `cells`, of `def`, each with a phase from its cell
/// (`phases`), noting the cells taken.
fn add(
    plants: &mut Vec<((i64, i64), VegetablePlant)>,
    taken: &mut BTreeSet<(i64, i64)>,
    def: &VegetableDef,
    cells: Vec<(i64, i64)>,
    phases: Seed,
) {
    for (x, y) in cells {
        taken.insert((x, y));
        let plant = VegetablePlant {
            vegetable: def.key.clone(),
            phase: phases.at(&[x, y]) as u32,
            picked: None,
        };
        plants.push(((x, y), plant));
    }
}

#[cfg(test)]
mod tests;
