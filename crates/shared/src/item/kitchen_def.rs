//! Kitchen tools' rows of the table of kinds (`def.rs`, cooking plan PR 8):
//! the Pan, Bowl and Sieve cooking steps use (`cooking::Tool::of_item`), put
//! on hearths (`crafting_bench::KITCHEN_TOOLS`) or carried. Weights first
//! guesses.

use super::item_def::plain;
use super::{ItemDef, ItemKind, Shape};

impl ItemKind {
    pub(super) const fn kitchen_def(self) -> ItemDef {
        match self {
            // An iron frying pan and its handle.
            Self::Pan => plain("Pan", 1600, Shape::rectangle(1, 2)),
            // A wide earthenware mixing bowl.
            Self::Bowl => plain("Bowl", 900, Shape::rectangle(1, 1)),
            // A sieve of horsehair mesh in a wooden hoop.
            _ => plain("Sieve", 300, Shape::rectangle(1, 1)),
        }
    }
}
