//! Abilities: named things a creature can do, which will hook onto skills. See
//! `docs/context/shared/ability.md`.
//!
//! Active ones are done when asked (and can go on the action bar), toggles
//! turned on and off there; passive ones work by themselves. Every creature
//! knows Attack, Heal, Magic and Sneak; others come with its kind, from a skill
//! (`skill.rs`) or are learned (`Learned`), and some need a tool (`tool.rs`).

use serde::{Deserialize, Serialize};

use crate::sheet::Attribute;

mod bar;
mod bar_slot;
mod knowing;
mod learned;
mod skill;
mod tool;

pub use bar::{ActionBar, BAR_SLOTS, BarChange, BarRefusal};
pub use bar_slot::{BarSlot, BarThing, usable_from_bar};
pub use knowing::{known, known_by_kind};
pub use learned::Learned;
pub use skill::ALCHEMY;

/// Something a creature can do.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Ability {
    Flutter,
    Kick,
    NightVision,
    Trample,
    // New abilities go at the end: messages carry one by its place in this list.
    /// Striking with what is held, or without: every creature knows it.
    Attack,
    /// Mana turned into Health, on yourself or another (`magic`): every
    /// creature knows it for now, as there is no way to learn spells yet.
    Heal,
    /// Moving slowly and quietly, under the pace Sneaking is learned at
    /// (`energy::SNEAK_TICKS_PER_CELL`): a toggle every creature knows.
    Sneak,
    /// Vegetables, nuts or seeds pressed into a bottle of oil: from Alchemy,
    /// with an oil press (`tool`).
    PressOil,
    /// A process of apparatus steps worked through the Alchemy panel
    /// (`shared::alchemy`): from Alchemy, with an apparatus carried. Saved
    /// bars' Grind and Brew (retired with the Alchemy plan) load as it.
    #[serde(alias = "Grind", alias = "Brew")]
    Alchemy,
    /// Empty bottles filled at water faced, one after another (`potion`'s
    /// `collect`): learned on a first empty bottle.
    CollectWater,
    /// A tree faced felled for logs, with a one-handed axe held
    /// (`shared::woodcutting`): learned on first wielding one.
    FellTree,
    /// A log split into kindling, with a one-handed axe held: learned with
    /// Fell Tree.
    CutKindling,
    /// Fruit pressed into juice, into a carried cask or vessel
    /// (`shared::juice`): from Alchemy, with an oil press.
    PressJuice,
    /// A cask of juice, cordial, or water with fruit or grain set fermenting
    /// with yeast (`shared::ferment`): from Fermenting.
    Ferment,
    /// A campfire lit on the ground faced, or the one faced fed, with a piece
    /// of kindling (`shared::campfire`): learned on first carrying kindling.
    MakeCampfire,
    /// A process of tool steps in rows worked through the Cooking panel
    /// (`shared::cooking`): from Cooking, at a hearth, a town, a campfire or
    /// with what is carried.
    Cook,
    /// Opens the spell design panel, where spells are made for the spellbook
    /// (`shared::spell`, magic plan): every creature knows it.
    Magic,
}

/// Whether an ability is done when asked, or works by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Use {
    /// Done when asked: it can go on the action bar.
    Active,
    /// Always at work: it can't go on the action bar.
    Passive,
    /// Turned on and off when asked, on the action bar: on until turned off.
    Toggle,
}

impl Ability {
    /// Every ability, in the order they are listed to players.
    pub const ALL: [Ability; 17] = [
        Self::Attack,
        Self::Heal,
        Self::Magic,
        Self::Sneak,
        Self::Flutter,
        Self::Kick,
        Self::NightVision,
        Self::Trample,
        Self::PressOil,
        Self::PressJuice,
        Self::Alchemy,
        Self::Cook,
        Self::Ferment,
        Self::CollectWater,
        Self::FellTree,
        Self::CutKindling,
        Self::MakeCampfire,
    ];

    /// What it is called.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Attack => "Attack",
            Self::Heal => "Heal",
            Self::Flutter => "Flutter",
            Self::Kick => "Kick",
            Self::NightVision => "Night Vision",
            Self::Trample => "Trample",
            Self::Sneak => "Sneak",
            Self::PressOil => "Press Oil",
            Self::PressJuice => "Press Juice",
            Self::Alchemy => "Alchemy",
            Self::CollectWater => "Collect Water",
            Self::FellTree => "Fell Tree",
            Self::CutKindling => "Cut Kindling",
            Self::Ferment => "Ferment",
            Self::MakeCampfire => "Make Campfire",
            Self::Cook => "Cook",
            Self::Magic => "Magic",
        }
    }

    /// Whether it harms someone (or a door): Attack, Kick and Trample. None
    /// of these can be used in The Way Inn (`shared::inn`), where nothing
    /// may harm anyone.
    pub const fn harms(self) -> bool {
        matches!(self, Self::Attack | Self::Kick | Self::Trample)
    }

    /// Whether it is done when asked or works by itself.
    pub const fn use_kind(self) -> Use {
        match self {
            Self::NightVision => Use::Passive,
            Self::Sneak => Use::Toggle,
            Self::Attack
            | Self::Heal
            | Self::Flutter
            | Self::Kick
            | Self::Trample
            | Self::PressOil
            | Self::PressJuice
            | Self::Alchemy
            | Self::CollectWater
            | Self::FellTree
            | Self::CutKindling
            | Self::Ferment
            | Self::MakeCampfire
            | Self::Cook
            | Self::Magic => Use::Active,
        }
    }

    /// The attributes it leans on (first guesses, owner OK 2026-11-07), which colour
    /// it: Attack as the combat skills (it may follow the weapon held's skill later),
    /// the kinds' own by what their bodies do.
    pub const fn leans(self) -> &'static [Attribute] {
        use Attribute::*;
        match self {
            Self::Attack | Self::Kick => &[Strength, Agility],
            // Trample by its body; Fell Tree and Cut Kindling as their skill,
            // Woodcutting.
            Self::Trample | Self::FellTree | Self::CutKindling => &[Strength, Toughness],
            Self::Flutter => &[Agility],
            // Fire-making, as survival.
            Self::MakeCampfire => &[Toughness, Perception],
            // As its skill, Water.
            // Collect Water and Ferment as their skills, Water and Fermenting.
            // Cook as its skill, Cooking.
            Self::NightVision | Self::CollectWater | Self::Ferment | Self::Cook => &[Perception],
            Self::Sneak => &[Agility],
            // As its skill, Healing, under Divine.
            Self::Heal => &[Empathy],
            // As their skill, Alchemy.
            Self::PressOil | Self::PressJuice | Self::Alchemy => &[Intellect],
            // As spells' Form, which every spell uses.
            Self::Magic => &[Intellect, Will],
        }
    }

    /// What it does, in a line for its tip.
    pub const fn description(self) -> &'static str {
        match self {
            Self::Attack => "Strikes your target with what you hold, or without.",
            Self::Heal => "Heals you or your allies, and tries to keep you alive if you ask.",
            Self::Flutter => "Takes to the air for a moment.",
            Self::Kick => "Kicks your target a step away, or the closed door beside you.",
            Self::NightVision => "Sees in the dark.",
            Self::Trample => "Charges down whoever is in the way.",
            Self::Sneak => "Moves slowly and quietly while it is on.",
            Self::PressOil => "Presses vegetables, nuts or seeds into a bottle of oil.",
            Self::PressJuice => "Presses fruit into juice, in a cask or vessel you carry.",
            Self::Alchemy => "Works herbs and liquids through your apparatus, step by step.",
            Self::CollectWater => {
                "Fills your empty bottles, one after another, at the water you face."
            }
            Self::FellTree => "Fells the tree you face with the one-handed axe you hold.",
            Self::CutKindling => "Splits a log into kindling with the one-handed axe you hold.",
            Self::MakeCampfire => {
                "Lights a campfire on the ground you face with kindling, or feeds the fire you face."
            }
            Self::Cook => "Cooks food through your tools, step by step, in rows that can join.",
            Self::Magic => "Designs spells, step by step, for your spellbook.",
            Self::Ferment => {
                "Sets a cask of juice, or of water with fruit or grain, fermenting with yeast."
            }
        }
    }

    /// The Mana it spends, in whole points (`magic`).
    pub const fn mana_cost(self) -> u32 {
        match self {
            Self::Heal => crate::magic::HEAL_MANA,
            _ => 0,
        }
    }

    /// Whether it is done (or turned on and off) when asked, and so can go on
    /// the action bar.
    pub const fn is_active(self) -> bool {
        matches!(self.use_kind(), Use::Active | Use::Toggle)
    }

    /// Whether it is turned on and off rather than done.
    pub const fn is_toggle(self) -> bool {
        matches!(self.use_kind(), Use::Toggle)
    }

    /// Whether it may be made automatic on the action bar, to be done whenever it
    /// can be (owner, 2026-11-07): only what is safe to leave on, as it waits
    /// until it can be done. Attack (a target, in reach, an enemy) and Heal
    /// (on yourself, at a quarter of your Health) so far.
    pub const fn can_be_automatic(self) -> bool {
        matches!(self, Self::Attack | Self::Heal)
    }
}

#[cfg(test)]
mod tests;
