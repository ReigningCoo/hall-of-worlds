//! Herb patches on a visit's level (`shared::herb`): what a site's beds grow,
//! where each patch's harvests are kept (a herbalist's beds set into the
//! wilderness are its site's, through the `Mirror`, as doors are), and telling
//! everyone on a level when one is harvested.

use shared::dungeon::Level;
use shared::herb::{HerbPatch, plant_beds};
use shared::poi::Poi;

use super::doors::changed_at;
use super::{Mirror, Visit, update_sight};
use crate::ServerWorld;
use crate::connections::Client;
use crate::storage::SiteId;

/// Where a patch's harvests are kept: the site, level and cell it belongs to.
pub(crate) type HerbKey = (SiteId, u32, (i64, i64));

/// A patch just harvested, for `share_herbs`: where it is kept, and when.
#[derive(Clone, Copy, Debug)]
pub(crate) struct HerbChange {
    pub(crate) key: HerbKey,
    pub(crate) picked: u64,
}

/// What a site's first level of beds grows (`plant_beds`), in its own cells:
/// herbs suiting the land at its poi, from its seed. Nothing on levels with
/// no beds.
pub(crate) fn site_herbs(
    world: &ServerWorld,
    poi: &Poi,
    level: &Level,
) -> Vec<((i64, i64), HerbPatch)> {
    let wild = &world.wilderness.wild;
    let habitat = wild.habitat_at(poi.x.rem_euclid(wild.points()), poi.y);
    let mut planted = level.clone();
    plant_beds(&mut planted, &world.herbs, &habitat, poi.seed);
    planted
        .herbs()
        .map(|(at, patch)| (at, patch.clone()))
        .collect()
}

/// Where the patch on cell `(x, y)` of level `index` of site `id` is kept
/// (`kept_x` is `x` round the planet, in the wilderness): in the wilderness, a
/// cutout's cell is its site's.
pub(crate) fn herb_key(
    id: SiteId,
    index: u32,
    mirrors: &[Mirror],
    x: i64,
    kept_x: i64,
    y: i64,
) -> HerbKey {
    if id == SiteId::Wilderness
        && let Some((mirror, at)) = (mirrors.iter()).find_map(|m| m.of(x, y).map(|at| (m, at)))
    {
        return (mirror.id, mirror.index, at);
    }
    (id, index, (kept_x, y))
}

/// When each patch on `level` was last harvested, by its cell there, as saved
/// where `key` says each is kept.
pub(crate) fn picked_of(
    world: &mut ServerWorld,
    level: &Level,
    key: impl Fn(i64, i64) -> HerbKey,
) -> Vec<((i64, i64), u64)> {
    let mut found = Vec::new();
    for ((x, y), _) in level.herbs() {
        let (site, index, at) = key(x, y);
        let saved = world.site(site).levels.get(&index);
        if let Some(&when) = saved.and_then(|level| level.herbs_picked.get(&at)) {
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

    /// Where the patch at `(x, y)` on its level is kept.
    pub(crate) fn herb_key(&self, x: i64, y: i64) -> HerbKey {
        herb_key(self.id, self.index, &self.mirrors, x, self.kept_at(x), y)
    }

    /// Notes the patch at `(x, y)` harvested at game minute `when`, to be sent
    /// again.
    pub(crate) fn herb_picked(&mut self, x: i64, y: i64, when: u64) {
        self.level.level.set_herb_picked(x, y, Some(when));
        changed_at(self, x, y);
    }
}

/// Keeps a harvest of the patch at `key` (at game minute `when`) in its
/// site's save, and has it shared with everyone else on its level.
pub(crate) fn keep_harvest(world: &mut ServerWorld, key: HerbKey, when: u64) {
    let (site, index, at) = key;
    let saved = world.site_mut(site).levels.entry(index).or_default();
    saved.herbs_picked.insert(at, when);
    world.herb_changes.push(HerbChange { key, picked: when });
}

/// Every tick: tells everyone whose level has a patch just harvested, by
/// setting it on their level (sent again if they can see it).
pub(crate) fn share_herbs(clients: &mut [Client], world: &mut ServerWorld) {
    let changes = std::mem::take(&mut world.herb_changes);
    if changes.is_empty() {
        return;
    }
    for client in clients.iter_mut() {
        let Some(visit) = &mut client.visit else {
            continue;
        };
        let here: Vec<((i64, i64), u64)> = (visit.level.level.herbs())
            .filter_map(|((x, y), patch)| {
                let key = visit.herb_key(x, y);
                let change = changes.iter().find(|change| change.key == key)?;
                (patch.picked != Some(change.picked)).then_some(((x, y), change.picked))
            })
            .collect();
        for &((x, y), when) in &here {
            visit.herb_picked(x, y, when);
        }
        if !here.is_empty() && !update_sight(client) {
            // Gone: noticed when its messages are next read.
            continue;
        }
    }
}

#[cfg(test)]
mod tests;
