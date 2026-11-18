//! Abilities that need a tool carried: without it they can't be used, and the
//! client greys them out like anything else that can't be used now.

use super::Ability;
use crate::item::{Inventory, ItemKind};

impl Ability {
    /// The tool it needs carried, if any: anywhere in what is carried, bags too.
    pub const fn tool(self) -> Option<ItemKind> {
        match self {
            Self::PressOil => Some(ItemKind::OilPress),
            Self::Grind => Some(ItemKind::MortarAndPestle),
            Self::Brew => Some(ItemKind::Cauldron),
            _ => None,
        }
    }

    /// Whether `carried` has its tool, or it needs none. Collect Water needs an
    /// empty bottle to fill.
    pub fn has_tool(self, carried: &Inventory) -> bool {
        if self == Self::CollectWater {
            return carried.empty_bottle().is_some();
        }
        self.tool().is_none_or(|kind| carried.count_of(kind) > 0)
    }
}
