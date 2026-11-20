//! Food's rows of the table of kinds (`def.rs`): what each is to carry. What
//! eating one does is `bonus::food` (fruit's: `fruit::eating`, vegetables': `vegetable::eating`); flour and yeast,
//! materials, have their rows here too. Bread, Dwarven Bread and Raw Meat are
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
            // Counted, a stack keying on its fruit (`shared::fruit`): its form
            // says its name, cells and weight.
            // Counted, a stack keying on its vegetable (`shared::vegetable`),
            // as fruit.
            Self::Vegetable => plain("Vegetable", 100, Shape::rectangle(1, 1)).counted(),
            // Counted, a stack keying on its fungus (`shared::fungus`), as
            // vegetables.
            Self::Fungus => plain("Fungus", 100, Shape::rectangle(1, 1)).counted(),
            // Counted, a stack keying on its grain (`shared::flour`): its form
            // says its name and weight. Not eaten as it is.
            Self::Flour => plain("Flour", 29, Shape::rectangle(1, 1)).counted(),
            // Each its own item, not counted, with quality (`shared::yeast`):
            // a small lidded crock of starter.
            Self::Yeast => return plain("Yeast", 250, Shape::rectangle(1, 1)),
            _ => plain("Fruit", 150, Shape::rectangle(1, 1)).counted(),
        };
        def.without_quality()
    }
}
