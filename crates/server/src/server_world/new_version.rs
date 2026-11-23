//! The generator version a site is first made with.

use shared::site::Generator;

use super::ServerWorld;
use crate::storage::SiteId;

impl ServerWorld {
    /// The generator version a site is made with when first entered: its
    /// generator's current one.
    pub(super) fn new_version(&self, id: SiteId) -> u32 {
        let kind = match id {
            SiteId::Poi(chunk) => self.pois.in_chunk(chunk).map(|poi| poi.kind),
            SiteId::Dev { kind, .. } => Some(kind),
            SiteId::Quest { .. } => Some(shared::quest::DUNGEON_KIND),
            SiteId::Wilderness => return shared::wilderness::WILDERNESS_VERSION,
            SiteId::DevArea(_) => return shared::site::DEV_AREA_VERSION,
            SiteId::Inn | SiteId::InnInstance(_) => return shared::site::INN_VERSION,
        };
        kind.map_or(0, |kind| Generator::for_poi(kind).version())
    }
}
