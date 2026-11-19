//! Which wild tree (`tree::TreeKind`) a `Cell::Tree` is: rolled from the
//! climate and cover of the world map point nearest it, whether water is near,
//! and a hash of the cell, so it is the same every time and never saved until
//! felled. The wilderness's own trees go by their cell round the planet; a
//! site's (an oasis's) by its poi's point and its own cell. A block's are
//! worked out together (`tree_kinds`), for the server to keep on its level.

use std::collections::HashMap;

use super::samples::Samples;
use super::{ChunkPlan, SCALE, Wilderness};
use crate::dungeon::{Cell, Level};
use crate::poi::{Poi, PoiKind};
use crate::tree::{Setting, TreeKind};

/// Water within this many cells (a river or the sea) counts as near: willow
/// and alder grow there. Claude's guess.
pub const NEAR_WATER: i64 = 4;

impl Wilderness {
    /// The setting trees grow in at world map point `(px, py)` (`px` taken
    /// round the planet), with `near_water` said by the caller.
    pub fn tree_setting(&self, px: i64, py: i64, near_water: bool) -> Setting {
        let px = px.rem_euclid(self.points);
        let height = self.surface.sample(px, py).height;
        let climate = self.climate.sample(px, py, height);
        let cover = self.vegetation.cover(px, py, height, climate);
        Setting::of(climate, cover, near_water)
    }

    /// The kind of the wilderness's own tree at cell `(x, y)`, whose chunk's
    /// plan is `plan`.
    pub fn tree_kind_at(&self, x: i64, y: i64, plan: &ChunkPlan) -> TreeKind {
        let x = self.wrap(x);
        let nearest = |c: i64| (c + SCALE / 2).div_euclid(SCALE);
        let setting = self.tree_setting(nearest(x), nearest(y), self.near_water(x, y, plan));
        let hash = self.seed.derive("tree kinds").at(&[x, y]);
        TreeKind::roll(setting, hash)
    }

    /// The kind of every `Cell::Tree` in `level`, cells of the chunk whose
    /// plan is `plan` (a block): what `tree_kind_at` gives each, worked out
    /// together, the terrain sampled once.
    pub fn tree_kinds(&self, level: &Level, plan: &ChunkPlan) -> Vec<((i64, i64), TreeKind)> {
        let (min_x, min_y, width, height) = level.bounds();
        let r = NEAR_WATER;
        let samples = Samples::new(self, (min_x - r, min_y - r, width + 2 * r, height + 2 * r));
        let reach = NEAR_WATER as f64;
        let rivers: Vec<_> = (plan.rivers.iter())
            .filter(|river| river.near(level.bounds(), reach))
            .collect();
        let wet = |x: i64, y: i64| samples.height(x, y) < self.low_water && !samples.frozen(x, y);
        let nearest = |c: i64| (c + SCALE / 2).div_euclid(SCALE);
        let mut settings = HashMap::new();
        let mut kinds = Vec::new();
        for y in min_y..min_y + height {
            for x in min_x..min_x + width {
                if level.cell(x, y) != Cell::Tree {
                    continue;
                }
                // The samples take `x` as it is; the rest round the planet.
                let wx = self.wrap(x);
                let near_water = rivers.iter().any(|river| river.outside(wx, y) <= reach)
                    || (-r..=r).any(|dy| (-r..=r).any(|dx| wet(x + dx, y + dy)));
                let point = (nearest(wx), nearest(y), near_water);
                let setting = *settings
                    .entry(point)
                    .or_insert_with(|| self.tree_setting(point.0, point.1, near_water));
                let hash = self.seed.derive("tree kinds").at(&[wx, y]);
                kinds.push(((x, y), TreeKind::roll(setting, hash)));
            }
        }
        kinds
    }

    /// The kind of the tree at cell `(x, y)` of a site's level (its own
    /// cells), at `poi`: an oasis's trees grow by water.
    pub fn site_tree_kind(&self, poi: &Poi, x: i64, y: i64) -> TreeKind {
        let setting = self.tree_setting(poi.x, poi.y, poi.kind == PoiKind::Oasis);
        let hash = poi.seed.derive("tree kinds").at(&[x, y]);
        TreeKind::roll(setting, hash)
    }

    /// Whether a river in `plan` or the sea is within `NEAR_WATER` of `(x, y)`.
    fn near_water(&self, x: i64, y: i64, plan: &ChunkPlan) -> bool {
        let reach = NEAR_WATER as f64;
        if plan.rivers.iter().any(|river| river.outside(x, y) <= reach) {
            return true;
        }
        let r = NEAR_WATER;
        let samples = Samples::new(self, (x - r, y - r, 2 * r + 1, 2 * r + 1));
        (-r..=r).any(|dy| {
            (-r..=r).any(|dx| {
                let (cx, cy) = (x + dx, y + dy);
                samples.height(cx, cy) < self.low_water && !samples.frozen(cx, cy)
            })
        })
    }
}
