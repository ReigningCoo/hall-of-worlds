//! Link Discord, beside Change User: offered while the server says the
//! account can link it (`common::AccountDiscord`), "Discord linked" once it
//! leads there too, and why not in red if linking didn't work. Pressing it
//! sends `LoginChoice::LinkDiscord` (`presses`, seen here too), and the server's page opens
//! in the browser (`connection`); until the server answers, this says to
//! finish there. While the server says it must be linked (`MustLink`), Play
//! is greyed and "+" does nothing (`LinkFirst`), until it is.

use bevy::prelude::*;
use shared::protocol::DiscordLink;

use crate::common::{AccountDiscord, LoginChoice};
use crate::dom::Dom;

/// Link Discord was pressed and the server hasn't answered yet.
#[derive(Resource, Default)]
pub(super) struct Linking(pub bool);

/// The server is only for its Discord's members, and this account must link
/// Discord before playing.
#[derive(Resource, Default)]
pub(super) struct LinkFirst(pub bool);

/// Shows Link Discord, what is waiting, or that Discord is linked.
pub(super) fn show_discord(
    discord: Res<AccountDiscord>,
    mut choices: MessageReader<LoginChoice>,
    mut linking: ResMut<Linking>,
    mut first: ResMut<LinkFirst>,
    mut dom: Dom,
) {
    if choices
        .read()
        .any(|choice| *choice == LoginChoice::LinkDiscord)
    {
        linking.0 = true;
    }
    if discord.is_changed() {
        linking.0 = false;
        match discord.0 {
            Some(DiscordLink::MustLink) => first.0 = true,
            Some(DiscordLink::Failed(_)) => {}
            _ => first.0 = false,
        }
        dom.set_flag("character_select/play", "disabled", first.0);
    } else if !linking.is_changed() {
        return;
    }
    let (offered, said, failed) = shown(discord.0.as_ref(), linking.0);
    dom.set_flag("character_select/link_discord", "hidden", !offered);
    dom.set_flag("character_select/discord", "hidden", said.is_none());
    dom.set_text("character_select/discord", said.unwrap_or_default());
    dom.set_flag(
        "character_select/discord_failed",
        "hidden",
        failed.is_none(),
    );
    dom.set_text(
        "character_select/discord_failed",
        failed.unwrap_or_default(),
    );
}

/// Whether Link Discord is offered, what is said beside it, and why linking
/// didn't work.
fn shown(discord: Option<&DiscordLink>, linking: bool) -> (bool, Option<String>, Option<String>) {
    match discord {
        Some(DiscordLink::Linked) => (false, Some("Discord linked".into()), None),
        _ if linking => {
            let said = "Log in with Discord in your browser to link it.";
            (false, Some(said.into()), None)
        }
        Some(DiscordLink::CanLink) => (true, None, None),
        Some(DiscordLink::MustLink) => {
            let said = "This server is for its Discord's members: link Discord to play.";
            (true, Some(said.into()), None)
        }
        Some(DiscordLink::Failed(reason)) => {
            let failed = format!("Linking Discord didn't work: {reason}.");
            (true, None, Some(failed))
        }
        None => (false, None, None),
    }
}

#[cfg(test)]
mod tests;
