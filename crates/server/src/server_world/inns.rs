//! The Way Inn's instances the home world keeps (`sites/inn`): opened as
//! needed, never saved, closed once nobody is in them.

use super::ServerWorld;
use crate::storage::{LevelSave, SiteId, SiteSave};

impl ServerWorld {
    /// The numbers of the inn's instances open here.
    pub(crate) fn open_inns(&self) -> Vec<u32> {
        (self.sites.keys())
            .filter_map(|id| match id {
                SiteId::InnInstance(number) => Some(*number),
                _ => None,
            })
            .collect()
    }

    /// Closes an instance of the inn: everything about it goes, what was left
    /// lying in it too.
    pub(crate) fn close_inn(&mut self, number: u32) {
        let id = SiteId::InnInstance(number);
        self.sites.remove(&id);
        self.unreachable_sites.remove(&id);
        self.unsaved_sites.unmark(&id);
        self.populations.forget_site(id);
        self.litter.forget(id);
    }

    /// Keeps a character's own rooms here while they are in the inn
    /// (`sites/inn/own_room.rs`): upstairs' things as saved with them.
    pub(crate) fn put_own_room(&mut self, id: SiteId, upstairs: LevelSave) {
        let mut save = SiteSave {
            version: self.new_version(id),
            ..Default::default()
        };
        save.levels.insert(shared::inn::UPSTAIRS, upstairs);
        self.unreachable_sites.remove(&id);
        self.sites.insert(id, save);
    }

    /// A character's own rooms' things kept here, if they are.
    pub(crate) fn own_room(&self, id: SiteId) -> Option<&LevelSave> {
        let site = self.sites.get(&id)?;
        site.levels.get(&shared::inn::UPSTAIRS)
    }

    /// Lets go of a character's own rooms (once kept with them).
    pub(crate) fn drop_own_room(&mut self, id: SiteId) {
        self.sites.remove(&id);
        self.unreachable_sites.remove(&id);
        self.unsaved_sites.unmark(&id);
    }
}
