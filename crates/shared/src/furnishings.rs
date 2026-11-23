//! Furnishings (tavern furniture plan PR 8): what castles and towers are
//! furnished with besides tables, seats and crafting benches: bedrooms (a
//! bed, a chest, a wardrobe), racks under their cellar casks and a wine rack
//! beside them, a fireplace in the great hall, and wall decorations, with
//! sconces, a chandelier and candles where people live. All from hashes of
//! `seed.derive("furnishings")`, placed after furniture, benches and cellar
//! casks, so no generator version, golden test or earlier piece moves.
//!
//! Also what layouts placed by code share (the Tavern Dev Area, The Way
//! Inn): `against_wall`, and what generated racks start with
//! (`rack_fill`, `rack_bottle`).

// Split into files (`cargo xtask lint` keeps files small).
mod bedroom;
mod cellar_racks;
mod decorate;
mod furnisher;
mod hall;
mod setting;
mod stock;
mod wall;

pub use setting::Setting;
pub use stock::{rack_bottle, rack_fill};
pub use wall::against_wall;

use crate::cellar::CellarCask;
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::SiteLevel;
use furnisher::Furnisher;

/// Which level of which site is being furnished.
#[derive(Clone, Copy, Debug)]
pub struct Floor {
    pub kind: PoiKind,
    /// The site's map seed.
    pub seed: Seed,
    pub index: u32,
    /// Whether it is under the ground (a basement).
    pub underground: bool,
    /// 0 to 5: each piece is left at 100 − 15 × ruin %.
    pub ruin: u8,
}

/// Furnishes `site`, a level of a castle or tower as generated and furnished,
/// whose cellar casks (if it is the cellar level) are `casks`: those go onto
/// racks of their size where one fits. Nothing for other kinds
/// (`Setting::for_kind`).
pub fn furnish_rooms(site: &mut SiteLevel, floor: Floor, casks: &[CellarCask]) {
    let Some(setting) = Setting::for_kind(floor.kind) else {
        return;
    };
    let lit = is_lit(floor.kind, floor.ruin);
    let mut furnisher = Furnisher::new(site, floor, casks);
    let cellar = cellar_racks::place_racks(&mut furnisher, &setting, casks);
    let hall = match floor.index == 0 {
        true => hall::place_fireplace(&mut furnisher, &setting),
        false => None,
    };
    let taken: Vec<_> = hall.into_iter().chain(cellar).collect();
    let bedrooms = bedroom::place_bedrooms(&mut furnisher, &setting, &taken);
    decorate::decorate(
        &mut furnisher,
        &setting,
        decorate::Roles {
            hall,
            cellar,
            bedrooms: &bedrooms,
            lit,
        },
    );
}

/// Whether a site of `kind` ruined by `ruin` is lived in: its sconces,
/// chandelier and candles are lit and its fireplaces always burn (the
/// server lights them once, `Fireside::always_lit`).
pub fn is_lit(kind: PoiKind, ruin: u8) -> bool {
    Setting::for_kind(kind).is_some_and(|setting| setting.lit) && ruin == 0
}

#[cfg(test)]
mod tests;
