//! Food's rows of the table of kinds (`def.rs`): what each is to carry. What
//! eating one does is `bonus::food`. Bread, Dwarven Bread and Raw Meat are
//! placeholders for trying the rules (owner, 2026-11-14), not sold yet.

use super::item_def::plain;
use super::{ItemDef, ItemKind, Shape};

impl ItemKind {
    pub(super) const fn food_def(self) -> ItemDef {
        let def = match self {
            Self::Bread => plain("Bread", 300, Shape::rectangle(2, 1)),
            // As heavy as it looks: like a lump of stone.
            Self::DwarvenBread => plain("Dwarven Bread", 900, Shape::rectangle(2, 1)),
            Self::RawMeat => plain("Raw Meat", 500, Shape::rectangle(2, 1)),
            _ => plain("Apple", 150, Shape::rectangle(1, 1)),
        };
        def.without_quality()
    }
}
