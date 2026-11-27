//! The Music panel's systems: what it is told and what it shows.

use bevy::prelude::*;

use super::super::actions::DebugAction;
use super::OpenPanels;
use super::music::{ChosenTune, CopyTitle, FORM};
use crate::dom::{Dom, UiEvent, UiValue};
use crate::music::{ForcedCombat, PlayTunes, PlayingTune, TuneList};
use crate::widgets::{ButtonPressed, copy_to_clipboard};

/// The tune the dropdown shows: the one chosen, else the one playing.
fn shown(chosen: &ChosenTune, playing: &PlayingTune) -> Option<usize> {
    chosen.0.or(playing.0)
}

/// A tune chosen plays alone; the clipboard copies the shown tune's title; Attack
/// forces combat music on or off; Reset goes back to music by place.
#[allow(clippy::too_many_arguments)]
pub(super) fn take_music_events(
    mut events: MessageReader<UiEvent>,
    mut pressed: MessageReader<ButtonPressed>,
    copy_buttons: Query<(), With<CopyTitle>>,
    mut chosen: ResMut<ChosenTune>,
    mut forced: ResMut<ForcedCombat>,
    playing: Res<PlayingTune>,
    list: Res<TuneList>,
    mut tunes: MessageWriter<PlayTunes>,
) {
    if pressed.read().any(|press| copy_buttons.contains(press.0)) {
        let tune = shown(&chosen, &playing).and_then(|at| list.0.get(at));
        if let Some(tune) = tune {
            copy_to_clipboard(&tune.title);
        }
    }
    for event in events.read() {
        let (FORM, id) = event.id.parts() else {
            continue;
        };
        match (id, &event.value) {
            ("tune", UiValue::Choice { index, .. }) => {
                chosen.0 = Some(*index);
                tunes.write(PlayTunes::One(*index));
            }
            ("attack", UiValue::Pressed) => forced.0 = !forced.0,
            ("reset", UiValue::Pressed) => {
                chosen.0 = None;
                forced.0 = false;
                tunes.write(PlayTunes::ByPlace);
            }
            _ => {}
        }
    }
}

/// Asked from elsewhere (Play All Tunes), the dropdown follows again.
pub(super) fn forget_choice(mut asked: MessageReader<PlayTunes>, mut chosen: ResMut<ChosenTune>) {
    for tunes in asked.read() {
        if !matches!(tunes, PlayTunes::One(_)) && chosen.0.is_some() {
            chosen.0 = None;
        }
    }
}

/// What was last shown, so only changes are sent.
#[derive(Default, PartialEq)]
pub(super) struct Shown {
    titles: usize,
    tune: Option<usize>,
    forced: bool,
}

/// Lists the tunes, shows the one playing (or chosen) with where it is
/// played, and Attack pressed while combat music is forced: on opening and
/// whenever they change.
pub(super) fn show_music(
    open: Res<OpenPanels>,
    list: Res<TuneList>,
    chosen: Res<ChosenTune>,
    playing: Res<PlayingTune>,
    forced: Res<ForcedCombat>,
    mut was: Local<Option<Shown>>,
    mut dom: Dom,
) {
    if !open.is_open(DebugAction::PlayTune) {
        *was = None;
        return;
    }
    let now = Shown {
        titles: list.0.len(),
        tune: shown(&chosen, &playing),
        forced: forced.0,
    };
    let before = was.take();
    let id = |part: &str| format!("{FORM}/{part}");
    let titles_changed = before.as_ref().is_none_or(|b| b.titles != now.titles);
    if titles_changed && !list.0.is_empty() {
        let titles = list.0.iter().map(|tune| tune.title.clone()).enumerate();
        dom.set_options(
            &id("tune"),
            titles.map(|(at, title)| (at.to_string(), title)),
        );
    }
    if titles_changed || before.as_ref().is_none_or(|b| b.tune != now.tune) {
        let tune = now
            .tune
            .and_then(|at| list.0.get(at).map(|tune| (at, tune)));
        if let Some((at, _)) = tune {
            dom.set(&id("tune"), "value", &at.to_string());
        }
        let used = match (tune, list.0.is_empty()) {
            (Some((_, tune)), _) => format!("Played in: {}", tune.used),
            (None, true) => "No tunes: music is off or still loading.".to_owned(),
            (None, false) => "Nothing playing.".to_owned(),
        };
        dom.set_text(&id("used"), used);
    }
    if before.as_ref().is_none_or(|b| b.forced != now.forced) {
        dom.set_flag(&id("attack"), "selected", now.forced);
    }
    *was = Some(now);
}
