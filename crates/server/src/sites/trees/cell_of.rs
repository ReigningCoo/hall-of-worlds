//! Where a kept cell (`PlantKey`) is on a level: the other way from
//! `plant_key`. On the level's own map, the cell itself (in the wilderness,
//! the one of its columns round the planet inside the level); in a cutout set
//! into the wilderness, back through its `Mirror`.

use crate::sites::plants::PlantKey;
use crate::sites::{Mirror, Visit};
use crate::storage::SiteId;

/// The cell on a level of `own` (site and level), covering `bounds`, that
/// `key` is kept at, if any: `wraps` is the planet's width in the wilderness.
pub(in crate::sites) fn cell_of(
    (site, index, (x, y)): PlantKey,
    own: (SiteId, u32),
    wraps: Option<i64>,
    mirrors: &[Mirror],
    (min_x, min_y, width, height): (i64, i64, i64, i64),
) -> Option<(i64, i64)> {
    for mirror in mirrors.iter().filter(|m| (m.id, m.index) == (site, index)) {
        let cell = (x - mirror.offset.0, y - mirror.offset.1);
        if mirror.of(cell.0, cell.1) == Some((x, y)) {
            return Some(cell);
        }
    }
    if (site, index) != own || !(min_y..min_y + height).contains(&y) {
        return None;
    }
    let x = match wraps {
        Some(planet) => min_x + (x - min_x).rem_euclid(planet),
        None => x,
    };
    (min_x..min_x + width).contains(&x).then_some((x, y))
}

impl Visit {
    /// The cell of its level that `key` is kept at, if it is on it.
    pub(crate) fn cell_of(&self, key: PlantKey) -> Option<(i64, i64)> {
        let own = (self.id, self.index);
        let bounds = self.level.level.bounds();
        cell_of(key, own, self.wraps_at(), &self.mirrors, bounds)
    }
}
