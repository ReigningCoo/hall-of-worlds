//! Forgetting what a character knew of sites, for a site that changed or a
//! map forgotten.

use super::{CharacterWorldSave, SiteId};

impl CharacterWorldSave {
    /// Forgets everything they knew of a site: what they saw, the bags and
    /// doors there, its version and where they walked out.
    pub fn forget_site(&mut self, id: &SiteId) {
        self.memory.remove(id);
        self.bags.remove(id);
        self.doors.remove(id);
        self.site_versions.remove(id);
        self.left_at.remove(id);
    }

    /// Forgets every site, as `forget_site` does.
    pub fn forget_sites(&mut self) {
        self.memory.clear();
        self.bags.clear();
        self.doors.clear();
        self.site_versions.clear();
        self.left_at.clear();
    }
}
