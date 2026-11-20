//! `Fungi`: every mushroom, fungus and lichen, found by key or tag. Built in
//! only, from `fungi.ron`: like vegetables, no mods yet, so both sides have the
//! same table without it being sent.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::{FungusDef, FungusTag};

/// The fungi built into the game, as written.
const BUILT_IN: &str = include_str!("fungi.ron");

static BUILT_IN_TABLE: LazyLock<Fungi> = LazyLock::new(|| {
    let defs = Fungi::read(BUILT_IN).unwrap_or_else(|error| panic!("the built-in fungi: {error}"));
    Fungi::new(defs)
});

/// Every fungus, in the table's order.
#[derive(Clone, Debug, Default)]
pub struct Fungi {
    defs: Vec<FungusDef>,
    by_key: BTreeMap<String, usize>,
    by_tag: BTreeMap<FungusTag, usize>,
}

impl Fungi {
    /// The fungi built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The fungi written in a RON file (a list of `FungusDef`s), each checked,
    /// with their lookalikes in the same file.
    pub fn read(text: &str) -> Result<Vec<FungusDef>, String> {
        let defs: Vec<FungusDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for def in &defs {
            def.check()?;
            let known = |key: &String| defs.iter().any(|other| other.key == *key);
            if let Some(missing) = def.lookalikes.iter().find(|key| !known(key)) {
                return Err(format!("{}: no lookalike {missing}", def.key));
            }
        }
        Ok(defs)
    }

    /// A table of these; a later one with a key already in it is left out.
    pub fn new(defs: Vec<FungusDef>) -> Self {
        let mut fungi = Self::default();
        for def in defs {
            if fungi.by_key.contains_key(&def.key) {
                continue;
            }
            let at = fungi.defs.len();
            fungi.by_key.insert(def.key.clone(), at);
            fungi.by_tag.insert(FungusTag::of(&def.key), at);
            fungi.defs.push(def);
        }
        fungi
    }

    /// The fungus with this key.
    pub fn get(&self, key: &str) -> Option<&FungusDef> {
        self.by_key.get(key).map(|&at| &self.defs[at])
    }

    /// The fungus with this tag (`FungusTag::of` its key).
    pub fn by_tag(&self, tag: FungusTag) -> Option<&FungusDef> {
        self.by_tag.get(&tag).map(|&at| &self.defs[at])
    }

    /// Every fungus, in the table's order.
    pub fn iter(&self) -> impl Iterator<Item = &FungusDef> {
        self.defs.iter()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
