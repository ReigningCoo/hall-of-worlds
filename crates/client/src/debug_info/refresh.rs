//! Keeping the string up to date while the panel is open: remade whenever what
//! it says changes, the description going in once typing has stopped for
//! `TYPING_PAUSE`; copying takes the latest description at once.

use bevy::prelude::*;
use shared::debug_report::{DebugReport, encode};

use super::gather::Sources;
use super::markup::MOST_PROBLEM;
use super::panel::CopyButton;
use super::platform::utc_now;
use super::state::{Panel, TYPING_PAUSE};
use crate::dom::{Dom, UiEvent, UiValue};
use crate::widgets::{ButtonPressed, copy_to_clipboard};

/// What is typed into the description, and the copy button.
pub(super) fn read_events(
    mut events: MessageReader<UiEvent>,
    mut pressed: MessageReader<ButtonPressed>,
    copy_buttons: Query<(), With<CopyButton>>,
    time: Res<Time<Real>>,
    mut panel: ResMut<Panel>,
    sources: Sources,
    mut dom: Dom,
) {
    for event in events.read() {
        if let ("debug_info", "problem") = event.id.parts()
            && let UiValue::Text(words) = &event.value
        {
            // (The box keeps to it too.)
            panel.typed = words.chars().take(MOST_PROBLEM).collect();
            panel.typed_at = Some(time.elapsed_secs_f64());
        }
    }
    let copy = pressed.read().any(|press| copy_buttons.contains(press.0));
    if copy && panel.is_open() {
        panel.take_typed();
        remake(&mut panel, &sources, &mut dom);
        copy_to_clipboard(&panel.string);
    }
}

/// Remakes the string while the panel is open.
pub(super) fn refresh(
    time: Res<Time<Real>>,
    mut panel: ResMut<Panel>,
    sources: Sources,
    mut dom: Dom,
) {
    if !panel.is_open() {
        return;
    }
    if panel
        .typed_at
        .is_some_and(|at| time.elapsed_secs_f64() - at >= TYPING_PAUSE)
    {
        panel.take_typed();
    }
    remake(&mut panel, &sources, &mut dom);
}

/// Makes the string again if anything it says has changed, and shows it.
fn remake(panel: &mut Panel, sources: &Sources, dom: &mut Dom) {
    let report = sources.report(&panel.problem);
    if panel.made_from.as_ref() == Some(&report) {
        return;
    }
    panel.string = encode(&DebugReport {
        utc: utc_now(),
        ..report.clone()
    });
    panel.made_from = Some(report);
    dom.set("debug_info/string", "value", &panel.string);
}

/// The string as it would be made now, with the latest description: for the
/// debug API.
#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
pub(super) fn string_now(panel: &mut Panel, sources: &Sources, dom: &mut Dom) -> String {
    panel.take_typed();
    remake(panel, sources, dom);
    panel.string.clone()
}
