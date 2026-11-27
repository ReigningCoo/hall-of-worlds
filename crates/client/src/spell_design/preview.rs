//! The picture the design would have once saved, beside its name and cost
//! (owner, 2026-11-27): the same look as a spell in the book (Magic's, or
//! Heal's icon for one healing first; its kept or automatic name), greyed
//! while it can't be saved. The panel's `<object type="ability">` shows it by
//! its name in `KnownAbilities`; its id, `spell_design/preview`, is where the
//! spell popup (`spell_popup`) finds it, from `PreviewSpell`, in place of a
//! plain tip.

use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::lang::Lang;
use shared::spell::{Spell, auto_name};

use super::PreviewSpell;
use super::draft::Draft;
use super::side::can_store;
use crate::character::CharacterSheet;
use crate::common::{Wording, spell_icon};
use crate::dom::{AbilityThing, Dom, KnownAbilities};
use crate::widgets::{AbilityLook, Shade, attribute_colour};

/// The preview's name for `<object type="ability" for="…">`.
pub(super) const PREVIEW: &str = "spell_design.preview";

/// How a design looks: as `abilities` draws a spell in the book.
pub(super) fn design_look(
    spell: &Spell,
    name: Option<&str>,
    lang: &Lang,
    usable: bool,
    assets: &AssetServer,
) -> AbilityLook {
    AbilityLook {
        icon: assets.load(spell_icon(spell)),
        name: name.map_or_else(|| auto_name(spell, lang), str::to_owned),
        passive: false,
        usable,
        colour: attribute_colour(Ability::Magic.leans(), Shade::Active),
        item: None,
    }
}

/// Says how the preview looks, again when the design, the skills, the book
/// or the words change; hidden while there is nothing to show.
pub(super) fn show_preview(
    draft: Res<Draft>,
    (sheet, wording, assets): (Res<CharacterSheet>, Res<Wording>, Res<AssetServer>),
    mut known: ResMut<KnownAbilities>,
    mut preview: ResMut<PreviewSpell>,
    mut dom: Dom,
    mut shown: Local<Option<Entity>>,
) {
    let Some(document) = draft.document else {
        *shown = None;
        if preview.0.is_some() {
            preview.0 = None;
        }
        return;
    };
    let fresh = *shown != Some(document);
    if !(fresh || draft.is_changed() || sheet.is_changed() || wording.is_changed()) {
        return;
    }
    let spell = draft.spell();
    let usable = can_store(&draft, &sheet.0).is_ok();
    let mut look = design_look(&spell, draft.name.as_deref(), &wording.0, usable, &assets);
    // No plain tip: the spell popup says what it is.
    look.name = String::new();
    let thing = AbilityThing {
        thing: BarThing::Ability(Ability::Magic),
        look,
    };
    known.set(PREVIEW, thing);
    dom.set_flag("spell_design/preview", "hidden", spell.steps.is_empty());
    let now = (!spell.steps.is_empty()).then(|| (spell, draft.name.clone()));
    if preview.0 != now {
        preview.0 = now;
    }
    *shown = Some(document);
}
