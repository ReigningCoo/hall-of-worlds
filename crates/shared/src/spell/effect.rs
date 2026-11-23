//! `Effect`: what a step of a spell does to what it reaches. Each is its own
//! skill (under Arcane, Divine or Insight), and Void reverses those after it.

use serde::{Deserialize, Serialize};

use crate::skill::SkillId;

/// One effect a spell can hold (the magic plan's table of effects).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Effect {
    Fire,
    Water,
    Air,
    Earth,
    /// Reverses every effect after it in its step (`parts`).
    Void,
    Identify,
    Curse,
    Illusion,
    Warding,
    Heal,
    Poison,
    Teleport,
    Light,
    Haste,
    Calm,
    Blessing,
    Levitate,
    Mana,
}

impl Effect {
    /// Every effect, in the panel's order.
    pub const ALL: [Self; 18] = [
        Self::Fire,
        Self::Water,
        Self::Air,
        Self::Earth,
        Self::Void,
        Self::Identify,
        Self::Curse,
        Self::Illusion,
        Self::Warding,
        Self::Heal,
        Self::Poison,
        Self::Teleport,
        Self::Light,
        Self::Haste,
        Self::Calm,
        Self::Blessing,
        Self::Levitate,
        Self::Mana,
    ];

    /// Its key, as in language files: the last part of its skill's path,
    /// except Heal's (Healing).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Fire => "fire",
            Self::Water => "water",
            Self::Air => "air",
            Self::Earth => "earth",
            Self::Void => "void",
            Self::Identify => "identify",
            Self::Curse => "curse",
            Self::Illusion => "illusion",
            Self::Warding => "warding",
            Self::Heal => "heal",
            Self::Poison => "poison",
            Self::Teleport => "teleport",
            Self::Light => "light",
            Self::Haste => "haste",
            Self::Calm => "calm",
            Self::Blessing => "blessing",
            Self::Levitate => "levitate",
            Self::Mana => "mana",
        }
    }

    /// The path of the skill it practises and is cheaper by.
    pub const fn skill_path(self) -> &'static str {
        match self {
            Self::Fire => "magic/arcane/fire",
            Self::Water => "magic/arcane/water",
            Self::Air => "magic/arcane/air",
            Self::Earth => "magic/arcane/earth",
            Self::Void => "magic/arcane/void",
            Self::Teleport => "magic/arcane/teleport",
            Self::Haste => "magic/arcane/haste",
            Self::Levitate => "magic/arcane/levitate",
            Self::Mana => "magic/arcane/mana",
            Self::Heal => "magic/divine/healing",
            Self::Warding => "magic/divine/warding",
            Self::Blessing => "magic/divine/blessing",
            Self::Curse => "magic/divine/curse",
            Self::Poison => "magic/divine/poison",
            Self::Calm => "magic/divine/calm",
            Self::Identify => "magic/insight/identify",
            Self::Illusion => "magic/insight/illusion",
            Self::Light => "magic/insight/light",
        }
    }

    /// The skill node.
    pub fn skill(self) -> SkillId {
        SkillId::find(self.skill_path()).expect("the skill tree has every effect")
    }
}
