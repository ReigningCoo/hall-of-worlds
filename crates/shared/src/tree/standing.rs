//! `Tree`: any tree that can be felled, wild or a fruit tree, and what it gives.

use serde::{Deserialize, Serialize};

use super::{TreeKind, TreeStage, fruit_wood};
use crate::material::Material;

/// A tree standing on a cell: a wild `Cell::Tree` by its kind, or a fruit
/// plant with a trunk by its fruit's key.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tree {
    Wild(TreeKind),
    Fruit(String),
}

impl Tree {
    /// Its wood: `None` for a trunk without true wood (banana, papaya,
    /// saguaro), which gives no logs.
    pub fn wood(&self) -> Option<Material> {
        match self {
            Self::Wild(kind) => Some(kind.def().wood),
            Self::Fruit(key) => fruit_wood(key).map(|(wood, _)| wood),
        }
    }

    /// How many logs felling it gives at `stage` (`TreeStage::logs`): 1 to 12
    /// for a grown tree, by its kind and size; none without wood.
    pub fn logs(&self, stage: TreeStage) -> u8 {
        let grown = match self {
            Self::Wild(kind) => kind.def().logs,
            Self::Fruit(key) => fruit_wood(key).map_or(0, |(_, logs)| logs),
        };
        stage.logs(grown)
    }
}
