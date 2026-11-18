//! What a character has made before, for making again (PR 6): each mix ground
//! (its herbs by key and its oil) with what it gave last time, saved on its
//! sheet (`Sheet::recipes`), latest first; and what its ointment last brewed
//! into (PR 7).

use serde::{Deserialize, Serialize};

use crate::potion::{Contents, Oil, Water};

/// The most recipes kept: the oldest go first.
pub const MOST_RECIPES: usize = 20;

/// One mix made before: its herbs (keys, sorted) and oil, and the doses the
/// last one made held (they change with skill and the herbs' quality).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Recipe {
    pub herbs: Vec<String>,
    pub oil: Oil,
    pub made: Contents,
    /// What its ointment last brewed into, and with which water: none until
    /// brewed (and in older saves).
    #[serde(default)]
    pub brewed: Option<BrewedWith>,
}

/// An ointment brewed: the water and the potion (`None` when none formed).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BrewedWith {
    pub water: Water,
    pub potion: Option<Contents>,
}

impl Recipe {
    /// A recipe of these herbs and this oil, with what it made.
    pub fn new(herbs: impl IntoIterator<Item = String>, oil: Oil, made: Contents) -> Self {
        let mut herbs: Vec<String> = herbs.into_iter().collect();
        herbs.sort();
        herbs.dedup();
        Self {
            herbs,
            oil,
            made,
            brewed: None,
        }
    }

    /// Whether it is the same mix as `other`, whatever each made.
    pub fn same_mix(&self, other: &Recipe) -> bool {
        self.herbs == other.herbs && self.oil == other.oil
    }
}

/// The mixes one character has made, latest first: none in older saves.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Recipes(Vec<Recipe>);

impl Recipes {
    /// Remembers a mix just made: to the front, replacing the same mix made
    /// before, at most `MOST_RECIPES` kept.
    pub fn remember(&mut self, recipe: Recipe) {
        self.0.retain(|old| !old.same_mix(&recipe));
        self.0.insert(0, recipe);
        self.0.truncate(MOST_RECIPES);
    }

    /// Notes what the ointment `made` brewed into on the latest mix that made
    /// it, if one is remembered.
    pub fn brewed(&mut self, made: &Contents, brewed: BrewedWith) {
        if let Some(recipe) = self.0.iter_mut().find(|recipe| &recipe.made == made) {
            recipe.brewed = Some(brewed);
        }
    }

    /// Every one, latest first.
    pub fn iter(&self) -> impl Iterator<Item = &Recipe> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
