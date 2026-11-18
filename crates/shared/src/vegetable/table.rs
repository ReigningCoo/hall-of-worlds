//! `Vegetables`: every vegetable, pulse and grain, found by key or tag. Built
//! in only, from `vegetables.ron`: like fruit, no mods yet, so both sides have
//! the same table without it being sent.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::{VegetableDef, VegetableTag};

/// The vegetables built into the game, as written.
const BUILT_IN: &str = include_str!("vegetables.ron");

static BUILT_IN_TABLE: LazyLock<Vegetables> = LazyLock::new(|| {
    let defs = Vegetables::read(BUILT_IN)
        .unwrap_or_else(|error| panic!("the built-in vegetables: {error}"));
    Vegetables::new(defs)
});

/// Every vegetable, pulse and grain, in the table's order.
#[derive(Clone, Debug, Default)]
pub struct Vegetables {
    defs: Vec<VegetableDef>,
    by_key: BTreeMap<String, usize>,
    by_tag: BTreeMap<VegetableTag, usize>,
}

impl Vegetables {
    /// The vegetables built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The vegetables written in a RON file (a list of `VegetableDef`s), each
    /// checked.
    pub fn read(text: &str) -> Result<Vec<VegetableDef>, String> {
        let defs: Vec<VegetableDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for def in &defs {
            def.check()?;
        }
        Ok(defs)
    }

    /// A table of these; a later one with a key already in it is left out.
    pub fn new(defs: Vec<VegetableDef>) -> Self {
        let mut vegetables = Self::default();
        for def in defs {
            if vegetables.by_key.contains_key(&def.key) {
                continue;
            }
            let at = vegetables.defs.len();
            vegetables.by_key.insert(def.key.clone(), at);
            vegetables.by_tag.insert(VegetableTag::of(&def.key), at);
            vegetables.defs.push(def);
        }
        vegetables
    }

    /// The vegetable with this key.
    pub fn get(&self, key: &str) -> Option<&VegetableDef> {
        self.by_key.get(key).map(|&at| &self.defs[at])
    }

    /// The vegetable with this tag (`VegetableTag::of` its key).
    pub fn by_tag(&self, tag: VegetableTag) -> Option<&VegetableDef> {
        self.by_tag.get(&tag).map(|&at| &self.defs[at])
    }

    /// Every vegetable, in the table's order.
    pub fn iter(&self) -> impl Iterator<Item = &VegetableDef> {
        self.defs.iter()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
