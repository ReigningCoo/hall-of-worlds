//! How each ability is being used, worked out once for the action bar and the
//! Abilities page alike: used by itself (automatic on the bar), on (a toggle
//! such as Sneak, or a panel's ability while its panel is open), queued to be
//! used by hand.

use shared::ability::{Ability, ActionBar};

use bevy::prelude::*;

use super::ferment::Fermenting;
use super::manual::Manual;
use super::press_juice::JuiceChooser;
use super::press_oil::PressChooser;
use crate::common::{OpenPanels, Sneaking};
use crate::dom::AbilityMarks;

/// The abilities in each state now, compared to know when to draw again.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Marks {
    auto: Vec<Ability>,
    on: Vec<Ability>,
    queued: Vec<Ability>,
}

impl Marks {
    /// The states now, from the bar, what was chosen by hand, Sneak and the
    /// panels open.
    pub(super) fn now(
        bar: &ActionBar,
        manual: &Manual,
        sneaking: Sneaking,
        panels: &OpenPanels,
    ) -> Self {
        let auto = (bar.slots.iter().flatten())
            .filter(|held| held.auto)
            .filter_map(|held| held.ability())
            .collect();
        let on = Ability::ALL
            .into_iter()
            .filter(|&ability| is_on(ability, sneaking, panels))
            .collect();
        Self {
            auto,
            on,
            queued: manual.queued.iter().copied().collect(),
        }
    }

    /// How this ability is being used.
    pub(super) fn of(&self, ability: Ability) -> AbilityMarks {
        AbilityMarks {
            auto: self.auto.contains(&ability),
            on: self.on.contains(&ability),
            queued: self.queued.contains(&ability),
        }
    }
}

/// Whether an ability shows as on: a toggle that is (Sneak, the only one so
/// far), or one whose panel is open.
fn is_on(ability: Ability, sneaking: Sneaking, panels: &OpenPanels) -> bool {
    (ability.is_toggle() && sneaking.0) || panels.is_open(ability)
}

/// Says whether this module's own panels (Press Oil's, Press Juice's,
/// Ferment's) are open, as `alchemy` and `cooking` say of theirs.
pub(super) fn tell_own_panels(
    (press, juice, fermenting): (Res<PressChooser>, Res<JuiceChooser>, Res<Fermenting>),
    mut panels: ResMut<OpenPanels>,
) {
    OpenPanels::tell(&mut panels, Ability::PressOil, press.open);
    OpenPanels::tell(&mut panels, Ability::PressJuice, juice.open);
    OpenPanels::tell(&mut panels, Ability::Ferment, fermenting.open);
}

#[cfg(test)]
mod tests;
