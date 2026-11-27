//! Which kinds of item may be a set's pieces (owner, 2026-11-27): anything
//! worn or held to fight with that has quality and can be kept, but whole
//! suits of armour, a horse's armour and the lantern (Claude's choice: a
//! suit is already all of one body's places, a horse's can't go with a
//! person's pieces, and a lantern is a tool).

use std::sync::OnceLock;

use crate::creature::Location;
use crate::item::{ItemKind, WearDef};

/// Worn or held, with quality, but never a set's piece.
const NOT_PIECES: [ItemKind; 7] = [
    ItemKind::ArmourCoat,
    ItemKind::Corslet,
    ItemKind::HalfArmour,
    ItemKind::FullArmour,
    ItemKind::Barding,
    ItemKind::Caparison,
    ItemKind::Lantern,
];

/// The kinds a set's worn pieces may be, in `ItemKind::ALL`'s order.
pub(super) fn worn_kinds() -> &'static [ItemKind] {
    static WORN: OnceLock<Vec<ItemKind>> = OnceLock::new();
    WORN.get_or_init(|| pieces_where(|wear| !held(wear)))
}

/// The weapons any set may have, in `ItemKind::ALL`'s order.
pub(super) fn weapons() -> &'static [ItemKind] {
    static WEAPONS: OnceLock<Vec<ItemKind>> = OnceLock::new();
    WEAPONS.get_or_init(|| pieces_where(held))
}

/// How a kind is worn: only asked of kinds that are.
pub(super) fn wear(kind: ItemKind) -> WearDef {
    kind.def().wear.expect("a set's pieces are worn or held")
}

/// The kinds that may be pieces, worn as `keep` says.
fn pieces_where(keep: impl Fn(WearDef) -> bool) -> Vec<ItemKind> {
    let pieces = ItemKind::ALL.into_iter().filter(|kind| {
        let def = kind.def();
        let kept = def.has_quality && !def.counted && def.container.is_none();
        kept && !NOT_PIECES.contains(kind) && def.wear.is_some_and(&keep)
    });
    pieces.collect()
}

/// Whether it is held in a hand rather than worn.
fn held(wear: WearDef) -> bool {
    wear.locations.contains(&Location::Held)
}
