//! Knowing fungi by name (fungi plan PR 7, owner's decision 10: never wrong
//! information): what a character may be shown a fungus as (`Shown`), and the
//! Mycology check picking one makes to learn its name (`names_on_picking`).

use super::{Fungi, FungusDef, FungusKnowledge, Group};
use crate::herb::skill_percent;

/// The skill of knowing fungi: picking one practises it and its check teaches
/// the name; it helps recognise a poison on eating too.
pub const MYCOLOGY: &str = "gathering/mycology";

/// The chance of learning a fungus's name on picking with no skill at all,
/// in percent: first guesses.
pub const NAME_BASE: u32 = 20;

/// Never more likely than this, in percent.
pub const NAME_MOST: u32 = 95;

/// What a character may be shown a fungus as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shown<'a> {
    /// Its own name: it knows it.
    Known(&'a FungusDef),
    /// "Looks like X": it knows the name of X, one of its lookalikes (the
    /// first known, in the table's order of `lookalikes`).
    LooksLike(&'a FungusDef),
    /// "Unknown mushroom", or "Unknown lichen" when `lichen`.
    Unknown { lichen: bool },
}

impl FungusKnowledge {
    /// How `def` may be shown to whoever knows this: by name if known, else
    /// as looking like a lookalike whose name is known, else unknown.
    pub fn shown<'a>(&self, def: &'a FungusDef, fungi: &'a Fungi) -> Shown<'a> {
        if self.knows_name(&def.key) {
            return Shown::Known(def);
        }
        let looks_like = (def.lookalikes.iter())
            .filter(|key| self.knows_name(key))
            .find_map(|key| fungi.get(key));
        match looks_like {
            Some(other) => Shown::LooksLike(other),
            None => Shown::Unknown {
                lichen: def.group == Group::Lichen,
            },
        }
    }
}

/// The chance in percent of learning a fungus's name on picking it, by the
/// picker's Mycology and Foraging values (`herb::skill_value`): `NAME_BASE`
/// plus Mycology's percent plus half Foraging's, at most `NAME_MOST`.
pub fn name_chance(mycology: u32, foraging: u32) -> u32 {
    (NAME_BASE + skill_percent(mycology) + skill_percent(foraging) / 2).min(NAME_MOST)
}

/// Whether picking teaches the name, `roll` from 0 to 99.
pub fn names_on_picking(mycology: u32, foraging: u32, roll: u32) -> bool {
    roll < name_chance(mycology, foraging)
}
