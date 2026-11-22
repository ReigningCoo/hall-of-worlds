//! Developer tools: putting furniture on a level by hand (Place Furniture), to
//! look at every variation of each piece.

use serde::{Deserialize, Serialize};

use crate::furniture::Piece;

/// A piece for the server to put on the player's site level, replacing any it
/// overlaps (`ClientMessage::DevFurniture`). The server checks it is a piece
/// furnishing could make (`Piece::is_well_made`) standing on floor; it is not
/// kept (except a piece with places, `furniture::Holder`), and nothing checks
/// it cuts nothing off.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DevFurniture {
    pub piece: Piece,
}
