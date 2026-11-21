//! `RecipeBook`: the recipes one character knows, saved on their sheet
//! (`Sheet::book`), latest first. Learnt ones come from reading a scroll
//! (which is used up) or from elsewhere later; own ones are every process
//! they cooked well, the same process again moving to the front, at most
//! `MOST_OWN` kept. Removing a learnt one read from a scroll gives the scroll
//! back.

use serde::{Deserialize, Serialize};

use super::{Craft, RecipeForm};
use crate::cooking::{DishName, Recipe};
use crate::item::Item;

/// The most own recipes kept: the oldest go first.
pub const MOST_OWN: usize = 100;

/// A recipe a character worked out themselves: what they did, and what it
/// was named as when made (generic dishes too).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Own {
    Cooking { recipe: Recipe, name: DishName },
}

impl Own {
    pub fn craft(&self) -> Craft {
        match self {
            Self::Cooking { .. } => Craft::Cooking,
        }
    }

    /// Whether it is the same process as `other`, whatever each was named.
    pub fn same(&self, other: &Own) -> bool {
        match (self, other) {
            (Self::Cooking { recipe: a, .. }, Self::Cooking { recipe: b, .. }) => a == b,
        }
    }
}

/// One recipe in the book.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Entry {
    /// A real dish's recipe, learnt: from a scroll (given back if removed)
    /// or not.
    Learnt {
        craft: Craft,
        dish: String,
        scroll: bool,
    },
    /// A process the character cooked.
    Own(Own),
}

impl Entry {
    /// Whether it is the same recipe as `other`: a learnt one by its craft
    /// and dish, an own one by its process.
    pub fn same(&self, other: &Entry) -> bool {
        match (self, other) {
            (
                Self::Learnt { craft, dish, .. },
                Self::Learnt {
                    craft: c, dish: d, ..
                },
            ) => craft == c && dish == d,
            (Self::Own(a), Self::Own(b)) => a.same(b),
            _ => false,
        }
    }
}

/// Why a scroll can't be read into the book.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refused {
    /// It isn't a recipe scroll.
    NotAScroll,
    /// Its dish has no recipe (a server's table without it).
    Unknown,
    /// The book has it already: the scroll is kept.
    Known,
}

/// What removing a recipe gives back: the scroll it was read from, if any.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Removed {
    pub scroll: Option<RecipeForm>,
}

/// Every recipe one character knows, latest first: none in older saves.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecipeBook(Vec<Entry>);

impl RecipeBook {
    /// Reads a scroll into the book, to the front. The caller uses the
    /// scroll up when this succeeds.
    pub fn read(&mut self, scroll: &Item) -> Result<(), Refused> {
        let form = scroll.recipe().ok_or(Refused::NotAScroll)?;
        if !form.is_known() {
            return Err(Refused::Unknown);
        }
        match self.learn(form, true) {
            true => Ok(()),
            false => Err(Refused::Known),
        }
    }

    /// Learns `form`'s recipe, from a scroll or not; `false` if known.
    pub fn learn(&mut self, form: &RecipeForm, scroll: bool) -> bool {
        if self.knows(form) {
            return false;
        }
        let entry = Entry::Learnt {
            craft: form.craft,
            dish: form.dish.clone(),
            scroll,
        };
        self.0.insert(0, entry);
        true
    }

    /// Whether `form`'s recipe is learnt.
    pub fn knows(&self, form: &RecipeForm) -> bool {
        self.0.iter().any(|entry| {
            matches!(entry, Entry::Learnt { craft, dish, .. }
                if *craft == form.craft && *dish == form.dish)
        })
    }

    /// Remembers a process just made well: to the front, replacing the same
    /// process made before, the oldest own ones beyond `MOST_OWN` dropped.
    pub fn remember(&mut self, own: Own) {
        self.0
            .retain(|entry| !matches!(entry, Entry::Own(old) if old.same(&own)));
        self.0.insert(0, Entry::Own(own));
        let mut kept = 0;
        self.0.retain(|entry| match entry {
            Entry::Own(_) => {
                kept += 1;
                kept <= MOST_OWN
            }
            Entry::Learnt { .. } => true,
        });
    }

    /// Remembers a cooking process (`Recipe::normalised`) and what it made.
    pub fn remember_cooking(&mut self, recipe: Recipe, name: DishName) {
        let recipe = recipe.normalised();
        self.remember(Own::Cooking { recipe, name });
    }

    /// Takes `entry` out of the book: what comes back, or `None` if it isn't
    /// in it.
    pub fn remove(&mut self, entry: &Entry) -> Option<Removed> {
        let at = self.0.iter().position(|e| e.same(entry))?;
        let scroll = match self.0.remove(at) {
            Entry::Learnt {
                craft,
                dish,
                scroll: true,
            } => Some(RecipeForm { craft, dish }),
            _ => None,
        };
        Some(Removed { scroll })
    }

    /// Every recipe, latest first.
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
