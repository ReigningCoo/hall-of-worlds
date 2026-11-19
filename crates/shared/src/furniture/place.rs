//! Placing furniture on a site's level: tables near the middle of rooms with
//! seats along them, and loose seats against walls, never cutting anything off.
//!
//! Everything comes from hashes of the site's seed (`derive("furniture")`),
//! never from a generator's sequence, and is added after the generator and any
//! ruin, so walls, doors and sites' golden tests are as they were.

// Split into files (`cargo xtask lint` keeps files small).
mod loose;
mod palette;
mod placer;
mod reach;
pub(crate) mod rooms;
mod seats;
mod setting;
mod tables;

pub use setting::Setting;

use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::SiteLevel;
use placer::Placer;

/// In 100ths: how much less likely each step of ruin makes a piece to be left.
const RUIN_LOSS_PERCENT: u64 = 15;

/// Furnishes level `index` of a site of `kind` from its map `seed`, ruined by
/// `ruin` (0 to 5): each piece is left at 100 − 15 × ruin %. Nothing for kinds
/// with no furniture (`Setting::for_kind`).
pub fn furnish(site: &mut SiteLevel, kind: PoiKind, seed: Seed, index: u32, ruin: u8) {
    let Some(setting) = Setting::for_kind(kind) else {
        return;
    };
    let seed = seed.derive("furniture");
    let kept = 100u64.saturating_sub(RUIN_LOSS_PERCENT * u64::from(ruin));
    let mut placer = Placer::new(site, setting, seed, index, kept);
    placer.place_tables();
    placer.place_loose();
    for piece in placer.into_pieces() {
        site.level.place_furniture(piece);
    }
}
