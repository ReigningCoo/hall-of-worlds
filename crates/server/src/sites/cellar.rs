//! Castles' and towers' cellar casks (`shared::cellar`): stocked once, the
//! first time their level is made, as casks lying on the ground with
//! contents rolled here (their ids are the server's). A first level is
//! played set into the wilderness, so its casks go onto the wilderness's
//! ground at the cutout's cells; the site's own level save remembers either
//! way that it has been stocked.

use shared::cellar::stock;
use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;
use shared::poi::Poi;
use shared::site::{Site, SiteLevel};
use shared::spot::Spot;

use crate::ServerWorld;
use crate::ids::new_id;
use crate::storage::SiteId;

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
    let ground = &mut world
        .site_mut(onto.0)
        .levels
        .entry(onto.1)
        .or_default()
        .ground;
    for cask in casks {
        let mut item = Item::new(new_id(), ItemKind::Cask);
        item.form = Some(ItemForm::Cask(stock(cask, &mut rng)));
        // On the ground, its clocks go by world time from now.
        item.lay_casks_down(now);
        let (x, y) = cask.at;
        let at = Spot::middle_of((x + offset.0, y + offset.1));
        ground.entry(at).or_default().push(item);
    }
}
