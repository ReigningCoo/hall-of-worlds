//! `Fruits`: every fruit and nut, found by key or tag. Built in only, from
//! `fruits.ron`: unlike herbs, no mods yet, so both sides have the same table
//! without it being sent.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::{FruitDef, FruitTag};

/// The fruits built into the game, as written.
const BUILT_IN: &str = include_str!("fruits.ron");

static BUILT_IN_TABLE: LazyLock<Fruits> = LazyLock::new(|| {
    let defs =
        Fruits::read(BUILT_IN).unwrap_or_else(|error| panic!("the built-in fruits: {error}"));
    Fruits::new(defs)
});

/// Every fruit and nut, in the table's order.
#[derive(Clone, Debug, Default)]
pub struct Fruits {
    defs: Vec<FruitDef>,
    by_key: BTreeMap<String, usize>,
    by_tag: BTreeMap<FruitTag, usize>,
}

impl Fruits {
    /// The fruits built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The fruits written in a RON file (a list of `FruitDef`s), each checked.
    pub fn read(text: &str) -> Result<Vec<FruitDef>, String> {
        let defs: Vec<FruitDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for def in &defs {
            def.check()?;
        }
        Ok(defs)
    }

    /// A table of these; a later one with a key already in it is left out.
    pub fn new(defs: Vec<FruitDef>) -> Self {
        let mut fruits = Self::default();
        for def in defs {
            if fruits.by_key.contains_key(&def.key) {
                continue;
            }
            let at = fruits.defs.len();
            fruits.by_key.insert(def.key.clone(), at);
            fruits.by_tag.insert(FruitTag::of(&def.key), at);
            fruits.defs.push(def);
        }
        fruits
    }

    /// The fruit with this key.
    pub fn get(&self, key: &str) -> Option<&FruitDef> {
        self.by_key.get(key).map(|&at| &self.defs[at])
    }

    /// The fruit with this tag (`FruitTag::of` its key).
    pub fn by_tag(&self, tag: FruitTag) -> Option<&FruitDef> {
        self.by_tag.get(&tag).map(|&at| &self.defs[at])
    }

    /// Every fruit, herbs' fruits too, in the table's order.
    pub fn iter(&self) -> impl Iterator<Item = &FruitDef> {
        self.defs.iter()
    }

    /// The fruits that are items of their own (`FruitDef::is_item`).
    pub fn items(&self) -> impl Iterator<Item = &FruitDef> {
        self.defs.iter().filter(|def| def.is_item())
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
