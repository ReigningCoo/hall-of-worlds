//! `Behaviours`: every race's behaviour on a server, found by race. The game's
//! own come from `behaviours.ron`, one for every race; a server's mods replace
//! them by race. Never sent to clients.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::BehaviourDef;
use crate::creature::CreatureKind;

/// The behaviours built into the game, as written.
const BUILT_IN: &str = include_str!("behaviours.ron");

static BUILT_IN_TABLE: LazyLock<Behaviours> = LazyLock::new(|| {
    let defs = Behaviours::read(BUILT_IN)
        .unwrap_or_else(|error| panic!("the built-in behaviours: {error}"));
    let mut table = Behaviours::default();
    table.merge(defs);
    if let Some(missing) = CreatureKind::ALL
        .iter()
        .find(|kind| !table.by_kind.contains_key(kind))
    {
        panic!("the built-in behaviours have none for {missing:?}");
    }
    table
});

/// Every race's behaviour, by race.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Behaviours {
    by_kind: BTreeMap<CreatureKind, BehaviourDef>,
}

impl Behaviours {
    /// The game's own behaviours.
    pub fn built_in() -> Self {
        BUILT_IN_TABLE.clone()
    }

    /// The game's own behaviours, shared, without a server's mods.
    pub fn built_in_ref() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The behaviours written in a RON file (a list of `BehaviourDef`s), each
    /// checked (`BehaviourDef::check`): an error for a mod's author if any is
    /// wrong, or if one race is written twice.
    pub fn read(text: &str) -> Result<Vec<BehaviourDef>, String> {
        let defs: Vec<BehaviourDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for (at, def) in defs.iter().enumerate() {
            def.check()?;
            if defs[..at].iter().any(|before| before.kind == def.kind) {
                return Err(format!("{:?} is written twice", def.kind));
            }
        }
        Ok(defs)
    }

    /// Adds these, each replacing its race's. How many replaced one.
    pub fn merge(&mut self, defs: Vec<BehaviourDef>) -> usize {
        defs.into_iter()
            .filter_map(|def| self.by_kind.insert(def.kind, def))
            .count()
    }

    /// A race's behaviour (the game's own, if this table lacks it).
    pub fn of(&self, kind: CreatureKind) -> &BehaviourDef {
        match self.by_kind.get(&kind) {
            Some(def) => def,
            None => &BUILT_IN_TABLE.by_kind[&kind],
        }
    }

    /// Every race's behaviour, by race.
    pub fn iter(&self) -> impl Iterator<Item = &BehaviourDef> {
        self.by_kind.values()
    }

    pub fn len(&self) -> usize {
        self.by_kind.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_kind.is_empty()
    }
}
