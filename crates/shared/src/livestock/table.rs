//! `Livestock`: every kept animal, found by key or creature kind. Built in
//! only, from `livestock.ron`, so both sides have the same table without it
//! being sent.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::LivestockDef;
use crate::creature::CreatureKind;

/// The animals built into the game, as written.
const BUILT_IN: &str = include_str!("livestock.ron");

static BUILT_IN_TABLE: LazyLock<Livestock> = LazyLock::new(|| {
    let defs =
        Livestock::read(BUILT_IN).unwrap_or_else(|error| panic!("the built-in livestock: {error}"));
    Livestock::new(defs)
});

/// Every kept animal, in the table's order.
#[derive(Clone, Debug, Default)]
pub struct Livestock {
    defs: Vec<LivestockDef>,
    by_key: BTreeMap<String, usize>,
}

impl Livestock {
    /// The animals built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The animals written in a RON file, each checked: keys and kinds used
    /// once, variants of an animal in the same file that is no variant itself.
    pub fn read(text: &str) -> Result<Vec<LivestockDef>, String> {
        let defs: Vec<LivestockDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for (at, def) in defs.iter().enumerate() {
            def.check()?;
            let earlier = &defs[..at];
            if earlier.iter().any(|other| other.key == def.key) {
                return Err(format!("{}: written twice", def.key));
            }
            if def.kind.is_some() && earlier.iter().any(|other| other.kind == def.kind) {
                return Err(format!("{}: its kind is another's", def.key));
            }
            if let Some(of) = &def.variant_of {
                let found = defs.iter().find(|other| other.key == *of);
                if found.is_none_or(|other| other.variant_of.is_some() || other.key == def.key) {
                    return Err(format!("{}: no animal {of} to be a variant of", def.key));
                }
            }
        }
        Ok(defs)
    }

    /// A table of these; a later one with a key already in it is left out.
    pub fn new(defs: Vec<LivestockDef>) -> Self {
        let mut table = Self::default();
        for def in defs {
            if table.by_key.contains_key(&def.key) {
                continue;
            }
            table.by_key.insert(def.key.clone(), table.defs.len());
            table.defs.push(def);
        }
        table
    }

    /// The animal with this key.
    pub fn get(&self, key: &str) -> Option<&LivestockDef> {
        self.by_key.get(key).map(|&at| &self.defs[at])
    }

    /// The animal this creature kind is, if it is one.
    pub fn by_kind(&self, kind: CreatureKind) -> Option<&LivestockDef> {
        self.defs.iter().find(|def| def.kind == Some(kind))
    }

    /// The animals that are variants of this one.
    pub fn variants<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a LivestockDef> {
        self.defs
            .iter()
            .filter(move |def| def.variant_of.as_deref() == Some(key))
    }

    /// Every animal, in the table's order.
    pub fn iter(&self) -> impl Iterator<Item = &LivestockDef> {
        self.defs.iter()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
