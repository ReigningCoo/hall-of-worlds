//! What the artifact line's end pays: an artifact, or the planet's own set
//! item (decisions.md: one per planet, from the planet's seed, so players may
//! later learn what each planet has), a piece of one of the pool's sets
//! (any kind a set may have: `shared::item_set`). Artifacts are only
//! weapons, armour pieces and jewellery: things with quality worth carrying
//! for good (Claude's choice).

use crate::item::ItemKind;
use crate::item_set::{ItemSet, POOL, SetPiece};
use crate::rng::Seed;

/// The kinds an artifact may be: weapons, armour pieces (not whole suits,
/// nor a horse's) and jewellery. Set items are any of a set's pieces.
pub const PRIZE_KINDS: [ItemKind; 33] = [
    ItemKind::Axe,
    ItemKind::Dagger,
    ItemKind::Knife,
    ItemKind::Longsword,
    ItemKind::Polearm,
    ItemKind::ShortSword,
    ItemKind::Spear,
    ItemKind::Sword,
    ItemKind::ArmArmour,
    ItemKind::ArmingCap,
    ItemKind::ArmingVest,
    ItemKind::BevoredHelm,
    ItemKind::Chausses,
    ItemKind::FootArmour,
    ItemKind::Coif,
    ItemKind::Collar,
    ItemKind::Gorget,
    ItemKind::Bevor,
    ItemKind::Aventail,
    ItemKind::Mantle,
    ItemKind::DemiGauntlets,
    ItemKind::Gambeson,
    ItemKind::Gauntlets,
    ItemKind::Helmet,
    ItemKind::LegArmour,
    ItemKind::PaddedChausses,
    ItemKind::PaddedGloves,
    ItemKind::ShoulderArmour,
    ItemKind::TorsoArmour,
    ItemKind::CuirassierTassets,
    ItemKind::WaistArmour,
    ItemKind::Necklace,
    ItemKind::Ring,
];

/// The planet's set item, from its seed (`WorldSeed::value`): a piece of one
/// of the pool's sets (`shared::item_set`), the same for everyone. Its kind
/// and quality are the set's (`SetPiece::kind`, `SetPiece::quality`).
pub fn set_item_of(planet: u64) -> SetPiece {
    let mut rng = Seed::new(planet).derive("set item").rng();
    let set = rng.range_u64(0, u64::from(POOL)) as u16;
    let pieces = ItemSet::of(set).pieces.len() as u64;
    let piece = rng.range_u64(0, pieces) as u8;
    SetPiece { set, piece }
}

/// An artifact's kind, from a seed of its own (the new item's).
pub fn artifact_kind(seed: Seed) -> ItemKind {
    let mut rng = seed.derive("artifact kind").rng();
    PRIZE_KINDS[rng.range_u64(0, PRIZE_KINDS.len() as u64) as usize]
}

#[cfg(test)]
mod tests;
