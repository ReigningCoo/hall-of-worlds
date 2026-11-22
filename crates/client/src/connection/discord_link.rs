//! What the server says of Discord and the account logged in to
//! (`DiscordLink`), kept for Character Select's Link Discord
//! (`common::AccountDiscord`): forgotten on a new connection's
//! `LoginOptions`.

use bevy::prelude::*;
use shared::protocol::ServerMessage;

use super::FromServer;
use crate::common::AccountDiscord;

pub(super) fn keep_discord_link(
    mut from_server: MessageReader<FromServer>,
    mut discord: ResMut<AccountDiscord>,
) {
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::LoginOptions { .. } => {
                discord.set_if_neq(AccountDiscord(None));
            }
            // Set even when the same, so a refused link shows again.
            ServerMessage::DiscordLink(told) => discord.0 = Some(told.clone()),
            _ => {}
        }
    }
}
