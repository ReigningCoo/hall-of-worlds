//! `Requirement`: one thing an item asks of whoever uses it.

use super::armour::armour_requirements;
use crate::bonus::food;
use crate::creature::CreatureKind;
use crate::item::{Item, ItemCategory};
use crate::sheet::{Attribute, Attributes};
use crate::skill::SkillId;

/// One thing an item asks of whoever uses it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Requirement {
    /// At least this much of an attribute (with effects, not other worn items).
    Attribute { attribute: Attribute, min: u16 },
    /// At least this value of a skill, in hundredths. Nothing asks for one yet.
    Skill { id: SkillId, min: u32 },
    /// Only for these kinds of creature: food (`bonus::food`), which others
    /// are refused or poisoned by.
    Kinds(Vec<CreatureKind>),
}

impl Requirement {
    /// Whether these attributes meet it in full. Skills and kinds are not
    /// attributes, so count as met here: whoever asks for them checks them.
    pub fn met_by(&self, attributes: &Attributes) -> bool {
        match *self {
            Self::Attribute { attribute, min } => attributes.get(attribute) >= min,
            Self::Skill { .. } | Self::Kinds(_) => true,
        }
    }

    /// Whether a creature of this kind meets it: anyone meets all but `Kinds`.
    pub fn suits(&self, kind: CreatureKind) -> bool {
        match self {
            Self::Kinds(kinds) => kinds.contains(&kind),
            _ => true,
        }
    }
}

/// What this item asks of whoever wears or uses it: armour by how heavy it is
/// for what it covers, food the kinds it suits (`bonus::food`).
pub fn item_requirements(item: &Item) -> Vec<Requirement> {
    if item.kind.category() == ItemCategory::Armour {
        return armour_requirements(item);
    }
    match food(item.kind) {
        Some(food) if !food.kinds.is_empty() => vec![Requirement::Kinds(food.kinds.to_vec())],
        _ => Vec::new(),
    }
}
