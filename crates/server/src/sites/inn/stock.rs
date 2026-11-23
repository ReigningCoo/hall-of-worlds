//! The inn's furnishings kept full (The Way Inn plan, PR 5): an instance is
//! made fresh each time it opens (never saved), so each floor is stocked
//! as it is first made (`stock_inn`): every rack full of sealed drink (the
//! brewing room's of empty casks, to ferment in), the fireplaces always
//! burning, the kitchen's hearths and the Alchemy bench with every tool, the
//! treasury's chests of gold. Then the racks refill themselves
//! (`restock_inns`, every `RESTOCK_SECONDS`): an empty place gets a new
//! bottle or cask, and a drained one is swapped for a full one. Drinks are
//! free until there is trade.

use bevy::prelude::*;
use shared::bottle_rack::RackBottles;
use shared::cask::CaskForm;
use shared::cask_rack::RackCasks;
use shared::cellar::{CellarCask, stock};
use shared::chest::{Storage, Stored};
use shared::crafting_bench::BenchTools;
use shared::fireplace::Fireside;
use shared::furnishings::rack_bottle;
use shared::furniture::{Furniture, Piece};
use shared::inn::{self, RoomKind};
use shared::item::{Item, ItemKind, Quality, coin_items};
use shared::item_form::ItemForm;
use shared::material::Material;
use shared::potion::Liquid;
use shared::rng::{Rng, Seed};
use shared::site::SiteLevel;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crate::ServerWorld;
use crate::ids::new_id;
use crate::storage::{LevelSave, SiteId};

/// How often an instance's racks are refilled, in real seconds.
pub(crate) const RESTOCK_SECONDS: u64 = 60;
/// What each of the treasury's chests holds, in copper (gold coins).
const TREASURE_COPPER: u64 = 500 * 10_000;
/// How many tries a full drink gets before settling for what was rolled.
const TRIES: usize = 20;

/// When each open instance's racks were last refilled. Not saved.
#[derive(Default)]
pub(crate) struct Restocked(BTreeMap<SiteId, Instant>);

impl Restocked {
    /// Forgets an instance's: it is closed.
    pub(crate) fn forget(&mut self, id: SiteId) {
        self.0.remove(&id);
    }
}

fn unix_now() -> u64 {
    crate::quests::unix_now().max(0) as u64
}

/// A random sequence of the server's own, for rolling drinks.
fn rng() -> Rng {
    let since = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    let nanos = since.map_or(0, |since| since.as_nanos() as u64);
    Seed::new(nanos).derive("inn stock").rng()
}

/// The kind of room `piece` stands in on floor `index`.
fn room_of(index: u32, piece: &Piece) -> Option<RoomKind> {
    let (x, y) = piece.at;
    (inn::rooms(index).into_iter())
        .find(|room| room.contains(x, y))
        .map(|room| room.kind)
}

/// Stocks floor `index` of an inn instance (`level` as made), once.
pub(in crate::sites) fn stock_inn(
    world: &mut ServerWorld,
    id: SiteId,
    index: u32,
    level: &SiteLevel,
) {
    if !matches!(id, SiteId::InnInstance(_)) {
        return;
    }
    let saved = world.site(id).levels.get(&index);
    if saved.is_some_and(|level| level.furnishings_stocked) {
        return;
    }
    let mut rng = rng();
    let now = unix_now();
    let saved = world.site_mut(id).levels.entry(index).or_default();
    saved.furnishings_stocked = true;
    let level = &level.level;
    for piece in level.furniture().chain(level.furniture_on_top()) {
        let room = room_of(index, piece);
        match piece.furniture {
            Furniture::CaskRack { .. } | Furniture::BottleRack { .. } => {
                refill(saved, piece, room, (&mut rng, now));
            }
            Furniture::Fireplace { fireplace } => {
                let mut fireside = Fireside::new(fireplace);
                fireside.always_lit = true;
                saved.fireplaces.insert(piece.at, fireside);
            }
            Furniture::CraftingBench { craft } => {
                let mut tools = BenchTools::new(craft);
                tools.always_lit = craft.is_hearth();
                for kind in craft.tools() {
                    let mut item = Item::new(new_id(), kind);
                    item.quality = Some(Quality::Common);
                    let _ = tools.put(item);
                }
                saved.benches.insert(piece.at, tools);
            }
            Furniture::Chest { chest } if room == Some(RoomKind::Treasury) => {
                let mut stored = Stored::new(Storage::Chest(chest));
                let gold = coin_items(TREASURE_COPPER, 1000, None, &mut new_id);
                for coins in gold {
                    let _ = stored.put(coins);
                }
                saved.stores.insert(piece.at, stored);
            }
            _ => {}
        }
    }
}

/// Every `RESTOCK_SECONDS`, refills the racks of each open instance's floors
/// made so far, telling whoever has them.
pub(crate) fn restock_inns(world: &mut ServerWorld, now: Instant) {
    let every = Duration::from_secs(RESTOCK_SECONDS);
    for number in world.open_inns() {
        let id = SiteId::InnInstance(number);
        let last = world.inn_restocked.0.get(&id).copied();
        if last.is_some_and(|last| now.duration_since(last) < every) {
            continue;
        }
        world.inn_restocked.0.insert(id, now);
        let floors: Vec<u32> = world.site(id).levels.keys().copied().collect();
        let mut rng = rng();
        let unix = unix_now();
        for index in floors {
            let Some(level) = inn::level(index) else {
                continue;
            };
            let level = &level.level;
            let racks = (level.furniture().chain(level.furniture_on_top())).filter(|piece| {
                matches!(
                    piece.furniture,
                    Furniture::CaskRack { .. } | Furniture::BottleRack { .. }
                )
            });
            for piece in racks {
                let room = room_of(index, piece);
                let saved = world.site_mut(id).levels.entry(index).or_default();
                if refill(saved, piece, room, (&mut rng, unix)) {
                    world.bench_changes.push((id, index, piece.at));
                }
            }
        }
    }
}

/// Fills the rack `piece` (in a room of `room`) to every place, swapping
/// drained casks and bottles for full ones; the brewing room's casks come
/// empty, to ferment in, and stay as players leave them. Whether anything
/// changed.
fn refill(
    saved: &mut LevelSave,
    piece: &Piece,
    room: Option<RoomKind>,
    (rng, now): (&mut Rng, u64),
) -> bool {
    let brewing = room == Some(RoomKind::Brewing);
    match piece.furniture {
        Furniture::CaskRack { rack } => {
            let casks = (saved.racks.entry(piece.at)).or_insert_with(|| RackCasks::new(rack));
            if casks.rack != rack {
                *casks = RackCasks::new(rack);
            }
            let drained: Vec<_> = (casks.items())
                .filter(|item| !brewing && item.cask().is_none_or(CaskForm::is_empty))
                .map(|item| item.id)
                .collect();
            let mut changed = !drained.is_empty();
            for id in drained {
                casks.take(id);
            }
            while casks.items().count() < rack.capacity() {
                let wood = rng.pick_weighted(&[(Material::Oak, 3), (Material::Chestnut, 1)]);
                let form = match brewing {
                    true => CaskForm::new(rack.size, wood),
                    false => full_cask(rack.size, wood, rng),
                };
                let mut item = Item::new(new_id(), ItemKind::Cask);
                item.form = Some(ItemForm::Cask(form));
                item.lay_casks_down(now);
                if casks.put(item).is_err() {
                    break;
                }
                changed = true;
            }
            changed
        }
        Furniture::BottleRack { rack } => {
            let bottles =
                (saved.bottle_racks.entry(piece.at)).or_insert_with(|| RackBottles::new(rack));
            if bottles.rack != rack {
                *bottles = RackBottles::new(rack);
            }
            let drained: Vec<_> = (bottles.items())
                .filter(|item| match &item.form {
                    Some(ItemForm::Vessel(vessel)) => vessel.is_empty(),
                    _ => true,
                })
                .map(|item| item.id)
                .collect();
            let mut changed = !drained.is_empty();
            for id in drained {
                bottles.take(id);
            }
            while bottles.items().count() < rack.capacity() {
                let mut item = Item::new(new_id(), ItemKind::Vessel);
                item.form = Some(ItemForm::Vessel(full_bottle(rng)));
                item.lay_casks_down(now);
                if bottles.put(item).is_err() {
                    break;
                }
                changed = true;
            }
            changed
        }
        _ => false,
    }
}

/// A sealed cask full of drink, rolled as a cellar's.
fn full_cask(size: shared::cask::CaskSize, wood: Material, rng: &mut Rng) -> CaskForm {
    let cask = CellarCask {
        at: (0, 0),
        size,
        wood,
    };
    let mut form = stock(cask, rng);
    for _ in 0..TRIES {
        let drink = matches!(form.liquid, Some(Liquid::Drink(_)));
        if drink && !form.opened {
            break;
        }
        form = stock(cask, rng);
    }
    form
}

/// A sealed wine bottle of drink, rolled as a generated rack's.
fn full_bottle(rng: &mut Rng) -> shared::vessel::VesselForm {
    let mut bottle = rack_bottle(rng);
    for _ in 0..TRIES {
        if !bottle.is_empty() {
            break;
        }
        bottle = rack_bottle(rng);
    }
    bottle
}

#[cfg(test)]
mod tests;
