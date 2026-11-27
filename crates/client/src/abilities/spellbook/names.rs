//! Each spell in the book by name, for an `<object type="ability">` (the
//! Spells tab's pictures): how it looks now, greyed when it can't be cast.

use bevy::prelude::*;
use shared::ability::BarThing;
use shared::spell::SpellKey;

use super::super::peace::Peace;
use super::super::spells::{can_cast, spell_look};
use crate::character::CharacterSheet;
use crate::common::{OwnVitals, Wording};
use crate::dom::{AbilityThing, KnownAbilities};

/// A spell's name for `<object type="ability" for="…">`.
pub(in crate::abilities) fn spell_id(key: SpellKey) -> String {
    format!("spell.{}", key.0)
}

/// Says how each spell in the book looks, again when its name, whether it
/// can be cast or the book changes.
pub(in crate::abilities) fn name_spells(
    (sheet, wording, vitals, peace): (
        Res<CharacterSheet>,
        Res<Wording>,
        Res<OwnVitals>,
        Res<Peace>,
    ),
    (time, assets): (Res<Time>, Res<AssetServer>),
    mut known: ResMut<KnownAbilities>,
    mut named_for: Local<Vec<(SpellKey, bool, Option<String>)>>,
) {
    let now = time.elapsed_secs_f64();
    let standing: Vec<(SpellKey, bool, Option<String>)> = (sheet.0.spellbook.iter())
        .map(|kept| {
            let key = SpellKey::of(&kept.spell);
            let usable = can_cast(key, &sheet.0, (&vitals, now), peace.0);
            (key, usable, kept.name.clone())
        })
        .collect();
    if *named_for == standing && !wording.is_changed() {
        return;
    }
    for &(key, usable, _) in &standing {
        let mut look = spell_look(key, &sheet.0, &wording.0, usable, &assets);
        // No plain tip: the spell popup (`spell_popup`) says what it is.
        look.name = String::new();
        let thing = AbilityThing {
            thing: BarThing::Spell(key),
            look,
        };
        known.set(spell_id(key), thing);
    }
    *named_for = standing;
}
