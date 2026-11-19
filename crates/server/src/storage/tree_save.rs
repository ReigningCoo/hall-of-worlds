//! What is kept of one tree's woodcutting (`LevelSave::trees`).

use serde::{Deserialize, Serialize};

/// One tree's woodcutting, as saved: damage from felling cut short, and the
/// game minute it was felled, if it was. Dropped once neither matters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeSave {
    #[serde(default)]
    pub damage: Option<shared::woodcutting::Damage>,
    #[serde(default)]
    pub felled: Option<u64>,
}
