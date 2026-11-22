//! Logging in through providers' websites (OIDC), such as Google, and Discord
//! (OAuth2, `discord.rs`). See "Logging in through a provider" in
//! `docs/context/server/server.md`.
//!
//! A client asks to log in with a provider (`Credentials::Provider`) and is sent
//! the provider's page to open in the player's browser. The provider sends the
//! browser back to this server (`CALLBACK_PATH`, on the network thread), which
//! swaps the code for the email address the provider has verified (or the
//! Discord user id) and hands it to the game (`finished`). Only a key made from
//! the address or id with the login secret is kept (`LoginSecret`), and the
//! account found by it.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::RawQuery;
use axum::routing::get;
use bevy::prelude::*;
use shared::protocol::LoginMethod;

use crate::accounts::{DiscordKey, EmailKey};

// Split into files (`cargo xtask lint` keeps files small).
mod callback;
mod config;
mod discord;
mod http;
mod pending;
mod provider;
mod secret;
mod setup;

pub(crate) use discord::DiscordUser;
pub(crate) use secret::LoginSecret;
pub(crate) use setup::Setup;

use http::Http;
use pending::{Pending, Waiting};
use provider::{Kind, Provider};

/// Where providers send players back to.
pub const CALLBACK_PATH: &str = "/login/callback";

/// A login back from a provider, for the game to finish.
pub(crate) struct Finished {
    /// The state it was begun with (`Logins::begin`).
    pub state: String,
    /// Which provider.
    pub provider: String,
    /// Who the provider vouched for, or why it didn't.
    pub proved: Result<Proved, String>,
}

/// Who a provider vouched for.
pub(crate) enum Proved {
    /// An OIDC provider: the email address (never kept or logged).
    Email(String),
    /// Discord: the user.
    Discord(DiscordUser),
}

/// By hand, so an address never reaches a log.
impl std::fmt::Debug for Proved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email(_) => f.write_str("Proved::Email(hidden)"),
            Self::Discord(user) => write!(f, "Proved::Discord({user:?})"),
        }
    }
}

/// The providers this server logs players in with, and the logins under way.
/// Shared by the game and the network thread.
#[derive(Resource, Clone)]
pub struct Logins(Arc<Inner>);

struct Inner {
    providers: Vec<Provider>,
    /// Where providers send players back to (`CALLBACK_PATH` on the server's
    /// public address).
    callback: String,
    secret: LoginSecret,
    /// Whether Discord is the only way in (`HOW_DISCORD_ONLY`).
    discord_only: bool,
    http: Http,
    pending: Mutex<Pending>,
    finished: (Mutex<Sender<Finished>>, Mutex<Receiver<Finished>>),
}

impl Logins {
    /// Reads each OIDC provider's discovery document, leaving out (with a
    /// warning) those that can't be read, and adds Discord: `None` if that
    /// leaves none. `from_environment` (in `setup.rs`) reads the settings.
    pub(crate) fn start(setup: Setup, callback: String, secret: LoginSecret) -> Option<Self> {
        let http = Http::new()
            .inspect_err(|error| warn!("{error}: logins through providers are off"))
            .ok()?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a tokio runtime");
        let mut providers: Vec<Provider> = runtime.block_on(async {
            let mut ready = Vec::new();
            for settings in setup.oidc {
                let name = settings.name.clone();
                match Provider::discover(&http, settings).await {
                    Ok(provider) => ready.push(provider),
                    Err(error) => warn!("Can't use {name} for logging in: {error}"),
                }
            }
            ready
        });
        providers.extend(setup.discord.map(Provider::discord));
        if providers.is_empty() {
            return None;
        }
        let names: Vec<&str> = providers.iter().map(|p| p.name.as_str()).collect();
        info!(
            "Logging in through {}, back at {callback}",
            names.join(", ")
        );
        let (sender, receiver) = mpsc::channel();
        Some(Self(Arc::new(Inner {
            providers,
            callback,
            secret,
            discord_only: setup.discord_only,
            http,
            pending: Mutex::default(),
            finished: (Mutex::new(sender), Mutex::new(receiver)),
        })))
    }

    /// Whether Discord is the only way in (`HOW_DISCORD_ONLY`): no other
    /// provider and no guests, and sessions only for Discord's accounts.
    pub fn discord_only(&self) -> bool {
        self.0.discord_only
    }

    /// The ways to log in this offers, for `LoginOptions`.
    pub(crate) fn methods(&self) -> Vec<LoginMethod> {
        let providers = self.0.providers.iter();
        providers
            .map(|provider| match &provider.kind {
                Kind::Oidc => LoginMethod::Provider {
                    name: provider.name.clone(),
                    label: provider.label.clone(),
                },
                Kind::Discord { invite } => LoginMethod::Discord {
                    invite: invite.clone(),
                },
            })
            .collect()
    }

    /// Starts logging in with the provider of this name: its state, to find the
    /// login again when it is finished, and the page to send the player to.
    /// An error says why it can't.
    pub(crate) fn begin(&self, name: &str) -> Result<(String, String), String> {
        let found = self.0.providers.iter().position(|p| p.name == name);
        let index = found.ok_or_else(|| format!("this server doesn't log in with {name:?}"))?;
        let mut pending = self.0.pending.lock().expect("never poisoned");
        let (state, challenge) = pending.begin(index)?;
        let page = self.0.providers[index].login_page(&self.0.callback, &state, &challenge);
        Ok((state, page))
    }

    /// Forgets a login its player gave up on, so it can't be finished.
    pub(crate) fn forget(&self, state: &str) {
        self.0.pending.lock().expect("never poisoned").forget(state);
    }

    fn take(&self, state: &str) -> Option<Waiting> {
        self.0.pending.lock().expect("never poisoned").take(state)
    }

    fn finish(&self, finished: Finished) {
        let sender = self.0.finished.0.lock().expect("never poisoned");
        // The receiver lives as long as this.
        let _ = sender.send(finished);
    }

    /// The logins back from providers since last asked.
    pub(crate) fn finished(&self) -> Vec<Finished> {
        let receiver = self.0.finished.1.lock().expect("never poisoned");
        receiver.try_iter().collect()
    }

    /// The key an email address is known by (`LoginSecret::email_key`).
    pub(crate) fn email_key(&self, email: &str) -> EmailKey {
        self.0.secret.email_key(email)
    }

    /// The key a Discord user id is known by (`LoginSecret::discord_key`).
    pub(crate) fn discord_key(&self, user: u64) -> DiscordKey {
        self.0.secret.discord_key(user)
    }

    /// The page providers send players back to.
    pub(crate) fn routes(&self) -> Router {
        let logins = self.clone();
        Router::new().route(
            CALLBACK_PATH,
            get(move |query: RawQuery| callback::came_back(logins.clone(), query)),
        )
    }
}

#[cfg(test)]
mod discord_tests;
#[cfg(test)]
pub(crate) mod tests;
