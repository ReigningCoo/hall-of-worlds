//! Notes for new players (`ServerMessage::Tutorial`): the server decides when
//! one is wanted, the client words and shows it.

use serde::{Deserialize, Serialize};

/// Which note.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tutorial {
    /// A new character's first arrival in The Way Inn (`shared::inn`): what
    /// the inn is, that its door opens onto the Hall of Worlds, and how to
    /// come back (off the walkway's edge halfway between two gates).
    WayInn,
}
