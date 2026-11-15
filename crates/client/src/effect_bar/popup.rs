//! An effect's popup: resting the pointer (or a finger) on its icon opens a
//! `hover` popup (`dom`), as items' do, with its name, a line per bonus, what
//! ends it early and how long it has left, counting down. It goes with the
//! effect, or as hover popups do.

use bevy::prelude::*;
use shared::bonus::{Effect, Source};

use super::bar::EffectIcon;
use super::clock::ServerTick;
use super::words::{bonus_lines, ends_early, ends_in, name};
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::dom::{Dom, HeldObject, Hit, KeptOpen, Popup, Template, UiEvent, UiValue, document};
use crate::html;
use crate::widgets::UiId;

/// The form the popup's ids are in.
const FORM: &str = "effect_popup";
const BOX: UiId = UiId::new(FORM, "box");
/// How long the pointer rests on an icon before its popup opens (real time),
/// as for items.
const REST: f32 = 0.15;
/// How far from the pointer the popup's corner goes.
const OFFSET: Vec2 = Vec2::new(16.0, 16.0);

pub(super) static EFFECT_POPUP: Template = html!(
    "effect_popup",
    r#"
<form id="effect_popup">
  <popup id="box" hover>
    <p id="name" class="medium" margin-right="20">.</p>
    <p id="bonuses" hidden>.</p>
    <p id="ends" hidden>.</p>
    <p id="left">.</p>
  </popup>
</form>
"#
);

/// The popup open (about which effect, its document, what it says of the
/// time left), and the icon the pointer is resting on and since when.
#[derive(Resource, Default)]
pub(super) struct EffectPopup {
    open: Option<(Source, Entity, String)>,
    resting: Option<(Source, f32)>,
}

impl EffectPopup {
    fn close(&mut self, commands: &mut Commands) {
        if let Some((_, document, _)) = self.open.take() {
            commands.entity(document).try_despawn();
        }
    }
}

/// Despawns the popup when it says it closed.
pub(super) fn forget_closed(
    mut events: MessageReader<UiEvent>,
    mut popup: ResMut<EffectPopup>,
    mut commands: Commands,
) {
    for event in events.read() {
        if event.value == UiValue::Closed && event.id == BOX {
            popup.close(&mut commands);
        }
    }
}

/// Follows which icon the pointer is on: keeps its popup open, or opens one
/// once it has rested there, in place of one about another not locked.
#[allow(clippy::too_many_arguments)]
pub(super) fn follow_pointer(
    hit: Res<Hit>,
    icons: Query<&EffectIcon>,
    held: Res<HeldObject>,
    real: Res<Time<Real>>,
    locks: Query<(&Popup, &UiId)>,
    (mut popup, mut kept): (ResMut<EffectPopup>, ResMut<KeptOpen>),
    (sheet, clock, time, wording): (
        Res<CharacterSheet>,
        Res<ServerTick>,
        Res<Time>,
        Res<Wording>,
    ),
    mut dom: Dom,
    mut commands: Commands,
) {
    let pointed = hit.first(|entity| icons.get(entity).ok().map(|icon| icon.0));
    let Some(source) = pointed.filter(|_| held.thing().is_none()) else {
        popup.resting = None;
        return;
    };
    if popup
        .open
        .as_ref()
        .is_some_and(|(open, ..)| *open == source)
    {
        kept.0.push(BOX);
        popup.resting = None;
        return;
    }
    let now = real.elapsed_secs();
    let since = match popup.resting {
        Some((was, since)) if was == source => since,
        _ => {
            popup.resting = Some((source, now));
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
    let Some(effect) = sheet.0.effects.0.iter().find(|e| e.source == source) else {
        return;
    };
    let document = commands.spawn(document(&EFFECT_POPUP)).id();
    let at = (hit.at.unwrap_or_default() + OFFSET).round();
    dom.set("effect_popup/box", "left", &at.x.to_string());
    dom.set("effect_popup/box", "top", &at.y.to_string());
    let lang = &wording.0;
    dom.set_text("effect_popup/name", name(source, lang));
    let lines = bonus_lines(effect, lang);
    dom.set_flag("effect_popup/bonuses", "hidden", lines.is_empty());
    dom.set_text("effect_popup/bonuses", lines.join("\n"));
    let ends = ends_early(effect, lang);
    dom.set_flag("effect_popup/ends", "hidden", ends.is_none());
    dom.set_text("effect_popup/ends", ends.unwrap_or_default());
    let left = left_words(effect, &clock, &time, &wording);
    dom.set_text("effect_popup/left", left.clone());
    kept.0.push(BOX);
    popup.open = Some((source, document, left));
}

/// "Ends in N s" now, or nothing before the server has said the tick.
fn left_words(effect: &Effect, clock: &ServerTick, time: &Time, wording: &Wording) -> String {
    let now = clock.now(time.elapsed_secs_f64());
    now.and_then(|now| ends_in(effect, now, &wording.0))
        .unwrap_or_default()
}

/// Counts the time left down while it is open, and closes it once its effect
/// has gone (ended, or ended by the server).
pub(super) fn count_down(
    mut popup: ResMut<EffectPopup>,
    (sheet, clock, time, wording): (
        Res<CharacterSheet>,
        Res<ServerTick>,
        Res<Time>,
        Res<Wording>,
    ),
    mut dom: Dom,
    mut commands: Commands,
) {
    let Some((source, _, said)) = &popup.open else {
        return;
    };
    let now = clock.now(time.elapsed_secs_f64());
    let effect = sheet.0.effects.0.iter().find(|e| e.source == *source);
    let lasting =
        effect.filter(|e| now.is_some_and(|now| e.end.is_some_and(|end| now < end as f64)));
    let Some(effect) = lasting else {
        popup.close(&mut commands);
        return;
    };
    let left = left_words(effect, &clock, &time, &wording);
    if left != *said {
        dom.set_text("effect_popup/left", left.clone());
        if let Some(open) = popup.open.as_mut() {
            open.2 = left;
        }
    }
}
