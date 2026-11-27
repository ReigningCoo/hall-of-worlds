//! A spell's popup: resting the pointer (or a finger) on a spell's picture
//! opens a `hover` popup (`dom`), as items' do, with its name, its key on the
//! action bar, and what it is step by step with numbers for the player now,
//! before the target resists (`shared::spell::tip_lines`). It goes as hover
//! popups do.

use bevy::prelude::*;
use shared::spell::{auto_name, tip_lines};

use super::subject::{Subject, pointed, spell_of};
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::dom::{
    AbilityCell, AbilitySlotShown, Dom, HeldObject, Hit, KeptOpen, Popup, Template, UiEvent,
    UiValue, document,
};
use crate::html;
use crate::spell_design::PreviewSpell;
use crate::widgets::UiId;

/// The form the popup's ids are in.
const FORM: &str = "spell_popup";
pub(super) const BOX: UiId = UiId::new(FORM, "box");
/// How long the pointer rests on a picture before its popup opens (real
/// time), as for items.
const REST: f32 = 0.15;
/// How far from the pointer the popup's corner goes.
pub(super) const OFFSET: Vec2 = Vec2::new(16.0, 16.0);
/// How far a step's outcomes and filters are indented, in pixels.
const INDENT: &str = "16";

pub(super) static SPELL_POPUP: Template = html!(
    "spell_popup",
    r#"
<form id="spell_popup">
  <popup id="box" hover>
    <p id="name" class="medium" margin-right="20">.</p>
    <p id="key" hidden>.</p>
    <div id="lines" class="tight"/>
    <template id="line"><p id="line" max-width="360"/></template>
  </popup>
</form>
"#
);

/// The popup open (about what, its document), and the picture the pointer is
/// resting on and since when.
#[derive(Resource, Default)]
pub(super) struct SpellPopup {
    open: Option<(Subject, Entity)>,
    /// Where the pointer was when it opened.
    at: Option<Vec2>,
    resting: Option<(Subject, f32)>,
}

impl SpellPopup {
    /// Where the pointer was when the open popup opened.
    pub(super) fn opened_at(&self) -> Option<Vec2> {
        self.open.as_ref().and(self.at)
    }

    fn close(&mut self, commands: &mut Commands) {
        if let Some((_, document)) = self.open.take() {
            commands.entity(document).try_despawn();
        }
    }
}

/// Despawns the popup when it says it closed.
pub(super) fn forget_closed(
    mut events: MessageReader<UiEvent>,
    mut popup: ResMut<SpellPopup>,
    mut commands: Commands,
) {
    for event in events.read() {
        if event.value == UiValue::Closed && event.id == BOX {
            popup.close(&mut commands);
        }
    }
}

/// Follows which spell's picture the pointer is on: keeps its popup open, or
/// opens one once it has rested there, in place of one not locked.
#[allow(clippy::too_many_arguments)]
pub(super) fn follow_pointer(
    hit: Res<Hit>,
    (cells, slots, ids): (Query<&AbilityCell>, Query<&AbilitySlotShown>, Query<&UiId>),
    held: Res<HeldObject>,
    real: Res<Time<Real>>,
    locks: Query<(&Popup, &UiId)>,
    (mut popup, mut kept): (ResMut<SpellPopup>, ResMut<KeptOpen>),
    (sheet, wording, preview): (Res<CharacterSheet>, Res<Wording>, Res<PreviewSpell>),
    mut dom: Dom,
    mut commands: Commands,
) {
    let subject = pointed(&hit, &cells, &slots, &ids).filter(|_| held.thing().is_none());
    let Some(subject) = subject else {
        popup.resting = None;
        return;
    };
    if popup
        .open
        .as_ref()
        .is_some_and(|(open, _)| *open == subject)
    {
        kept.0.push(BOX);
        popup.resting = None;
        return;
    }
    let now = real.elapsed_secs();
    let since = match &popup.resting {
        Some((was, since)) if *was == subject => *since,
        _ => {
            popup.resting = Some((subject.clone(), now));
            now
        }
    };
    if now - since < REST {
        return;
    }
    popup.resting = None;
    let locked = locks.iter().any(|(it, id)| *id == BOX && it.locked);
    if popup.open.is_some() && locked {
        return;
    }
    popup.close(&mut commands);
    let Some((spell, own)) = spell_of(&subject, &sheet.0.spellbook, &preview) else {
        return;
    };
    let document = commands.spawn(document(&SPELL_POPUP)).id();
    let pointer = hit.at.unwrap_or_default();
    popup.at = Some(pointer);
    let at = (pointer + OFFSET).round();
    dom.set("spell_popup/box", "left", &at.x.to_string());
    dom.set("spell_popup/box", "top", &at.y.to_string());
    let lang = &wording.0;
    let name = own.unwrap_or_else(|| auto_name(&spell, lang));
    dom.set_text("spell_popup/name", name);
    let key = match &subject {
        Subject::Book(_, key) => key.clone(),
        Subject::Preview => String::new(),
    };
    dom.set_flag("spell_popup/key", "hidden", key.is_empty());
    dom.set_text("spell_popup/key", format!("Key: {key}"));
    for (at, line) in tip_lines(&spell, Some(&sheet.0.skills), lang)
        .into_iter()
        .enumerate()
    {
        dom.copy_template("spell_popup/line", "spell_popup/lines", at);
        let id = format!("spell_popup/line.{at}");
        dom.set_text(&id, line.text);
        if line.indented {
            dom.set(&id, "margin-left", INDENT);
        }
    }
    kept.0.push(BOX);
    popup.open = Some((subject, document));
}
