//! Fruit plants growing wild: very rare (owner, 2026-11-17), one try on each
//! world map point for a single plant, chosen by the point's biome and warmth
//! (`fruit::wild_fruits`), the commoner fruits more often. Only on open land
//! with open land all round, so a trunk never closes a way, and never on a
//! herb patch or vegetable.

use super::{BlockPos, ChunkPlan, SCALE, Wilderness};
use crate::dungeon::{Cell, Level};
use crate::fruit::{Abundance, FruitPlant, Fruits, choose, wild_fruits};
use crate::rng::Seed;

/// The chance of a fruit plant on each world map point, in percent: about
/// one every 33 points. A first guess.
pub const WILD_FRUIT_PERCENT: u64 = 3;

/// How far a plant is from its point's edge, in cells.
const MARGIN: i64 = 2;

impl Wilderness {
    /// The fruit plants growing wild in `block`, whose cells and herb patches
    /// are `cells`: only on open ground and grass with the same all round,
    /// never in a cutout, on its border, in a gate's clearing or on a herb.
    pub fn fruit_plants(
        &self,
        block: BlockPos,
        plan: &ChunkPlan,
        cells: &Level,
    ) -> Vec<((i64, i64), FruitPlant)> {
        let (min_x, min_y, width, height) = self.block_bounds(block);
        let open = |x: i64, y: i64| {
            matches!(cells.cell(x, y), Cell::Ground | Cell::Grass)
                && !plan.cutout().is_some_and(|cutout| cutout.contains(x, y))
                && !plan.keeps_clear(x, y)
                && !(plan.gates().iter()).any(|gate| gate.contains(self.wrap(x), y))
        };
        let free = |x: i64, y: i64| {
            cells.herb_at(x, y).is_none()
                && cells.vegetable_at(x, y).is_none()
                && (-1..=1).all(|dy| (-1..=1).all(|dx| open(x + dx, y + dy)))
        };
        let tries = self.seed.derive("wild fruit");
        let phases = self.seed.derive("wild fruit phase");
        let mut plants = Vec::new();
        let points_y = min_y.div_euclid(SCALE)..(min_y + height).div_euclid(SCALE);
        for py in points_y {
            for px in min_x.div_euclid(SCALE)..(min_x + width).div_euclid(SCALE) {
                let mut rng = Seed::new(tries.at(&[px, py])).rng();
                if rng.range_u64(0, 100) >= WILD_FRUIT_PERCENT {
                    continue;
                }
                let span = (SCALE - 2 * MARGIN) as u64;
                let x = px * SCALE + MARGIN + rng.range_u64(0, span) as i64;
                let y = py * SCALE + MARGIN + rng.range_u64(0, span) as i64;
                if !free(x, y) {
                    continue;
                }
                let suited = wild_fruits(Fruits::built_in(), &self.habitat_at(px, py));
                let Some(def) = choose(&suited, Abundance::wild_weight, &mut rng) else {
                    continue;
                };
                let plant = FruitPlant {
                    fruit: def.key.clone(),
                    phase: phases.at(&[x, y]) as u32,
                    picked: None,
                };
                plants.push(((x, y), plant));
            }
        }
        plants
    }
}

#[cfg(test)]
mod tests;
