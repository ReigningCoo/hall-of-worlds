//! Woodcutting's rows of the table of kinds (`def.rs`): logs and kindling
//! (`shared::woodcutting`), whose forms say their wood, name and weight, and
//! the Ash a campfire leaves (`shared::campfire`).

use super::item_def::plain;
use super::{ItemDef, ItemKind, Shape};

impl ItemKind {
    pub(super) const fn wood_def(self) -> ItemDef {
        let def = match self {
            // 30 × 30 × 100 cm: 8 cells long, 3 across (owner). Its weight is
            // its wood's (`woodcutting::LogForm::grams`).
            Self::Log => plain("Log", 45_000, Shape::rectangle(8, 3)),
            // Counted, 1 kg a piece, a stack keying on its wood.
            Self::Kindling => plain("Kindling", 1000, Shape::rectangle(2, 1)).counted(),
            // What a campfire leaves (`shared::campfire`): a handful a piece
            // of kindling burnt, counted, no form.
            _ => plain("Ash", crate::campfire::ASH_GRAMS, Shape::rectangle(1, 1)).counted(),
        };
        def.without_quality()
    }
}
