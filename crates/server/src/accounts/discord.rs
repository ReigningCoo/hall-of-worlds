//! The Discord user an account belongs to: kept on the account
//! (`AccountSave::discord`) when it logs in through Discord, for linking
//! another way in to it and for checking who is on the server's Discord.
//! Never logged: logs name accounts by id.

use super::{AccountId, Accounts};

impl Accounts {
    /// Notes that this account is this Discord user's, saving it if that is
    /// news. An error if saves can't be reached.
    pub(crate) fn note_discord(&mut self, id: &AccountId, user: u64) -> Result<(), String> {
        if self.account(id)?.discord == Some(user) {
            return Ok(());
        }
        if let Some(account) = self.loaded.get_mut(id) {
            account.discord = Some(user);
        }
        self.save(id);
        Ok(())
    }

    /// The Discord user this account belongs to, if any. An error if saves
    /// can't be reached.
    pub(crate) fn discord_of(&mut self, id: &AccountId) -> Result<Option<u64>, String> {
        Ok(self.account(id)?.discord)
    }
}
