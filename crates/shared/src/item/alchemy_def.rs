//! Alchemy's rows of the table of kinds (`def.rs`): its tools and apparatus
//! (`ability::Ability::tool`, `alchemy::Apparatus::item`), and the Retort's
//! solids, counted without quality (`alchemy::Solid`). Weights first guesses.

use super::item_def::plain;
use super::{ItemDef, ItemKind, Shape};
use crate::alchemy::{MERCURY_GRAMS, SALT_GRAMS, SULPHUR_GRAMS};

impl ItemKind {
    pub(super) const fn alchemy_def(self) -> ItemDef {
        let square = Shape::rectangle(1, 1);
        match self {
            Self::OilPress => plain("Oil Press", 6000, Shape::rectangle(2, 2)),
            Self::MortarAndPestle => plain("Mortar and Pestle", 1500, square),
            Self::Retort => plain("Retort", 900, Shape::rectangle(1, 2)),
            Self::Alembic => plain("Alembic", 1800, Shape::rectangle(2, 2)),
            Self::Crucible => plain("Crucible", 1200, square),
            Self::Filter => plain("Filter", 300, square),
            Self::Pelican => plain("Pelican", 1200, Shape::rectangle(2, 2)),
            Self::Salt => solid("Salt", SALT_GRAMS),
            Self::Sulphur => solid("Sulphur", SULPHUR_GRAMS),
            Self::Mercury => solid("Mercury", MERCURY_GRAMS),
            // Kept for cooking later; no longer used.
            _ => plain("Cauldron", 4000, Shape::rectangle(2, 2)),
        }
    }
}

/// A Retort solid's row: one cell, counted, without quality.
const fn solid(name: &'static str, grams: u32) -> ItemDef {
    plain(name, grams, Shape::rectangle(1, 1))
        .without_quality()
        .counted()
}
