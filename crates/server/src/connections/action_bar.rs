//! A character's action bar: which abilities are in its ten slots and which are
//! used by themselves, changed by the player and checked by the shared rules.

use bevy::prelude::*;
use shared::ability::{BarChange, known};
use shared::protocol::ServerMessage;

use super::Client;

impl Client {
    /// The player asks to change their bar: made if the rules allow, for the body
    /// they are in now and what they have learned. Either way the client is told the bar as it is, so a
    /// refused change is put right. `false` if the client has gone.
    pub(super) fn change_bar(&mut self, change: BarChange) -> bool {
        let known = known(self.inventory.creature, &self.sheet);
        match self.action_bar.apply(change, &known, &self.sheet.spellbook) {
            Ok(()) => self.unsaved = true,
            Err(why) => debug!("Refused an action bar change {change:?}: {why:?}"),
        }
        self.send_action_bar()
    }

    /// Tells the client their action bar.
    pub(super) fn send_action_bar(&self) -> bool {
        self.link
            .send(&ServerMessage::ActionBar(self.action_bar.clone()))
    }
}
