//! Generated furnishings (`shared::furnishings`, tavern furniture plan PR 8):
//! stocked once, the first time their level is made, as cellar casks are.
//! Cask racks get casks rolled as cellar casks (`cellar::stock`), wine and
//! spirit racks rolled bottles (`furnishings::rack_bottle`), each half to
//! wholly full (`rack_fill`); a cellar rack under a cellar cask is left to
//! `cellar.rs`, which puts that cask on it. A lived-in place's fireplaces
//! always burn (`furnishings::is_lit`). Kept by the piece's cell as anything
//! with places (`benches/held.rs`), a first level's in its own site's save.

use shared::bottle_rack::{BottleRack, RackBottles};
use shared::cask_rack::{CaskRack, RackCasks};
use shared::cellar::{CellarCask, stock};
use shared::fireplace::{Fireplace, Fireside};
use shared::furnishings::{is_lit, rack_bottle, rack_fill};
use shared::furniture::{Furniture, Piece};
use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;
use shared::material::Material;
use shared::poi::Poi;
use shared::rng::Rng;
use shared::site::{Site, SiteLevel};

use crate::ServerWorld;
use crate::ids::new_id;
use crate::storage::{LevelSave, SiteId};

/// Stocks level `index` of a poi's or developer's site or a Dev Area (the
/// Tavern's; `level` as generated, before pieces made by hand go on),
/// unless done before.
pub(in crate::sites) fn stock_furnishings(
    world: &mut ServerWorld,
    (id, poi): (SiteId, &Poi),
    site: &Site,
    index: u32,
    level: &SiteLevel,
) {
    if !matches!(id, SiteId::Poi(_) | SiteId::Dev { .. } | SiteId::DevArea(_)) {
        return;
    }
    let saved = world.site(id).levels.get(&index);
    if saved.is_some_and(|level| level.furnishings_stocked) {
        return;
    }
    let stocked = |piece: &&Piece| {
        matches!(
            piece.furniture,
            Furniture::CaskRack { .. } | Furniture::BottleRack { .. } | Furniture::Fireplace { .. }
        )
    };
    let on_top = level.level.furniture_on_top();
    let pieces: Vec<Piece> = level
        .level
        .furniture()
        .chain(on_top)
        .filter(stocked)
        .copied()
        .collect();
    if pieces.is_empty() {
        return;
    }
    let casks = site.cellar_casks(index, level);
    let lit = is_lit(site.kind(), site.ruin());
    let mut rng = poi.seed.derive("furnishings stock").rng();
    let now = crate::quests::unix_now().max(0) as u64;
    let saved = world.site_mut(id).levels.entry(index).or_default();
    saved.furnishings_stocked = true;
    for piece in pieces {
        match piece.furniture {
            Furniture::CaskRack { rack } if !under_cask(&piece, &casks) => {
                stock_casks(saved, &piece, rack, (&mut rng, now));
            }
            Furniture::BottleRack { rack } => stock_bottles(saved, &piece, rack, (&mut rng, now)),
            Furniture::Fireplace { fireplace } => light(saved, &piece, fireplace, lit),
            _ => {}
        }
    }
}

/// Whether a cellar cask lies on `piece`.
fn under_cask(piece: &Piece, casks: &[CellarCask]) -> bool {
    casks.iter().any(|cask| piece.covers(cask.at))
}

/// The woods a rack's casks are rolled in, by weight (as cellars').
const WOODS: [(Material, u64); 3] = [
    (Material::Oak, 6),
    (Material::Chestnut, 2),
    (Material::Pine, 2),
];

fn stock_casks(saved: &mut LevelSave, piece: &Piece, rack: CaskRack, (rng, now): (&mut Rng, u64)) {
    if saved.racks.contains_key(&piece.at) {
        return;
    }
    let mut casks = RackCasks::new(rack);
    for _ in 0..rack_fill(rack.capacity(), rng) {
        let cask = CellarCask {
            at: piece.at,
            size: rack.size,
            wood: rng.pick_weighted(&WOODS),
        };
        let mut item = Item::new(new_id(), ItemKind::Cask);
        item.form = Some(ItemForm::Cask(stock(cask, rng)));
        item.lay_casks_down(now);
        let _ = casks.put(item);
    }
    saved.racks.insert(piece.at, casks);
}

fn stock_bottles(
    saved: &mut LevelSave,
    piece: &Piece,
    rack: BottleRack,
    (rng, now): (&mut Rng, u64),
) {
    if saved.bottle_racks.contains_key(&piece.at) {
        return;
    }
    let mut bottles = RackBottles::new(rack);
    for _ in 0..rack_fill(rack.capacity(), rng) {
        let mut item = Item::new(new_id(), ItemKind::Vessel);
        item.form = Some(ItemForm::Vessel(rack_bottle(rng)));
        item.lay_casks_down(now);
        let _ = bottles.put(item);
    }
    saved.bottle_racks.insert(piece.at, bottles);
}

/// A household's fireplace, always burning; others are left unlit.
fn light(saved: &mut LevelSave, piece: &Piece, fireplace: Fireplace, lit: bool) {
    if !lit || saved.fireplaces.contains_key(&piece.at) {
        return;
    }
    let mut fireside = Fireside::new(fireplace);
    fireside.always_lit = true;
    saved.fireplaces.insert(piece.at, fireside);
}

#[cfg(test)]
mod tests;
