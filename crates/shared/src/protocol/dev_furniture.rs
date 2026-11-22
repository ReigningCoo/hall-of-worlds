//! Developer tools: putting furniture on a level by hand (Place Furniture), to
//! look at every variation of each piece.

use serde::{Deserialize, Serialize};

use crate::decor::Decor;
use crate::furniture::Piece;

/// What Place Furniture asks the server to put on the player's site level
/// (`ClientMessage::DevFurniture`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevFurniture {
    /// A piece, replacing any it overlaps. The server checks it is a piece
    /// furnishing could make (`Piece::is_well_made`) standing on floor; it is
    /// not kept (except a piece with places, `furniture::Holder`), and nothing
    /// checks it cuts nothing off.
    Piece(Piece),
    /// Decor (tavern furniture plan PR 5), replacing what was in its place, if
    /// it fits there (`Decor::fits`, off the stairs). Not kept.
    Decor(Decor),
}
