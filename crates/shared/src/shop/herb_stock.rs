//! Whether a shop could have a herb in stock, for quests that send players to
//! gather one: a herb any of the giver's town's shops could sell is never asked
//! for (owner, 2026-11-27).

use super::sells;
use crate::item::ItemKind;
use crate::site::BuildingKind;

/// Whether a shop of this kind could stock the herb with this key. No shop
/// chooses which herbs it sells yet, so one selling herb items at all could
/// stock any; none sells them today, so this rules nothing out. Narrow it by
/// `herb` once shops choose their herbs.
pub fn could_stock_herb(kind: BuildingKind, herb: &str) -> bool {
    let _ = herb;
    sells(kind).iter().any(|&(item, _)| item == ItemKind::Herb)
}
