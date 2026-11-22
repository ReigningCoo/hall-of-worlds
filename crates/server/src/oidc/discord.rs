//! Discord as a way of logging in: plain OAuth2 rather than OIDC, so its pages
//! are fixed instead of discovered, and it vouches for a Discord user id
//! (`/users/@me`, scope `identify`) instead of an email address. Set up by
//! `HOW_DISCORD_CLIENT_ID` and `HOW_DISCORD_CLIENT_SECRET` (or `_FILE`), with
//! `HOW_DISCORD_INVITE` for a "Join our Discord" button and `HOW_DISCORD_ONLY`
//! to offer nothing else.

use std::fmt;

use serde_json::Value;

use super::config::secure_enough;
use crate::environment::{option_value, secret_value};

/// Where Discord's pages are.
pub(super) const DISCORD_SITE: &str = "https://discord.com";

/// Discord as the environment sets it up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DiscordSettings {
    pub client_id: String,
    pub client_secret: String,
    /// The invitation to the server's Discord server, if it gives one.
    pub invite: Option<String>,
    /// Where Discord's pages are: `DISCORD_SITE`, or a fake one in tests.
    pub site: String,
}

/// Discord's settings in `variables` (the environment's), if it is set up,
/// whether only Discord is offered (`HOW_DISCORD_ONLY`), and warnings for what
/// was left out and why.
pub(super) fn discord_from(
    variables: &[(String, String)],
) -> (Option<DiscordSettings>, bool, Vec<String>) {
    let value = |name: &str| {
        let found = variables.iter().find(|(key, _)| key == name);
        found.map(|(_, value)| value.clone())
    };
    let get = |name: &str| {
        value(name)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    };
    let only = option_value(value("HOW_DISCORD_ONLY").as_deref(), false);
    let mut warnings = Vec::new();
    let client_secret = match secret_value("HOW_DISCORD_CLIENT_SECRET", value) {
        Ok(secret) => secret,
        Err(problem) => {
            warnings.push(format!("{problem}, so Discord logins are off"));
            return (None, only, warnings);
        }
    };
    let client_id = get("HOW_DISCORD_CLIENT_ID");
    let (Some(client_id), Some(client_secret)) = (client_id, client_secret) else {
        if get("HOW_DISCORD_CLIENT_ID").is_some() || get("HOW_DISCORD_INVITE").is_some() {
            warnings.push(
                "HOW_DISCORD_CLIENT_ID or HOW_DISCORD_CLIENT_SECRET (or \
                 HOW_DISCORD_CLIENT_SECRET_FILE) isn't set, so Discord logins are off"
                    .into(),
            );
        }
        return (None, only, warnings);
    };
    let invite = get("HOW_DISCORD_INVITE").filter(|invite| match secure_enough(invite) {
        Ok(()) => true,
        Err(problem) => {
            warnings.push(format!("HOW_DISCORD_INVITE: {problem}, so it isn't shown"));
            false
        }
    });
    let settings = DiscordSettings {
        client_id,
        client_secret,
        invite,
        site: DISCORD_SITE.into(),
    };
    (Some(settings), only, warnings)
}

/// A Discord user, as Discord vouched for them: only for this login, never
/// kept but for the id (`AccountSave::discord`) and never logged.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct DiscordUser {
    /// Discord's id for them, which never changes.
    pub id: u64,
    /// Their Discord name, for their own client's menus only.
    pub name: String,
}

/// By hand, so neither reaches a log.
impl fmt::Debug for DiscordUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DiscordUser(hidden)")
    }
}

/// The user in Discord's answer to `/users/@me`.
pub(super) fn discord_user(info: &Value) -> Result<DiscordUser, String> {
    let id = info["id"]
        .as_str()
        .and_then(|id| id.parse().ok())
        .ok_or("Discord gave no user id")?;
    let name = info["username"]
        .as_str()
        .filter(|name| !name.trim().is_empty())
        .ok_or("Discord gave no user name")?;
    Ok(DiscordUser {
        id,
        name: name.trim().to_owned(),
    })
}
