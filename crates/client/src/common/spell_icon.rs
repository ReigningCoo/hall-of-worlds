//! A spell's icon wherever it is drawn (the action bar, the Spells tab, the
//! spell design panel's preview): Heal's for one that heals first, else
//! Magic's, so a design looks as it will once saved.

use shared::spell::{Effect, Spell};

/// The Magic ability's icon, which spells share.
pub const MAGIC_ICON: &str = "icons/lorc/magic-swirl.png";

/// Heal's icon, for a spell whose first effect heals.
const HEAL_ICON: &str = "icons/delapouite/healing.png";

/// The icon this spell is drawn with.
pub fn spell_icon(spell: &Spell) -> &'static str {
    let heals = (spell.effect_steps().next())
        .and_then(|step| step.effects.first())
        .is_some_and(|(effect, _)| *effect == Effect::Heal);
    if heals { HEAL_ICON } else { MAGIC_ICON }
}
