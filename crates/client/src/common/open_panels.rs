//! The abilities whose panels are open now: shown as on, on the action bar
//! and the Abilities page, as a toggle is.

use bevy::prelude::*;
use shared::ability::Ability;

/// Which abilities' panels are open (Alchemy's, Cook's, Press Oil's, ...):
/// each panel's owner says whether its own is, `abilities` shows it.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct OpenPanels(Vec<Ability>);

impl OpenPanels {
    /// Whether this ability's panel is open.
    pub fn is_open(&self, ability: Ability) -> bool {
        self.0.contains(&ability)
    }

    /// Says whether this ability's panel is open, changing only when it
    /// differs.
    pub fn tell(panels: &mut ResMut<Self>, ability: Ability, open: bool) {
        if panels.is_open(ability) == open {
            return;
        }
        if open {
            panels.0.push(ability);
        } else {
            panels.0.retain(|&one| one != ability);
        }
    }
}
