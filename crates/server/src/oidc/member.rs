//! Membership of the server's Discord server: required when
//! `HOW_DISCORD_SERVER_ID` is set, checked by a bot (`HOW_DISCORD_BOT_TOKEN`,
//! or `_FILE`) asking Discord for one member at a time over its REST API
//! (`members.rs`). No privileged intents are needed for that, and no player's
//! own token is kept. What Discord answers is read here (`read`).

use serde_json::Value;

use crate::environment::secret_value;

/// The Discord server players must be on, and the bot that checks.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct MemberSettings {
    /// The Discord server's id (Discord's "guild").
    pub server: u64,
    pub bot_token: String,
}

/// By hand, so the token never reaches a log.
impl std::fmt::Debug for MemberSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemberSettings {{ server: {}, bot_token: hidden }}",
            self.server
        )
    }
}

/// The membership required by `variables` (the environment's), if any. An
/// error if it is asked for but can't be checked, as the server would let in
/// people it was told not to.
pub(super) fn member_from(
    variables: &[(String, String)],
) -> Result<Option<MemberSettings>, String> {
    let value = |name: &str| {
        let found = variables.iter().find(|(key, _)| key == name);
        found.map(|(_, value)| value.clone())
    };
    let token = secret_value("HOW_DISCORD_BOT_TOKEN", value)?;
    let server = value("HOW_DISCORD_SERVER_ID")
        .map(|id| id.trim().to_owned())
        .filter(|id| !id.is_empty());
    let Some(server) = server else {
        if token.is_some() {
            bevy::log::warn!(
                "HOW_DISCORD_BOT_TOKEN is set but HOW_DISCORD_SERVER_ID isn't, so the bot isn't used"
            );
        }
        return Ok(None);
    };
    let server = server
        .parse()
        .map_err(|_| format!("HOW_DISCORD_SERVER_ID {server:?} isn't a Discord server's id"))?;
    let bot_token = token.ok_or(
        "HOW_DISCORD_SERVER_ID is set, but HOW_DISCORD_BOT_TOKEN (or \
         HOW_DISCORD_BOT_TOKEN_FILE) isn't, so membership can't be checked",
    )?;
    Ok(Some(MemberSettings { server, bot_token }))
}

/// What Discord said of one user and the server's Discord server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Membership {
    /// On it, with these Discord roles (by id).
    Member(Vec<u64>),
    /// Not on it (or no such user).
    NotMember,
    /// Discord couldn't be asked just now (unreachable, failing, or busy past
    /// waiting): players are let in as they are, and asked again later.
    Unknown(String),
    /// Discord refused the bot (a wrong token, or the bot not on that Discord
    /// server): nobody new can be let in until the host puts it right.
    Refused(String),
}

/// What one answer from Discord means.
#[derive(Debug, PartialEq)]
pub(super) enum Read {
    Done(Membership),
    /// Rate limited: ask again after this many milliseconds.
    Wait(u64),
}

/// Discord's error code for a server it doesn't know (or the bot isn't on).
const UNKNOWN_GUILD: u64 = 10004;

/// Reads Discord's answer to `GET /guilds/{server}/members/{user}`.
pub(super) fn read(status: u16, body: &Value) -> Read {
    let refused = |why: &str| {
        Read::Done(Membership::Refused(format!(
            "Discord refused the bot ({why}): check HOW_DISCORD_BOT_TOKEN, \
             HOW_DISCORD_SERVER_ID and that the bot is on that Discord server"
        )))
    };
    match status {
        200..=299 => {
            let roles = body["roles"].as_array().map(|roles| {
                let ids = roles.iter().filter_map(|role| role.as_str()?.parse().ok());
                ids.collect()
            });
            match roles {
                Some(roles) => Read::Done(Membership::Member(roles)),
                None => Read::Done(Membership::Unknown("Discord gave no roles".into())),
            }
        }
        404 if body["code"].as_u64() == Some(UNKNOWN_GUILD) => refused("unknown server"),
        404 => Read::Done(Membership::NotMember),
        401 | 403 => refused(&format!("{status}")),
        429 => {
            let seconds = body["retry_after"].as_f64().unwrap_or(1.0).max(0.0);
            Read::Wait((seconds * 1000.0).ceil() as u64)
        }
        _ => Read::Done(Membership::Unknown(format!("Discord answered {status}"))),
    }
}
