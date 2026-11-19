//! Abilities that need a tool carried: without it they can't be used, and the
//! client greys them out like anything else that can't be used now.

use super::Ability;
use crate::item::{Inventory, ItemKind};

impl Ability {
    /// The tool it needs carried, if any: anywhere in what is carried, bags too.
    pub const fn tool(self) -> Option<ItemKind> {
        match self {
            Self::PressOil | Self::PressJuice => Some(ItemKind::OilPress),
            Self::Grind => Some(ItemKind::MortarAndPestle),
            Self::Brew => Some(ItemKind::Cauldron),
            _ => None,
        }
    }

    /// Whether `carried` has its tool, or it needs none. Collect Water needs an
    /// empty bottle to fill, or a cask with room for water (`shared::cask`);
    /// Fell Tree and Cut Kindling a one-handed axe held, not just carried
    /// (`shared::woodcutting`).
    pub fn has_tool(self, carried: &Inventory) -> bool {
        match self {
            Self::CollectWater => {
                return carried.empty_bottle().is_some() || carried.cask_for_water();
            }
            Self::FellTree | Self::CutKindling => return carried.held_axe().is_some(),
            // A usable yeast carried; the cask may lie in reach.
            Self::Ferment => return !carried.ferment_choices().yeasts.is_empty(),
            _ => {}
        }
        self.tool().is_none_or(|kind| carried.count_of(kind) > 0)
    }
}
