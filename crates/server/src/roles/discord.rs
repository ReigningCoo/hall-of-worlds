//! Roles from the server's Discord (`HOW_DISCORD_SERVER_ID`): an account that
//! holds no role given by hand takes the first role in `discord_roles` whose
//! Discord role it has, or none (Player), each time it is checked. A role
//! given by hand (or claimed) is never changed by Discord, and lets the
//! account in without being on the Discord server; Banned, and Player given by
//! hand (a demotion), still need it. "Follow Discord" in the Accounts panel
//! (`hand_to_discord`) takes a role given by hand away, for Discord to set.

use shared::protocol::RoleSource;
use shared::role::{BANNED, PLAYER, Powers, RoleId};

use super::{Actor, Change, Roles};
use crate::accounts::AccountId;

impl Roles {
    /// Whether Discord decides this account's role and whether it may play:
    /// it holds no role given by hand.
    pub(crate) fn follows_discord(&self, account: AccountId) -> bool {
        let key = account.0.to_string();
        self.file.accounts.get(&key).is_none_or(|held| held.discord)
    }

    /// Whether a role given by hand lets this account in without being on the
    /// Discord server: any but Banned and Player.
    pub(crate) fn in_without_discord(&self, account: AccountId) -> bool {
        let key = account.0.to_string();
        let held = self.file.accounts.get(&key);
        let out = |role: &RoleId| role.is(BANNED) || role.is(PLAYER);
        held.is_some_and(|held| !held.discord && !out(&held.role))
    }

    /// Where an account's role came from.
    pub(crate) fn source(&self, account: AccountId) -> RoleSource {
        match self.file.accounts.get(&account.0.to_string()) {
            None => RoleSource::Unset,
            Some(held) if held.discord => RoleSource::Discord,
            Some(held) if held.by.is_some() => RoleSource::Hand,
            Some(_) => RoleSource::Server,
        }
    }

    /// Whether someone with `powers` may hand this account back to Discord:
    /// its role wasn't Discord's, and they may give it and every role
    /// Discord might (Player too).
    pub(crate) fn may_hand_to_discord(&self, account: AccountId, powers: &Powers) -> bool {
        let Some(held) = self.file.accounts.get(&account.0.to_string()) else {
            return false;
        };
        let player = RoleId::new(PLAYER);
        let discords = self.file.discord_roles.iter().map(|mapped| &mapped.role);
        let mut involved = std::iter::once(&held.role).chain(discords).chain([&player]);
        !held.discord && involved.all(|role| powers.grants.contains(role))
    }

    /// Takes away a role given by hand (or claimed), so the account follows
    /// Discord again: Player until its next check. Only by someone who
    /// `may_hand_to_discord`, never for their own account; written in the
    /// audit log. What is wrong, if anything.
    pub(crate) fn hand_to_discord(
        &mut self,
        by: AccountId,
        powers: &Powers,
        target: AccountId,
        reason: &str,
        now: u64,
    ) -> Result<(), String> {
        if by == target {
            return Err("nobody can change their own role".into());
        }
        if !self.may_hand_to_discord(target, powers) {
            return Err("that account's role isn't one you can hand back to Discord".into());
        }
        let key = target.0.to_string();
        let reason = match reason.trim() {
            "" => String::new(),
            reason => format!(": {reason}"),
        };
        let what = format!(
            "{} handed {key} back to Discord (was {}){reason}",
            by.0,
            self.holds(&key).0
        );
        self.change(&key, None, now, &what)?;
        self.changed.push(target);
        self.recheck.push(target);
        Ok(())
    }

    /// Accounts handed back to Discord since last asked, to check at once.
    pub(crate) fn take_recheck(&mut self) -> Vec<AccountId> {
        std::mem::take(&mut self.recheck)
    }

    /// Adds a line to the Discord mapping (tests; hosts edit `roles.ron`).
    #[cfg(test)]
    pub(crate) fn map_discord(&mut self, discord: u64, role: &str) {
        let role = RoleId::new(role);
        let line = super::file::DiscordRole { discord, role };
        self.file.discord_roles.push(line);
        self.file.check().expect("a mapping the roles allow");
    }

    /// The role Discord gives an account with these Discord roles: the first
    /// that matches, else Player.
    fn discord_gives(&self, discord_roles: &[u64]) -> RoleId {
        let mapping = self.file.discord_roles.iter();
        let mut matching = mapping.filter(|mapped| discord_roles.contains(&mapped.discord));
        matching
            .next()
            .map_or_else(|| RoleId::new(PLAYER), |m| m.role.clone())
    }

    /// Gives an account following Discord the role its Discord roles map to
    /// (`None`: not on the Discord server, so none), noting each change in the
    /// audit log as "Discord". Nothing for one with a role given by hand. An
    /// error if the roles can't be written.
    pub(crate) fn follow_discord(
        &mut self,
        account: AccountId,
        discord_roles: Option<&[u64]>,
        now: u64,
    ) -> Result<(), String> {
        if !self.follows_discord(account) {
            return Ok(());
        }
        let wanted = match discord_roles {
            Some(roles) => self.discord_gives(roles),
            None => RoleId::new(PLAYER),
        };
        if self.holds(&account.0.to_string()) == wanted {
            return Ok(());
        }
        let change = Change {
            to: &wanted,
            until: None,
            reason: "Discord",
        };
        self.set_role(Actor::Discord, account, false, change, now)
    }
}
