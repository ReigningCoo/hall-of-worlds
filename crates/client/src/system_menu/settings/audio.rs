//! Settings' Audio page: General and Music volumes, each a slider with a Mute
//! button drawn as a speaker, swapped for an Unmute button drawn as a
//! crossed-out speaker while muted (a button's picture is fixed once built). Disabled, with a note
//! saying why, while `HOW_SOUND` or `HOW_MUSIC` turns sound off for this run.

use bevy::prelude::*;

use crate::common::{AudioPrefs, Preferences, SoundSwitches};
use crate::dom::{Dom, UiEvent, UiValue};

/// Why no sound plays this run, if none does.
fn sound_note() -> &'static str {
    if cfg!(debug_assertions) {
        "Sound is off in development builds unless HOW_SOUND is true."
    } else {
        "Sound is off for this run: HOW_SOUND is false."
    }
}

const MUSIC_NOTE: &str = "Music is off for this run: HOW_MUSIC is false.";

/// The note shown above the rows, if any.
pub(super) fn note(switches: SoundSwitches) -> Option<&'static str> {
    if !switches.sound {
        Some(sound_note())
    } else if !switches.music {
        Some(MUSIC_NOTE)
    } else {
        None
    }
}

/// One row: its name in ids, its percent, whether muted, whether usable.
fn show_row(dom: &mut Dom, row: &str, percent: u8, muted: bool, usable: bool) {
    let id = |part: &str| format!("settings.audio/{row}{part}");
    dom.set(&id(""), "value", &percent.to_string());
    dom.set_text(&id("_percent"), format!("{percent}%"));
    dom.set_flag(&id("_mute"), "hidden", muted);
    dom.set_flag(&id("_unmute"), "hidden", !muted);
    dom.set_flag(&id(""), "disabled", !usable);
    dom.set_flag(&id("_mute"), "disabled", !usable);
    dom.set_flag(&id("_unmute"), "disabled", !usable);
}

/// Shows the page as the preferences have it, when they change (and at the
/// start).
pub(in crate::system_menu) fn show_audio_page(
    preferences: Res<Preferences>,
    switches: Res<SoundSwitches>,
    mut dom: Dom,
    mut shown: Local<Option<(AudioPrefs, SoundSwitches)>>,
) {
    let now = (preferences.audio, *switches);
    if *shown == Some(now) {
        return;
    }
    *shown = Some(now);
    let (audio, switches) = now;
    match note(switches) {
        Some(words) => {
            dom.set_text("settings.audio/off_note", words);
            dom.set_flag("settings.audio/off_note", "hidden", false);
        }
        None => dom.set_flag("settings.audio/off_note", "hidden", true),
    }
    let (general, music) = (audio.general, audio.music);
    show_row(
        &mut dom,
        "general",
        general,
        audio.general_muted,
        switches.sound,
    );
    let music_on = switches.music_on();
    show_row(&mut dom, "music", music, audio.music_muted, music_on);
}

/// Puts the page's choices into the preferences.
pub(in crate::system_menu) fn use_audio_page(
    mut events: MessageReader<UiEvent>,
    mut preferences: ResMut<Preferences>,
) {
    for event in events.read() {
        let mut audio = preferences.audio;
        match (event.id.parts(), &event.value) {
            (("settings.audio", "general"), UiValue::Number(value)) => {
                audio.general = percent(*value);
            }
            (("settings.audio", "music"), UiValue::Number(value)) => {
                audio.music = percent(*value);
            }
            (("settings.audio", "general_mute"), UiValue::Pressed) => audio.general_muted = true,
            (("settings.audio", "general_unmute"), UiValue::Pressed) => audio.general_muted = false,
            (("settings.audio", "music_mute"), UiValue::Pressed) => audio.music_muted = true,
            (("settings.audio", "music_unmute"), UiValue::Pressed) => audio.music_muted = false,
            _ => continue,
        }
        // Only a real change, so the preferences aren't saved for nothing.
        if preferences.audio != audio {
            preferences.audio = audio;
        }
    }
}

/// A slider's place as a whole percentage.
fn percent(value: f32) -> u8 {
    value.round().clamp(0.0, 100.0) as u8
}
