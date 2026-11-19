//! `Tree`: any tree that can be felled, wild or a fruit tree, and what it gives.

use serde::{Deserialize, Serialize};

use super::{TreeKind, TreeStage, fruit_kindling, fruit_wood};
use crate::fruit::FruitStage;
use crate::material::Material;

/// A tree standing on a cell: a wild `Cell::Tree` by its kind, or a fruit
/// plant with a trunk by its fruit's key.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tree {
    Wild(TreeKind),
    Fruit(String),
}

impl Tree {
    /// Its wood, for its logs or kindling: `None` only for a fruit plant
    /// without a trunk.
    pub fn wood(&self) -> Option<Material> {
        match self {
            Self::Wild(kind) => Some(kind.def().wood),
            Self::Fruit(key) => fruit_wood(key)
                .or_else(|| fruit_kindling(key))
                .map(|(wood, _)| wood),
        }
    }

    /// How many logs felling it gives at `stage` (`TreeStage::logs`): 1 to 12
    /// for a grown tree, by its kind and size; none without true wood.
    pub fn logs(&self, stage: TreeStage) -> u8 {
        let grown = match self {
            Self::Wild(kind) => kind.def().logs,
            Self::Fruit(key) => fruit_wood(key).map_or(0, |(_, logs)| logs),
        };
        stage.logs(grown)
    }

    /// How many pieces of kindling felling it gives at `stage`, for a trunk
    /// without true wood (banana, papaya, saguaro: 5 to 15 grown, fewer
    /// younger, as logs are); none for any other, which gives logs.
    pub fn kindling(&self, stage: TreeStage) -> u8 {
        match self {
            Self::Fruit(key) => fruit_kindling(key).map_or(0, |(_, pieces)| stage.logs(pieces)),
            Self::Wild(_) => 0,
        }
    }

    /// How grown a standing fruit tree counts as for felling, from its own
    /// stage: young in its first round (a quarter of the logs), else grown.
    pub fn fruit_stage(stage: FruitStage) -> TreeStage {
        match stage {
            FruitStage::Young => TreeStage::Young,
            _ => TreeStage::Grown,
        }
    }
}
