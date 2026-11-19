//! The Retort's solid (Alchemy plan PR 8): Salt, Sulphur or Mercury leaning a
//! mixture towards one row of attributes, or a vegetable transforming it
//! (`transform`). One solid a Retort, used up each run.

use super::Mix;
use super::transform::transformation;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::sheet::Row;
use crate::vegetable::VegetableTag;

/// What one Sulphur weighs, in grams: a small yellow lump.
pub const SULPHUR_GRAMS: u32 = 10;
/// What one Mercury weighs, in grams: a phial of quicksilver.
pub const MERCURY_GRAMS: u32 = 30;
/// How much the solid's own row is raised, and the other rows lowered, in
/// percent of each dose (owner, 2026-11-19).
pub const LEAN_PERCENT: i32 = 25;

/// A solid in a Retort.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Solid {
    /// The Body row +25%, Mind and Spirit −25%.
    Salt,
    /// The Mind row +25%, Body and Spirit −25%.
    Sulphur,
    /// The Spirit row +25%, Body and Mind −25%.
    Mercury,
    /// A vegetable with a transformation (`TRANSFORMATIONS`).
    Vegetable(VegetableTag),
}

impl Solid {
    /// The solid an item is, if a Retort takes it: Salt, Sulphur, Mercury or a
    /// vegetable some transformation uses.
    pub fn of_item(item: &Item) -> Option<Self> {
        match item.kind {
            ItemKind::Salt => Some(Self::Salt),
            ItemKind::Sulphur => Some(Self::Sulphur),
            ItemKind::Mercury => Some(Self::Mercury),
            ItemKind::Vegetable => {
                let tag = VegetableTag::of(&item.vegetable()?.vegetable);
                transformation(tag).map(|_| Self::Vegetable(tag))
            }
            _ => None,
        }
    }

    /// The row Salt, Sulphur or Mercury leans towards.
    pub fn leans_to(self) -> Option<Row> {
        match self {
            Self::Salt => Some(Row::Body),
            Self::Sulphur => Some(Row::Mind),
            Self::Mercury => Some(Row::Spirit),
            Self::Vegetable(_) => None,
        }
    }
}

/// Applies a Retort's solid to `mix`: a lean, or a transformation (a vegetable
/// no transformation uses changes nothing; the Retort refuses one first).
pub(super) fn apply_solid(mix: &mut Mix, solid: Solid) {
    if let Some(row) = solid.leans_to() {
        mix.lean(row, LEAN_PERCENT);
    }
    if let Solid::Vegetable(tag) = solid
        && let Some(change) = transformation(tag)
    {
        mix.transform(change);
    }
}

/// `count` of a solid sold or made: Salt, Sulphur or Mercury.
pub fn new_solid(kind: ItemKind, count: u32, id: Id) -> Item {
    let mut item = Item::new(id, kind);
    item.count = count.max(1);
    item
}
