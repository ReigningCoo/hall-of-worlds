//! Castles' and towers' cellar casks (`shared::cellar`): stocked once, the
//! first time their level is made, as casks lying on the ground with
//! contents rolled here (their ids are the server's), or on the rack of
//! their size under them (`shared::furnishings`). A first level is
//! played set into the wilderness, so its casks go onto the wilderness's
//! ground at the cutout's cells; the site's own level save remembers either
//! way that it has been stocked.

use shared::cask_rack::RackCasks;
use shared::cellar::{CellarCask, stock};
use shared::furniture::Furniture;
use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;
use shared::poi::Poi;
use shared::site::{Site, SiteLevel};
use shared::spot::Spot;

use crate::ServerWorld;
use crate::ids::new_id;
use crate::storage::{LevelSave, SiteId};

/// Stocks level `index` of a site (`level` as generated) if it is the
/// site's cellar and hasn't been stocked: a poi's or a developer's site.
pub(super) fn stock_cellar(
    world: &mut ServerWorld,
    (id, poi): (SiteId, &Poi),
    site: &Site,
    index: u32,
    level: &SiteLevel,
) {
    if !matches!(id, SiteId::Poi(_) | SiteId::Dev { .. }) {
        return;
    }
    stock_onto(world, (id, poi), site, index, level, (id, index), (0, 0));
}

/// Stocks a poi's first level set into the wilderness, `offset` cells away
/// (`Cutout::offset`), onto the wilderness's ground.
pub(super) fn stock_set_in_cellar(
    world: &mut ServerWorld,
    poi: &Poi,
    site: &Site,
    first: &SiteLevel,
    offset: (i64, i64),
) {
    let id = SiteId::Poi(poi.chunk);
    stock_onto(
        world,
        (id, poi),
        site,
        0,
        first,
        (SiteId::Wilderness, 0),
        offset,
    );
}

/// Puts the casks of level `index` of `id`'s site onto `onto`'s ground,
/// moved by `offset`, unless done before.
fn stock_onto(
    world: &mut ServerWorld,
    (id, poi): (SiteId, &Poi),
    site: &Site,
    index: u32,
    level: &SiteLevel,
    onto: (SiteId, u32),
    offset: (i64, i64),
) {
    let saved = world
        .sites
        .get(&id)
        .and_then(|save| save.levels.get(&index));
    if saved.is_some_and(|level| level.cellar_stocked) {
        return;
    }
    let casks = site.cellar_casks(index, level);
    if casks.is_empty() {
        return;
    }
    let saved = world.site_mut(id).levels.entry(index).or_default();
    saved.cellar_stocked = true;
    let mut rng = poi.seed.derive("cellar contents").rng();
    let now = crate::quests::unix_now().max(0) as u64;
    let mut on_ground = Vec::new();
    for cask in casks {
        let mut item = Item::new(new_id(), ItemKind::Cask);
        item.form = Some(ItemForm::Cask(stock(cask, &mut rng)));
        // On the ground or a rack, its clocks go by world time from now.
        item.lay_casks_down(now);
        match rack_under(level, cask) {
            Some(at) => put_on_rack(saved, at, item, level),
            None => on_ground.push((cask.at, item)),
        }
    }
    let ground = &mut world
        .site_mut(onto.0)
        .levels
        .entry(onto.1)
        .or_default()
        .ground;
    for ((x, y), item) in on_ground {
        let at = Spot::middle_of((x + offset.0, y + offset.1));
        ground.entry(at).or_default().push(item);
    }
}

/// The cell of the rack of `cask`'s size it lies on (`shared::furnishings`),
/// if any.
fn rack_under(level: &SiteLevel, cask: CellarCask) -> Option<(i64, i64)> {
    let piece = level.level.furniture_at(cask.at.0, cask.at.1)?;
    match piece.furniture {
        Furniture::CaskRack { rack } if rack.size == cask.size => Some(piece.at),
        _ => None,
    }
}

/// Puts `item` on the rack at `at`, kept in the site's own level save (a
/// first level's too, wherever it is played).
fn put_on_rack(saved: &mut LevelSave, at: (i64, i64), item: Item, level: &SiteLevel) {
    let Some(Furniture::CaskRack { rack }) =
        level.level.furniture_at(at.0, at.1).map(|p| p.furniture)
    else {
        return;
    };
    let casks = saved
        .racks
        .entry(at)
        .or_insert_with(|| RackCasks::new(rack));
    let _ = casks.put(item);
}
