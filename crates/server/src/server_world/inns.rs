//! The Way Inn's instances the home world keeps (`sites/inn`): opened as
//! needed, never saved, closed once nobody is in them.

use super::ServerWorld;
use crate::storage::SiteId;

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
}
