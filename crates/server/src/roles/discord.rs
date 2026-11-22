//! Roles from the server's Discord (`HOW_DISCORD_SERVER_ID`): an account that
//! holds no role given by hand takes the first role in `discord_roles` whose
//! Discord role it has, or none (Player), each time it is checked. A role
//! given by hand (or claimed) is never changed by Discord, and lets the
//! account in without being on the Discord server; Banned still keeps it out.

use shared::role::{BANNED, PLAYER, RoleId};

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
    /// Discord server: any but Banned.
    pub(crate) fn in_without_discord(&self, account: AccountId) -> bool {
        let key = account.0.to_string();
        let held = self.file.accounts.get(&key);
        held.is_some_and(|held| !held.discord && !held.role.is(BANNED))
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
