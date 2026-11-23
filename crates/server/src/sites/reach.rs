//! How far a visit can see, and whether its site is a town.

use super::{Place, TOWN_SIGHT, Visit};

impl Visit {
    /// Whether the site is a town, whose layout players always know.
    pub(super) fn is_town(&self) -> bool {
        self.site().is_some_and(|site| site.kind().is_town())
    }

    /// How far can be seen, in cells: open towns and the wilderness have a
    /// limit, or sight would be worked out across all of them.
    pub(super) fn reach(&self) -> Option<u32> {
        let reach = match &self.place {
            Place::Wilderness { .. } => Some(crate::wilderness::SIGHT),
            Place::Site { .. } => self.is_town().then_some(TOWN_SIGHT),
        };
        reach.map(|reach| reach + self.further)
    }
}
