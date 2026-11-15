//! Sending the player their `Sheet`, always followed by the action clock's
//! tick (`ServerMessage::Clock`), so the client can tell when effects end.

use shared::protocol::ServerMessage;

use super::Client;

impl Client {
    /// Sends the character's sheet as the client may know it, then `now`.
    /// `false` if the client has gone.
    pub(crate) fn send_sheet(&self, now: u64) -> bool {
        self.link.send(&ServerMessage::Sheet(self.sheet.as_sent()))
            && self.link.send(&ServerMessage::Clock { tick: now })
    }
}
