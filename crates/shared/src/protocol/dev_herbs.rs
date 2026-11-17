//! What dev tools can do to what a character knows of herbs (the debug menu's
//! Herbs panel).

use serde::{Deserialize, Serialize};

/// What `ClientMessage::DevHerbs` does, answered with the character's `Sheet`
/// (`Sheet::herbs`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevHerbs {
    /// Learns the name and every effect of the herb with this key.
    Learn(String),
    /// Learns the name and every effect of every herb the server has.
    LearnAll,
    /// Forgets everything known of herbs.
    ForgetAll,
}
