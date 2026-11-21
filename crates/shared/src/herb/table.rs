//! `Herbs`: every herb on a server, found by key. The ones built in come from
//! `herbs.ron`; a server's mods add to them or replace them by key, and it
//! sends the whole table to each client after `Welcome`.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use super::HerbDef;

/// The herbs built into the game, as written.
const BUILT_IN: &str = include_str!("herbs.ron");

static BUILT_IN_TABLE: LazyLock<Herbs> = LazyLock::new(|| {
    let defs = Herbs::read(BUILT_IN).unwrap_or_else(|error| panic!("the built-in herbs: {error}"));
    Herbs::from(defs)
});

/// Every herb, in the order they were added, found by key. Sent as just the list.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "Vec<HerbDef>", into = "Vec<HerbDef>")]
pub struct Herbs {
    defs: Vec<HerbDef>,
    /// Each key's place in `defs`.
    by_key: BTreeMap<String, usize>,
}

impl From<Vec<HerbDef>> for Herbs {
    fn from(defs: Vec<HerbDef>) -> Self {
        let mut herbs = Self::default();
        herbs.merge(defs);
        herbs
    }
}

impl From<Herbs> for Vec<HerbDef> {
    fn from(herbs: Herbs) -> Self {
        herbs.defs
    }
}

impl Herbs {
    /// The herbs built into the game.
    pub fn built_in() -> Self {
        BUILT_IN_TABLE.clone()
    }

    /// The herbs built into the game, shared, without a server's mods: for
    /// what needs only their names or data (cooking's dish names).
    pub fn built_in_ref() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The herbs written in a RON file (a list of `HerbDef`s), each checked
    /// (`HerbDef::check`): an error for a mod's author if any is wrong.
    pub fn read(text: &str) -> Result<Vec<HerbDef>, String> {
        let defs: Vec<HerbDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for def in &defs {
            def.check()?;
        }
        Ok(defs)
    }

    /// Adds these, each replacing any herb with its key where that one was, or
    /// else going at the end. How many replaced one.
    pub fn merge(&mut self, defs: Vec<HerbDef>) -> usize {
        let mut replaced = 0;
        for def in defs {
            match self.by_key.get(&def.key) {
                Some(&at) => {
                    self.defs[at] = def;
                    replaced += 1;
                }
                None => {
                    self.by_key.insert(def.key.clone(), self.defs.len());
                    self.defs.push(def);
                }
            }
        }
        replaced
    }

    /// The herb with this key.
    pub fn get(&self, key: &str) -> Option<&HerbDef> {
        self.by_key.get(key).map(|&at| &self.defs[at])
    }

    /// Every herb, in the order they were added.
    pub fn iter(&self) -> impl Iterator<Item = &HerbDef> {
        self.defs.iter()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
