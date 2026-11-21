//! Hearths (cooking plan PR 8): which sites have which, which are always
//! lit, towns' virtual one, the hearth someone faces, and the Ash cooking on
//! one leaves. A hearth is a crafting bench whose craft `is_hearth`.

use super::{Craft, KITCHEN_TOOLS, bench_faced};
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::furniture::{Furniture, Piece};
use crate::item::ItemKind;
use crate::poi::PoiKind;
use crate::spot::Spot;

/// Seconds of a hearth's heat used in cooking for each Ash it leaves: about
/// a piece of kindling's burning (`campfire::burn_seconds`), Claude's guess.
pub const ASH_HEAT_SECONDS: u64 = 450;

/// How likely a site of `kind` is to have hearths, in percent (Claude's
/// first guesses): every tower, castle and fort, most huts and cottages.
pub fn hearth_percent(kind: PoiKind) -> u64 {
    match kind {
        PoiKind::StoneTower | PoiKind::WoodenTower | PoiKind::Castle | PoiKind::Fort => 100,
        PoiKind::HerbalistHut
        | PoiKind::RedRoofCottage
        | PoiKind::GreyRoofCottage
        | PoiKind::StoneCottage
        | PoiKind::SnowyCottage => 90,
        PoiKind::ThatchedHut | PoiKind::StiltHut | PoiKind::GreenStiltHut | PoiKind::AdobeHouse => {
            70
        }
        _ => 0,
    }
}

/// The crafts of hearth a site of `kind` may have and how likely each, by
/// weight: small hearths in huts, larger in cottages and towers, a Range in
/// castles and forts. Empty where there are none.
pub fn hearth_weights(kind: PoiKind) -> &'static [(Craft, u64)] {
    match kind {
        PoiKind::Castle | PoiKind::Fort => &[(Craft::Range, 1)],
        PoiKind::StoneTower | PoiKind::WoodenTower => &[(Craft::Hearth, 2), (Craft::Range, 1)],
        PoiKind::HerbalistHut
        | PoiKind::RedRoofCottage
        | PoiKind::GreyRoofCottage
        | PoiKind::StoneCottage
        | PoiKind::SnowyCottage => &[(Craft::Stove, 1), (Craft::Hearth, 2), (Craft::Range, 1)],
        PoiKind::ThatchedHut | PoiKind::StiltHut | PoiKind::GreenStiltHut | PoiKind::AdobeHouse => {
            &[(Craft::Stove, 3), (Craft::Hearth, 1)]
        }
        _ => &[],
    }
}

/// Whether a site of `kind`'s hearths are always lit: buildings with a
/// household (towers, castles, forts; owner's). Elsewhere a hearth burns
/// kindling, lit and fed with Make Campfire.
pub fn always_lit(kind: PoiKind) -> bool {
    matches!(
        kind,
        PoiKind::StoneTower | PoiKind::WoodenTower | PoiKind::Castle | PoiKind::Fort
    )
}

/// Whether a site of `kind` gives a virtual hearth (owner's): towns, whose
/// buildings can't be entered. A `Craft::Hearth`, always lit, with every
/// kitchen tool (`town_tools`), none of them to take, and no storage.
pub fn town_hearth(kind: PoiKind) -> bool {
    matches!(
        kind,
        PoiKind::Hamlet | PoiKind::Village | PoiKind::Town | PoiKind::City
    )
}

/// The tools a town's virtual hearth has, at Common quality.
pub fn town_tools() -> &'static [ItemKind] {
    &KITCHEN_TOOLS
}

/// The hearth someone at `at` facing `heading` is at, as `bench_faced`.
pub fn hearth_faced(level: &Level, at: Spot, heading: Heading) -> Option<Piece> {
    bench_faced(level, at, heading).filter(
        |piece| matches!(piece.furniture, Furniture::CraftingBench { craft } if craft.is_hearth()),
    )
}

/// How many Ash a hearth leaves for `heat_seconds` of cooking on it (cooking
/// plan PR 9 calls it): one for each `ASH_HEAT_SECONDS` begun, none for none.
pub fn hearth_ash(heat_seconds: u64) -> u32 {
    heat_seconds
        .div_ceil(ASH_HEAT_SECONDS)
        .min(u64::from(u32::MAX)) as u32
}
