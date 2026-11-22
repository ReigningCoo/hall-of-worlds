//! Setting logins up from the environment: the OIDC providers (`config`),
//! Discord (`discord`), where players come back to, and the login secret.

use std::env;
use std::path::Path;

use bevy::prelude::*;

use super::config::{self, ProviderSettings};
use super::discord::{self, DiscordSettings};
use super::{LoginSecret, Logins};

/// The ways in the environment sets up, before they are started.
#[derive(Debug, Default)]
pub(crate) struct Setup {
    pub oidc: Vec<ProviderSettings>,
    pub discord: Option<DiscordSettings>,
    /// Only Discord, and no guests (`HOW_DISCORD_ONLY`).
    pub discord_only: bool,
}

impl Setup {
    /// From `variables` (the environment's), logging what is left out. An
    /// error if only Discord is wanted but it isn't set up: the server would
    /// otherwise let in people it was told not to.
    pub(super) fn from(variables: &[(String, String)]) -> Result<Self, String> {
        let (mut oidc, mut warnings) = config::providers_from(variables);
        let (discord, discord_only, more) = discord::discord_from(variables);
        warnings.extend(more);
        for warning in warnings {
            warn!("{warning}");
        }
        if discord_only {
            if discord.is_none() {
                return Err(
                    "HOW_DISCORD_ONLY is true, but Discord logins aren't set up \
                     (HOW_DISCORD_CLIENT_ID and HOW_DISCORD_CLIENT_SECRET)"
                        .into(),
                );
            }
            if !oidc.is_empty() {
                warn!("HOW_DISCORD_ONLY is true: the other providers (HOW_OIDC_...) are off");
                oidc.clear();
            }
        }
        Ok(Self {
            oidc,
            discord,
            discord_only,
        })
    }

    fn is_empty(&self) -> bool {
        self.oidc.is_empty() && self.discord.is_none()
    }
}

impl Logins {
    /// The providers the environment sets up, with the login secret kept in
    /// `data`. `None`, with warnings logged, if none can be used. Waits for each
    /// OIDC provider's discovery document, so call it once logging has started.
    /// An error if the secret can't be had (starting without it would lose
    /// every account), or if only Discord is wanted and it can't be used.
    pub fn from_environment(data: &Path) -> Result<Option<Self>, String> {
        let variables: Vec<(String, String)> = env::vars().collect();
        let setup = Setup::from(&variables)?;
        if setup.is_empty() {
            return Ok(None);
        }
        let discord_only = setup.discord_only;
        let callback = match config::callback_from(
            env::var("HOW_LOGIN_URL").ok().as_deref(),
            env::var("HOW_LETS_ENCRYPT_DOMAIN").ok().as_deref(),
        ) {
            Ok(callback) => callback,
            Err(problem) if discord_only => return Err(format!("{problem} (HOW_DISCORD_ONLY)")),
            Err(problem) => {
                warn!("{problem}: logins through providers are off");
                return Ok(None);
            }
        };
        let given = crate::environment::secret("HOW_LOGIN_SECRET")?;
        let secret = LoginSecret::load(data, given.as_deref())?;
        let logins = Self::start(setup, callback, secret);
        if discord_only && logins.is_none() {
            return Err("HOW_DISCORD_ONLY is true, but Discord logins can't start".into());
        }
        Ok(logins)
    }
}
