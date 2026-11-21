//! The tools a cooking process is built from, and the Cooking sub-skill each
//! one has (cooking plan, owner, 2026-11-20).

use serde::{Deserialize, Serialize};

use crate::item::ItemKind;

/// Cooking itself: the chance and the step limit lean on it.
pub const COOKING: &str = "crafting/cooking";
/// Cutting: the Knife's sub-skill.
pub const CUTTING: &str = "crafting/cooking/cutting";
/// Frying: the Pan's.
pub const FRYING: &str = "crafting/cooking/frying";
/// Boiling: the Cauldron's.
pub const BOILING: &str = "crafting/cooking/boiling";
/// Mixing: the Bowl's (churning is a long Bowl step).
pub const MIXING: &str = "crafting/cooking/mixing";
/// Baking: the Oven's.
pub const BAKING: &str = "crafting/cooking/baking";
/// Straining: the Sieve's.
pub const STRAINING: &str = "crafting/cooking/straining";
/// Pounding: the Mortar's, for spice pastes (Alchemy's Grinding is apart).
pub const POUNDING: &str = "crafting/cooking/pounding";

/// One tool. Rest is not a tool: it is time on an intermediate's clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tool {
    Knife,
    Pan,
    Cauldron,
    Bowl,
    Oven,
    Sieve,
    Mortar,
    // New tools go at the end: messages carry one by its place in this list.
}

impl Tool {
    pub const ALL: [Tool; 7] = [
        Self::Knife,
        Self::Pan,
        Self::Cauldron,
        Self::Bowl,
        Self::Oven,
        Self::Sieve,
        Self::Mortar,
    ];

    /// "Knife", "Pan", ...
    pub fn name(self) -> &'static str {
        match self {
            Self::Knife => "Knife",
            Self::Pan => "Pan",
            Self::Cauldron => "Cauldron",
            Self::Bowl => "Bowl",
            Self::Oven => "Oven",
            Self::Sieve => "Sieve",
            Self::Mortar => "Mortar & Pestle",
        }
    }

    /// Its sub-skill's path in the skill tree, under Cooking.
    pub fn sub_skill(self) -> &'static str {
        match self {
            Self::Knife => CUTTING,
            Self::Pan => FRYING,
            Self::Cauldron => BOILING,
            Self::Bowl => MIXING,
            Self::Oven => BAKING,
            Self::Sieve => STRAINING,
            Self::Mortar => POUNDING,
        }
    }

    /// Whether a step with it needs real heat: the Pan (dry heat), the
    /// Cauldron (wet heat) and the Oven. The rest work anywhere.
    pub fn needs_heat(self) -> bool {
        matches!(self, Self::Pan | Self::Cauldron | Self::Oven)
    }

    /// Whether what goes through it must hold a liquid (water, milk): the
    /// Cauldron cooks in one and the Sieve strains one off.
    pub fn needs_liquid(self) -> bool {
        matches!(self, Self::Cauldron | Self::Sieve)
    }

    /// The tool an item of this kind serves as: any knife or dagger is a
    /// Knife, the brewing Cauldron and the alchemy Mortar & pestle are
    /// themselves. Pans, bowls, ovens and sieves have no items yet: hearths
    /// will hold them (cooking plan PR 8).
    pub fn of_item(kind: ItemKind) -> Option<Tool> {
        match kind {
            ItemKind::Knife | ItemKind::Dagger => Some(Self::Knife),
            ItemKind::Cauldron => Some(Self::Cauldron),
            ItemKind::MortarAndPestle => Some(Self::Mortar),
            _ => None,
        }
    }

    /// Its place in `ALL`.
    pub fn index(self) -> usize {
        self as usize
    }
}
