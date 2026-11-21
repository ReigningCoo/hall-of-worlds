//! `Dishes`: the real dishes a cooked output can be named after, built in
//! from `dishes.ron`.

use std::sync::LazyLock;

use super::DishDef;

/// The dishes built into the game, as written.
const BUILT_IN: &str = include_str!("dishes.ron");

static BUILT_IN_TABLE: LazyLock<Dishes> = LazyLock::new(|| {
    let defs =
        Dishes::read(BUILT_IN).unwrap_or_else(|error| panic!("the built-in dishes: {error}"));
    Dishes::new(defs)
});

/// Every dish, in the table's order (earlier wins a tie).
#[derive(Clone, Debug, Default)]
pub struct Dishes {
    defs: Vec<DishDef>,
}

impl Dishes {
    /// The dishes built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The dishes written in a RON file, each checked, each key once.
    pub fn read(text: &str) -> Result<Vec<DishDef>, String> {
        let defs: Vec<DishDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for (at, def) in defs.iter().enumerate() {
            def.check()?;
            if defs[..at].iter().any(|other| other.key == def.key) {
                return Err(format!("{}: written twice", def.key));
            }
        }
        Ok(defs)
    }

    pub fn new(defs: Vec<DishDef>) -> Self {
        Self { defs }
    }

    pub fn get(&self, key: &str) -> Option<&DishDef> {
        self.defs.iter().find(|def| def.key == key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &DishDef> {
        self.defs.iter()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
