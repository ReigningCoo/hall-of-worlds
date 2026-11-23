//! Herb patches (`shared::herb`), fruit plants (`shared::fruit`), vegetable
//! plants (`shared::vegetable`) and fungi (`shared::fungus`, `fungi.rs`) on a
//! visit's level: what a site grows (herbalists' beds, farms' orchards and
//! fields of crops, fungi underground and on felled trees), where
//! each plant's harvests are kept (a site's set into the wilderness are its
//! site's, through the `Mirror`, as doors are), and telling everyone on a
//! level when one is harvested. A cell has one plant at most, so harvests are
//! kept by cell alike.

use shared::dungeon::Level;
use shared::fruit::{FruitPlant, Fruits, plant_orchards};
use shared::fungus::FungusPatch;
use shared::herb::{HerbPatch, plant_beds};
use shared::poi::Poi;
use shared::site::{Site, SiteLevel};
use shared::vegetable::{VegetablePlant, Vegetables, plant_fields};

use super::doors::changed_at;
use super::{Mirror, Visit, update_sight};
use crate::ServerWorld;
use crate::connections::Client;
use crate::storage::SiteId;

/// Where a plant's harvests are kept: the site, level and cell it belongs to.
pub(crate) type PlantKey = (SiteId, u32, (i64, i64));

/// A plant just harvested, for `share_plants`: where it is kept, and when.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PlantChange {
    pub(crate) key: PlantKey,
    pub(crate) picked: u64,
}

/// What a site's level grows, in its own cells.
#[derive(Clone, Debug, Default)]
pub(crate) struct SitePlants {
    pub(crate) herbs: Vec<((i64, i64), HerbPatch)>,
    pub(crate) fruit: Vec<((i64, i64), FruitPlant)>,
    pub(crate) vegetables: Vec<((i64, i64), VegetablePlant)>,
    pub(crate) fungi: Vec<((i64, i64), FungusPatch)>,
}

/// What a site's level grows: herbs in its beds (`plant_beds`), fruit in its
/// orchards (`plant_orchards`) and a crop in each other field of crops
/// (`plant_fields`), suiting the land at its poi, from its seed, and fungi
/// (`site_fungi`) on level `index` of `site`.
pub(crate) fn site_plants(
    world: &ServerWorld,
    (poi, site): (&Poi, &Site),
    index: u32,
    level: &SiteLevel,
) -> SitePlants {
    let habitat = world.habitat_of_poi(poi);
    let mut planted = level.level.clone();
    plant_beds(&mut planted, &world.herbs, &habitat, poi.seed);
    let herbs = (planted.herbs())
        .map(|(at, patch)| (at, patch.clone()))
        .collect();
    let fruit = plant_orchards(Fruits::built_in(), &level.fields, habitat.celsius, poi.seed);
    let vegetables = plant_fields(
        Vegetables::built_in(),
        &level.fields,
        habitat.biome,
        poi.seed,
    );
    let fungi = fungi::site_fungi(poi, site, index, level, &habitat);
    SitePlants {
        herbs,
        fruit,
        vegetables,
        fungi,
    }
}

/// Puts what level `index` of site `id` (at `poi`) grows on it, each as last
/// harvested, its trees' kinds, and how far its felled trees have grown back.
pub(crate) fn put_plants(
    world: &mut ServerWorld,
    (id, poi, site): (SiteId, &Poi, &Site),
    index: u32,
    level: &mut SiteLevel,
) {
    // Nothing grows in The Way Inn.
    if id.is_inn() {
        return;
    }
    let plants = site_plants(world, (poi, site), index, level);
    for ((x, y), patch) in plants.herbs {
        level.level.set_herb(x, y, patch);
    }
    for ((x, y), plant) in plants.fruit {
        level.level.set_fruit(x, y, plant);
    }
    for ((x, y), plant) in plants.vegetables {
        level.level.set_vegetable(x, y, plant);
    }
    for ((x, y), patch) in plants.fungi {
        level.level.set_fungus(x, y, patch);
    }
    let key = |x: i64, y: i64| plant_key(id, index, &[], x, x, y);
    for ((x, y), when) in picked_of(world, &level.level, key) {
        set_picked(&mut level.level, x, y, when);
    }
    // Its trees' kinds, rolled by its poi.
    let (min_x, min_y, width, height) = level.level.bounds();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.level.cell(x, y) == shared::dungeon::Cell::Tree {
                let kind = world.wilderness.wild.site_tree_kind(poi, x, y);
                level.level.set_tree_kind(x, y, Some(kind));
            }
        }
    }
    // Felled trees too, kept by cell alike.
    let bounds = level.level.bounds();
    let cell_of = |key| super::trees::cell_of(key, (id, index), None, &[], bounds);
    super::trees::put_felled(world, &mut level.level, &[(id, index)], cell_of);
}

/// Where the plant on cell `(x, y)` of level `index` of site `id` is kept
/// (`kept_x` is `x` round the planet, in the wilderness): in the wilderness, a
/// cutout's cell is its site's.
pub(crate) fn plant_key(
    id: SiteId,
    index: u32,
    mirrors: &[Mirror],
    x: i64,
    kept_x: i64,
    y: i64,
) -> PlantKey {
    if id == SiteId::Wilderness
        && let Some((mirror, at)) = (mirrors.iter()).find_map(|m| m.of(x, y).map(|at| (m, at)))
    {
        return (mirror.id, mirror.index, at);
    }
    (id, index, (kept_x, y))
}

/// Every cell of `level` with a herb patch, fruit plant, vegetable plant or
/// fungus patch (growing now or not).
fn plant_cells(level: &Level) -> impl Iterator<Item = (i64, i64)> + '_ {
    (level.herbs().map(|(at, _)| at))
        .chain(level.fruit_plants().map(|(at, _)| at))
        .chain(level.vegetable_plants().map(|(at, _)| at))
        .chain(level.kept_fungi().map(|(at, _)| at))
}

/// Notes the plant at `(x, y)` (a herb, fruit, vegetable or fungus) harvested at game minute `when`.
pub(crate) fn set_picked(level: &mut Level, x: i64, y: i64, when: u64) {
    level.set_herb_picked(x, y, Some(when));
    level.set_fruit_picked(x, y, Some(when));
    level.set_vegetable_picked(x, y, Some(when));
    level.set_fungus_picked(x, y, Some(when));
}

/// When each plant on `level` was last harvested, by its cell there, as saved
/// where `key` says each is kept.
pub(crate) fn picked_of(
    world: &mut ServerWorld,
    level: &Level,
    key: impl Fn(i64, i64) -> PlantKey,
) -> Vec<((i64, i64), u64)> {
    let mut found = Vec::new();
    for (x, y) in plant_cells(level) {
        let (site, index, at) = key(x, y);
        let saved = world.site(site).levels.get(&index);
        if let Some(&when) = saved.and_then(|level| level.plants_picked.get(&at)) {
            found.push(((x, y), when));
        }
    }
    found
}

impl Visit {
    /// The herb patch at `(x, y)` on its level, if any.
    pub(crate) fn herb_at(&self, x: i64, y: i64) -> Option<&HerbPatch> {
        self.level.level.herb_at(x, y)
    }

    /// The fruit plant at `(x, y)` on its level, if any.
    pub(crate) fn fruit_at(&self, x: i64, y: i64) -> Option<&FruitPlant> {
        self.level.level.fruit_at(x, y)
    }

    /// The vegetable plant at `(x, y)` on its level, if any.
    pub(crate) fn vegetable_at(&self, x: i64, y: i64) -> Option<&VegetablePlant> {
        self.level.level.vegetable_at(x, y)
    }

    /// The fungus patch growing at `(x, y)` on its level now, if any.
    pub(crate) fn fungus_at(&self, x: i64, y: i64) -> Option<&FungusPatch> {
        self.level.level.fungus_at(x, y)
    }

    /// Where the plant at `(x, y)` on its level is kept.
    pub(crate) fn plant_key(&self, x: i64, y: i64) -> PlantKey {
        plant_key(self.id, self.index, &self.mirrors, x, self.kept_at(x), y)
    }

    /// Notes the plant at `(x, y)` harvested at game minute `when`, to be sent
    /// again.
    pub(crate) fn plant_picked(&mut self, x: i64, y: i64, when: u64) {
        set_picked(&mut self.level.level, x, y, when);
        changed_at(self, x, y);
    }
}

/// Keeps a harvest of the plant at `key` (at game minute `when`) in its
/// site's save, and has it shared with everyone else on its level.
pub(crate) fn keep_harvest(world: &mut ServerWorld, key: PlantKey, when: u64) {
    let (site, index, at) = key;
    let saved = world.site_mut(site).levels.entry(index).or_default();
    saved.plants_picked.insert(at, when);
    world.plant_changes.push(PlantChange { key, picked: when });
}

/// Every tick: tells everyone whose level has a plant just harvested, by
/// setting it on their level (sent again if they can see it).
pub(crate) fn share_plants(clients: &mut [Client], world: &mut ServerWorld) {
    let changes = std::mem::take(&mut world.plant_changes);
    if changes.is_empty() {
        return;
    }
    for client in clients.iter_mut() {
        let Some(visit) = &mut client.visit else {
            continue;
        };
        let level = &visit.level.level;
        let picked_at = |x: i64, y: i64| {
            (level.herb_at(x, y).map(|patch| patch.picked))
                .or_else(|| level.fruit_at(x, y).map(|plant| plant.picked))
                .or_else(|| level.vegetable_at(x, y).map(|plant| plant.picked))
                .or_else(|| level.kept_fungus_at(x, y).map(|patch| patch.picked))
                .flatten()
        };
        let here: Vec<((i64, i64), u64)> = plant_cells(level)
            .filter_map(|(x, y)| {
                let key = visit.plant_key(x, y);
                let change = changes.iter().find(|change| change.key == key)?;
                (picked_at(x, y) != Some(change.picked)).then_some(((x, y), change.picked))
            })
            .collect();
        for &((x, y), when) in &here {
            visit.plant_picked(x, y, when);
        }
        if !here.is_empty() && !update_sight(client) {
            // Gone: noticed when its messages are next read.
            continue;
        }
    }
}

mod fungi;

#[cfg(test)]
mod fruit_tests;
#[cfg(test)]
mod fungus_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod vegetable_tests;
