//! Shopkeepers: who keeps which building, and their side of a trade.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::Item;
use shared::poi::ChunkPos;
use shared::shop::sells;
use shared::site::BuildingKind;

use super::Mirror;
use super::shop_here::shop_of;
use crate::ServerWorld;
use crate::connections::Client;
use crate::ids::new_id;
use crate::storage::SiteId;

/// Who keeps each of a town level's shops, by building. Each has one for good, made
/// the first time it is needed.
pub(super) fn keepers_of(world: &mut ServerWorld, visit: &NewKeepers) -> Vec<(u16, Id)> {
    let saved = world.site_mut(visit.id).levels.entry(visit.index);
    let saved = saved.or_default();
    let shops = visit.shops.iter().map(|&building| {
        let save = saved.buildings.entry(building).or_default();
        let keeper = *save.keeper.get_or_insert_with(new_id);
        let old_stock = std::mem::take(&mut save.stock);
        let record = saved.records.entry(keeper).or_default();
        record.carried.extend(old_stock);
        (building, keeper)
    });
    shops.collect()
}

/// A town level, for `keepers_of`.
pub(super) struct NewKeepers {
    pub(super) id: SiteId,
    pub(super) index: u32,
    pub(super) shops: Vec<u16>,
}

/// Who keeps the shops of the towns set into a wilderness window (`mirrors`),
/// by building, made as `keepers_of` does: they are the towns' own.
pub(super) fn keepers_in_window(world: &mut ServerWorld, mirrors: &[Mirror]) -> Vec<(u16, Id)> {
    let mut keepers = Vec::new();
    for mirror in mirrors {
        let SiteId::Poi(chunk) = mirror.id else {
            continue;
        };
        let Some(wild) = super::wild_chunk(world, chunk) else {
            continue;
        };
        let shops = wild.town.iter().filter(|b| !sells(b.look.kind).is_empty());
        let shops: Vec<u16> = shops.map(|b| b.look.id).collect();
        if shops.is_empty() {
            continue;
        }
        let town = NewKeepers {
            id: mirror.id,
            index: mirror.index,
            shops,
        };
        keepers.extend(keepers_of(world, &town));
    }
    keepers
}

/// Which building on the player's level this is the keeper of, if any.
pub(crate) fn keeper_building(client: &Client, world: &mut ServerWorld, with: Id) -> Option<u16> {
    shop_of(client, world, with).map(|shop| shop.building.look.id)
}

/// A shop's side of a trade with the player, as their level has it.
pub(crate) struct ShopTerms {
    /// The town it is in.
    pub(crate) town: ChunkPos,
    /// What kind of shop it is (what it normally sells).
    pub(crate) kind: BuildingKind,
    /// What it has.
    pub(crate) stock: Vec<Item>,
    /// Whether the player is within `trade::REACH` of its door.
    pub(crate) in_reach: bool,
    /// Where its save is: site, level and building.
    at: (SiteId, u32, u16),
}

/// The shop `with` keeps on the player's level, if any, with what it has.
pub(crate) fn shop_terms(client: &Client, world: &mut ServerWorld, with: Id) -> Option<ShopTerms> {
    let shop = shop_of(client, world, with)?;
    let at = client.visit.as_ref()?.spot();
    let level = world.site(shop.id).levels.get(&shop.index);
    let stock = level.and_then(|level| level.records.get(&with));
    Some(ShopTerms {
        town: shop.poi.chunk,
        kind: shop.building.look.kind,
        stock: stock
            .map(|record| record.carried.clone())
            .unwrap_or_default(),
        in_reach: shop.in_reach(at),
        at: (shop.id, shop.index, shop.building.look.id),
    })
}

/// Whether the player is within `trade::REACH` of the door of the shop `with`
/// keeps on their level: `shop_terms` without its stock.
pub(crate) fn near_shop(client: &Client, world: &mut ServerWorld, with: Id) -> bool {
    let at = client.visit.as_ref().map(|visit| visit.spot());
    let shop = shop_of(client, world, with);
    at.zip(shop).is_some_and(|(at, shop)| shop.in_reach(at))
}

/// A trade with a shop completed: what it has now, `sold` to it at game
/// minute `minute` (gone sooner than its own stock, `shop::sold_gone`).
pub(crate) fn traded_with(
    world: &mut ServerWorld,
    terms: &ShopTerms,
    with: Id,
    stock: Vec<Item>,
    (sold, minute): (&[Id], u64),
) {
    let (id, index, building) = terms.at;
    let level = world.site_mut(id).levels.entry(index).or_default();
    let level_stock_len = level.records.get(&with).map_or(0, |had| had.carried.len());
    let saved = level.buildings.entry(building).or_default();
    // A gift alone (a tip with nothing traded) isn't a trade.
    if !sold.is_empty() || stock.len() != level_stock_len {
        saved.trades = saved.trades.saturating_add(1);
    }
    saved.sold.extend(sold.iter().map(|&item| (item, minute)));
    saved
        .sold
        .retain(|item, _| stock.iter().any(|one| one.id == *item));
    level.records.entry(with).or_default().carried = stock;
}
