//! The spellbook (magic plan PR 4): the spells one character has designed,
//! kept on the sheet as the recipe book is (`shared::recipe`). Each spell is
//! known by its design, so the same design is never kept twice; each has the
//! player's own name or none (the automatic one, `auto_name`, which the
//! client words), and may be a favourite. Favourites are listed first, then
//! the rest, each latest first. A new character's book holds Heal.

use serde::{Deserialize, Serialize};

use super::check::{DesignError, check};
use super::design::{Spell, Step};
use super::effect::Effect;
use crate::skill::Skills;

/// At most this many spells in a book (a first guess).
pub const MOST_SPELLS: usize = 100;

/// The longest name a player may give a spell, in characters.
pub const LONGEST_NAME: usize = 40;

/// The spells one character keeps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spellbook {
    /// Latest stored first, favourites mixed in (`iter` lists them first).
    spells: Vec<BookSpell>,
}

/// One spell in a book.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookSpell {
    pub spell: Spell,
    /// The player's own name, or `None` for the automatic one.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub favourite: bool,
}

/// Why a change to the book was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookRefusal {
    /// The design breaks the rules or isn't unlocked (`check`).
    Design(DesignError),
    /// The same design is in the book already.
    Known,
    /// The book holds `MOST_SPELLS` already.
    Full,
    /// No such spell in the book.
    NotInBook,
    /// The name is too long or holds control characters.
    BadName,
}

/// The Heal spell: one Heal, on the caster.
pub fn heal_spell() -> Spell {
    Spell::new(vec![Step::effects(&[(Effect::Heal, 1)])])
}

impl Default for Spellbook {
    /// A new character's (and an older save's): Heal.
    fn default() -> Self {
        Self {
            spells: vec![BookSpell {
                spell: heal_spell(),
                name: None,
                favourite: false,
            }],
        }
    }
}

impl Spellbook {
    /// A book with nothing in it.
    pub fn empty() -> Self {
        Self { spells: Vec::new() }
    }

    /// Stores a design, checked against the caster's skills, at the front,
    /// under the name given (tidied by `tidy_name`; `None` automatic).
    pub fn store(
        &mut self,
        spell: Spell,
        name: Option<&str>,
        skills: Option<&Skills>,
    ) -> Result<(), BookRefusal> {
        check(&spell, skills).map_err(BookRefusal::Design)?;
        let name = tidy_name(name)?;
        if self.find(&spell).is_some() {
            return Err(BookRefusal::Known);
        }
        if self.spells.len() >= MOST_SPELLS {
            return Err(BookRefusal::Full);
        }
        self.spells.insert(
            0,
            BookSpell {
                spell,
                name,
                favourite: false,
            },
        );
        Ok(())
    }

    /// Gives a spell the player's own name, or the automatic one with `None`
    /// (or a name of only spaces).
    pub fn rename(&mut self, spell: &Spell, name: Option<&str>) -> Result<(), BookRefusal> {
        let name = tidy_name(name)?;
        let index = self.find(spell).ok_or(BookRefusal::NotInBook)?;
        self.spells[index].name = name;
        Ok(())
    }

    /// Marks a spell a favourite, or not.
    pub fn set_favourite(&mut self, spell: &Spell, favourite: bool) -> Result<(), BookRefusal> {
        let index = self.find(spell).ok_or(BookRefusal::NotInBook)?;
        self.spells[index].favourite = favourite;
        Ok(())
    }

    /// Takes a spell out, giving it back (for the design panel to hold).
    pub fn remove(&mut self, spell: &Spell) -> Result<BookSpell, BookRefusal> {
        let index = self.find(spell).ok_or(BookRefusal::NotInBook)?;
        Ok(self.spells.remove(index))
    }

    /// Whether this design is in the book.
    pub fn knows(&self, spell: &Spell) -> bool {
        self.find(spell).is_some()
    }

    /// The spell of this design, if in the book.
    pub fn get(&self, spell: &Spell) -> Option<&BookSpell> {
        self.find(spell).map(|index| &self.spells[index])
    }

    /// Every spell, favourites first, each part latest stored first.
    pub fn iter(&self) -> impl Iterator<Item = &BookSpell> {
        let favourites = self.spells.iter().filter(|kept| kept.favourite);
        favourites.chain(self.spells.iter().filter(|kept| !kept.favourite))
    }

    pub fn len(&self) -> usize {
        self.spells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.spells.is_empty()
    }

    fn find(&self, spell: &Spell) -> Option<usize> {
        self.spells.iter().position(|kept| kept.spell == *spell)
    }
}

/// A name as kept: spaces trimmed from both ends, `None` when nothing is
/// left; refused when longer than `LONGEST_NAME` or holding control
/// characters.
pub fn tidy_name(name: Option<&str>) -> Result<Option<String>, BookRefusal> {
    let Some(name) = name.map(str::trim).filter(|name| !name.is_empty()) else {
        return Ok(None);
    };
    if name.chars().count() > LONGEST_NAME || name.chars().any(char::is_control) {
        return Err(BookRefusal::BadName);
    }
    Ok(Some(name.to_owned()))
}
