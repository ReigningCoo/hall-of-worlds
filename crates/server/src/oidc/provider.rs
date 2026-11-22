//! A provider ready to use: where its pages are (from its discovery document,
//! or Discord's own), the address players are sent to, and what it vouches for:
//! an email address, or a Discord user.

use serde_json::Value;
use url::Url;

use super::Proved;
use super::config::{ProviderSettings, secure_enough};
use super::discord::{DiscordSettings, discord_user};
use super::http::Http;

/// What an OIDC login asks the provider for: only who they are (`openid`) and
/// their email address.
const SCOPES: &str = "openid email";

/// What a Discord login asks for: only who they are, not their address.
const DISCORD_SCOPES: &str = "identify";

/// A provider whose pages are known.
pub(super) struct Provider {
    /// What `LoginMethod` calls it.
    pub name: String,
    pub label: String,
    pub kind: Kind,
    client_id: String,
    client_secret: String,
    authorization: String,
    token: String,
    /// Where to ask who the token is for.
    userinfo: String,
}

/// What a provider is, and so what it vouches for.
pub(super) enum Kind {
    /// OIDC: a verified email address.
    Oidc,
    /// Discord (OAuth2): a user id, with the server's invitation if it has one.
    Discord { invite: Option<String> },
}

impl Provider {
    /// Reads the provider's discovery document, for where its pages are.
    pub(super) async fn discover(http: &Http, settings: ProviderSettings) -> Result<Self, String> {
        let issuer = settings.issuer.trim_end_matches('/');
        let found = http
            .get_json(&format!("{issuer}/.well-known/openid-configuration"), None)
            .await?;
        if found["issuer"]
            .as_str()
            .map(|said| said.trim_end_matches('/'))
            != Some(issuer)
        {
            return Err(format!(
                "its discovery document is for another issuer than {issuer}"
            ));
        }
        let page = |name: &str| {
            let page = found[name]
                .as_str()
                .ok_or_else(|| format!("its discovery document has no {name}"))?;
            secure_enough(page)?;
            Ok::<String, String>(page.to_owned())
        };
        Ok(Self {
            authorization: page("authorization_endpoint")?,
            token: page("token_endpoint")?,
            userinfo: page("userinfo_endpoint")?,
            name: settings.name,
            label: settings.label,
            kind: Kind::Oidc,
            client_id: settings.client_id,
            client_secret: settings.client_secret,
        })
    }

    /// Discord, whose pages are known without asking.
    pub(super) fn discord(settings: DiscordSettings) -> Self {
        let site = settings.site.trim_end_matches('/');
        Self {
            name: shared::protocol::DISCORD.into(),
            label: "Discord".into(),
            kind: Kind::Discord {
                invite: settings.invite,
            },
            client_id: settings.client_id,
            client_secret: settings.client_secret,
            authorization: format!("{site}/oauth2/authorize"),
            token: format!("{site}/api/oauth2/token"),
            userinfo: format!("{site}/api/users/@me"),
        }
    }

    /// Where to send the player to log in: back to `callback` afterwards with
    /// `state`, proving it is this login with the PKCE `challenge`.
    pub(super) fn login_page(&self, callback: &str, state: &str, challenge: &str) -> String {
        let mut url = Url::parse(&self.authorization).expect("checked, or Discord's");
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", callback)
            .append_pair("scope", self.scopes())
            .append_pair("state", state)
            .append_pair("code_challenge", challenge)
            .append_pair("code_challenge_method", "S256");
        url.into()
    }

    fn scopes(&self) -> &'static str {
        match self.kind {
            Kind::Oidc => SCOPES,
            Kind::Discord { .. } => DISCORD_SCOPES,
        }
    }

    /// Swaps the code the player came back with for a token, then asks who it
    /// is for: the email address, if the provider has verified it, or the
    /// Discord user. Nothing else is asked for or kept, and the token is
    /// forgotten straight away.
    pub(super) async fn prove(
        &self,
        http: &Http,
        callback: &str,
        code: &str,
        verifier: &str,
    ) -> Result<Proved, String> {
        let tokens = http
            .post_form(
                &self.token,
                &[
                    ("grant_type", "authorization_code"),
                    ("code", code),
                    ("redirect_uri", callback),
                    ("client_id", &self.client_id),
                    ("client_secret", &self.client_secret),
                    ("code_verifier", verifier),
                ],
            )
            .await?;
        let token = tokens["access_token"]
            .as_str()
            .ok_or("the provider gave no access token")?;
        let info = http.get_json(&self.userinfo, Some(token)).await?;
        match self.kind {
            Kind::Oidc => verified_email(&info).map(Proved::Email),
            Kind::Discord { .. } => discord_user(&info).map(Proved::Discord),
        }
    }
}

/// The email address in a provider's user info, if it says it has verified it
/// (some say so as text). Anyone could claim an address that isn't verified.
pub(super) fn verified_email(info: &Value) -> Result<String, String> {
    let email = info["email"]
        .as_str()
        .filter(|email| email.contains('@'))
        .ok_or("the provider gave no email address")?;
    let verified = match &info["email_verified"] {
        Value::Bool(verified) => *verified,
        Value::String(verified) => verified == "true",
        _ => false,
    };
    if !verified {
        return Err("the provider hasn't verified this email address".into());
    }
    Ok(email.to_owned())
}
