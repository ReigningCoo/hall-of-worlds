//! `HerbTable`: every herb on the server being played on (`shared::herb`), as
//! it last said (`ServerMessage::Herbs`, after `Welcome`): the game's own until
//! then. Set by `connection`; for whatever shows herbs.

use bevy::prelude::*;
use shared::herb::Herbs;

/// The herbs of the server being played on: its mods may add to or change the
/// game's own.
#[derive(Resource, Clone, Debug, Deref)]
pub struct HerbTable(pub Herbs);

impl Default for HerbTable {
    fn default() -> Self {
        Self(Herbs::built_in())
    }
}
