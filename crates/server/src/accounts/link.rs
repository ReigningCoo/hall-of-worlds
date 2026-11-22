//! Ways in to an account: finding the account a login leads to without making
//! one, and adding another way in to an account that has one already (linking
//! Discord, `connections/link_discord.rs`).

use super::{AccountId, Accounts, LoginKey, LoginSave};
use crate::saves::{Save, SaveKey};

impl Accounts {
    /// The account this login leads to, if it has been made. An error if saves
    /// can't be reached.
    pub(crate) fn login_of(&mut self, key: &LoginKey) -> Result<Option<AccountId>, String> {
        let hash = key.hash();
        if let Some(id) = self.logins.get(&hash) {
            return Ok(Some(*id));
        }
        let saved = match &self.queue {
            Some(queue) => queue.load(&SaveKey::Login(hash.clone()))?,
            None => None,
        };
        let Some(Save::Login(login)) = saved else {
            return Ok(None);
        };
        self.logins.insert(hash, login.account);
        Ok(Some(login.account))
    }

    /// Makes this login lead to this account, saving it. Only for a login
    /// `login_of` found none for: one already leading somewhere is moved.
    pub(crate) fn add_login(&mut self, key: &LoginKey, id: AccountId) {
        let hash = key.hash();
        if let Some(queue) = &self.queue {
            let save = LoginSave {
                account: id,
                expires: None,
            };
            queue.save(SaveKey::Login(hash.clone()), Save::Login(save));
        }
        self.logins.insert(hash, id);
    }
}
