//! Cast: the design on the panel cast as it is, saved or not (owner,
//! 2026-11-27). Greyed by exactly the rule a spell on the bar is
//! (`common::can_cast_spell`: allowed by the skills, the Mana for it, in The
//! Way Inn only on the caster), never for being in the spellbook already.
//! Pressed, it goes to `abilities` as `CastDesign`, which aims and queues it as
//! a spell slot pressed.

use bevy::prelude::*;
use shared::sheet::Sheet;

use super::CastDesign;
use super::draft::Draft;
use crate::character::CharacterSheet;
use crate::common::{CurrentDungeon, GameView, OwnVitals, can_cast_spell, in_inn};
use crate::dom::{Dom, UiEvent, UiValue};

/// What casting needs to know besides the design.
type Now<'w> = (
    Res<'w, CharacterSheet>,
    Res<'w, OwnVitals>,
    Res<'w, Time>,
    Res<'w, State<GameView>>,
    Res<'w, CurrentDungeon>,
);

/// Whether the design on the panel can be cast now.
fn castable(draft: &Draft, sheet: &Sheet, (vitals, now): (&OwnVitals, f64), inn: bool) -> bool {
    !draft.filters_stranded() && can_cast_spell(&draft.spell(), sheet, (vitals, now), inn)
}

/// Cast pressed: the design sent to `abilities` if it can be cast.
pub(super) fn answer_cast(
    mut events: MessageReader<UiEvent>,
    draft: Res<Draft>,
    (sheet, vitals, time, view, dungeon): Now,
    mut cast: MessageWriter<CastDesign>,
) {
    let now = time.elapsed_secs_f64();
    let inn = in_inn(*view.get(), &dungeon);
    for event in events.read() {
        let ("spell_design", "cast") = event.id.parts() else {
            continue;
        };
        if matches!(event.value, UiValue::Pressed)
            && castable(&draft, &sheet.0, (&vitals, now), inn)
        {
            cast.write(CastDesign(draft.spell()));
        }
    }
}

/// Greys Cast whenever it can't be cast, checked every frame as the Mana
/// refills; the button only changed when that does.
pub(super) fn show_cast(
    draft: Res<Draft>,
    (sheet, vitals, time, view, dungeon): Now,
    mut dom: Dom,
    mut shown: Local<Option<(Entity, bool)>>,
) {
    let Some(document) = draft.document else {
        *shown = None;
        return;
    };
    let now = time.elapsed_secs_f64();
    let inn = in_inn(*view.get(), &dungeon);
    let can = castable(&draft, &sheet.0, (&vitals, now), inn);
    if *shown != Some((document, can)) {
        dom.set_flag("spell_design/cast", "disabled", !can);
        *shown = Some((document, can));
    }
}
