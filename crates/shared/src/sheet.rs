//! What a creature is like: attributes, skills, abilities and how it looks. See
//! `docs/context/shared/creature.md` and `docs/context/abilities-skills.md`.
//!
//! Attributes and skills are for gameplay that doesn't exist yet, apart from what
//! strength says about carrying. Appearance never affects the rules.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::alignment::Alignment;
use crate::bonus::Effects;
use crate::creature::CreatureKind;
use crate::gender::Gender;
use crate::skill::Skills;
use crate::vitals::Vitals;

mod attributes;
mod bonuses;
mod build;
mod fits;
mod foes;
mod from_build;
mod hair;
mod height;
mod kind_bonus;
mod load;
mod looks;
mod markings;
mod shapeshift;

pub use crate::ability::Ability;
pub use attributes::{Attribute, Attributes, Row};
pub use bonuses::Situation;
pub use build::{Build, Dot, POINTS_TOTAL, TURN_STEPS, WEIGHT_TOTAL};
pub use foes::{Foe, Foes};
pub use from_build::{POINT_WEIGHT, SPREAD};
pub use height::{HEIGHT_SHARES, HEIGHT_WHOLE};
pub use kind_bonus::{BONUS_POINTS, bonus_from};
pub use load::Load;
pub use looks::{Choice, Pattern};
pub use markings::{Face, Marks};
pub use shapeshift::TrueForm;

/// Something that differs between creatures of a kind, only to look at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Trait {
    CoatColour,
    FeatherColour,
    HairColour,
    Height,
    SkinShade,
    // New traits go at the end: messages carry a trait by its place in this list.
    HairStyle,
    Beard,
    Markings,
}

impl Trait {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CoatColour => "Coat",
            Self::FeatherColour => "Feathers",
            Self::HairColour => "Hair",
            Self::Height => "Height",
            Self::SkinShade => "Skin",
            Self::HairStyle => "Hair style",
            Self::Beard => "Beard",
            Self::Markings => "Markings",
        }
    }
}

/// How one creature looks: a choice for each trait, the first if not given.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Appearance(pub BTreeMap<Trait, u8>);

impl Appearance {
    /// What is chosen for this trait, if this kind has it: the first if not given.
    pub fn choice(&self, kind: CreatureKind, which: Trait) -> Option<Choice> {
        if !kind.def().looks.contains(&which) {
            return None;
        }
        let chosen = self.0.get(&which).copied().unwrap_or(0);
        which.choices_for(kind).get(chosen as usize).copied()
    }

    /// What this trait is, in words.
    pub fn says(&self, kind: CreatureKind, which: Trait) -> &'static str {
        self.choice(kind, which).map_or("", |choice| choice.name)
    }

    /// The colour its body is drawn in: its skin, or its coat for one with fur.
    pub fn skin(&self, kind: CreatureKind) -> Option<[u8; 3]> {
        [Trait::SkinShade, Trait::CoatColour]
            .into_iter()
            .find_map(|which| self.choice(kind, which)?.colour)
    }

    /// The colour of its coat, if it has one: a taur's lower body's.
    pub fn coat(&self, kind: CreatureKind) -> Option<[u8; 3]> {
        self.choice(kind, Trait::CoatColour)?.colour
    }

    /// Its markings, if it has any (`Trait::Markings`), or a brindled coat's
    /// stripes.
    pub fn marks(&self, kind: CreatureKind) -> Option<Marks> {
        let coat = self
            .choice(kind, Trait::CoatColour)
            .map(|choice| choice.pattern);
        if let Some(Pattern::Brindle(stripes)) = coat {
            return Some(Marks::brindle(stripes));
        }
        match self.choice(kind, Trait::Markings)?.pattern {
            Pattern::Marked(marks) => Some(marks),
            _ => None,
        }
    }

    /// The colour of its feathers, if it has any.
    pub fn feathers(&self, kind: CreatureKind) -> Option<[u8; 3]> {
        self.choice(kind, Trait::FeatherColour)?.colour
    }
}

/// How one creature feels about another: what its token's colour says to them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Feeling {
    /// It doesn't feel anything: another player.
    #[default]
    None,
    Friendly,
    Neutral,
    Unfriendly,
}

/// What a creature does on noticing another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reaction {
    Ignore,
    /// Goes up to it, and stays nearby.
    Approach,
    /// Runs at it. Nothing happens on arriving until there is combat.
    Attack,
    Flee,
}

/// What one creature is like: starts from its kind, and can grow from there.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sheet {
    /// What it is like now. Missing from saves made before there were nine
    /// (`settled` fills it in).
    #[serde(default)]
    pub attributes: Attributes,
    /// How its attributes were chosen, kept to show and to work them out again.
    #[serde(default)]
    pub build: Build,
    /// Its gender (`shared::gender`), whose bonus is in its attributes. Neutral in
    /// a save from before genders, whose attributes are kept with no bonus added
    /// (owner, 2026-11-12).
    #[serde(default)]
    pub gender: Gender,
    /// How good or evil it is (`shared::alignment`): secret, never sent to a
    /// client (`as_sent`). A player always has one (0 in a save from before
    /// alignments, `settled`); an animal has none.
    #[serde(default)]
    pub alignment: Option<Alignment>,
    /// What it has learned (`skill::Skills`): saves from before the tree are read
    /// by the old names.
    pub skills: Skills,
    pub appearance: Appearance,
    /// Health and Mana: full in a save from before them.
    #[serde(default)]
    pub vitals: Vitals,
    /// What it really is while shapeshifted into another kind's body (`shapeshift`).
    #[serde(default)]
    pub true_form: Option<TrueForm>,
    /// What it remembers of each kind of creature it has fought (`Foes`).
    #[serde(default)]
    pub foes: Foes,
    /// Bonuses laid on it for a while or for good (`shared::bonus`): none in
    /// older saves, whose changes to the most `settled` turns into these.
    #[serde(default)]
    pub effects: Effects,
    /// What it has found out about herbs (`shared::herb`): nothing in older saves.
    #[serde(default)]
    pub herbs: crate::herb::HerbKnowledge,
    /// The abilities it has learned (`ability::Learned`), beside those its body
    /// and skills give it: none in older saves.
    #[serde(default)]
    pub abilities: crate::ability::Learned,
    /// The mixes it has ground before, for making again (`brewing::Recipes`):
    /// none in older saves.
    #[serde(default)]
    pub recipes: crate::brewing::Recipes,
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new(CreatureKind::default())
    }
}

impl Sheet {
    /// A plain one of this kind: Neutral, its dots in the middle, Body at the top.
    pub fn new(kind: CreatureKind) -> Self {
        Self::built(kind, Gender::default(), Build::CENTRED)
    }

    /// One of this kind and gender, its attributes chosen with these triangles,
    /// which must be valid (`Build::is_valid`).
    pub fn built(kind: CreatureKind, gender: Gender, build: Build) -> Self {
        Self {
            attributes: Attributes::for_kind(kind, gender, &build),
            build,
            gender,
            alignment: Some(Alignment::NEUTRAL),
            skills: Skills::starting(kind),
            appearance: Appearance::default(),
            vitals: Vitals::default(),
            true_form: None,
            foes: Foes::default(),
            effects: Effects::default(),
            herbs: Default::default(),
            abilities: Default::default(),
            recipes: Default::default(),
        }
    }

    /// As loaded, its body of kind `kind`: a save from before the nine attributes has
    /// none, so they are worked out again from its build (the centre, for such a save);
    /// one from before alignments starts at 0, and one from before effects has its
    /// changes to the most turned into effects.
    pub fn settled(mut self, kind: CreatureKind) -> Self {
        self.alignment.get_or_insert(Alignment::NEUTRAL);
        self.fold_old_changes();
        if self.attributes == Attributes::default() {
            if !self.build.is_valid() {
                self.build = Build::CENTRED;
            }
            // Its own kind's, shapeshifted again if it was.
            let own = self.own_kind(kind);
            self.true_form = None;
            self.attributes = Attributes::for_kind(own, self.gender, &self.build);
            if own != kind {
                self.shapeshift(own, kind);
            }
        }
        self
    }

    /// A copy to send its player, without what they may not know: their
    /// alignment. Every `Sheet` the server sends goes through this.
    pub fn as_sent(&self) -> Self {
        Self {
            alignment: None,
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests;
