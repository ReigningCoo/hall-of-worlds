//! The volumes chosen in Settings, Audio, put into play.

use bevy::audio::{GlobalVolume, Volume};
use bevy::prelude::*;

use crate::common::{AudioPrefs, Preferences, SoundSwitches};

/// How loud music plays, as a linear gain on top of `GlobalVolume`: 0 while
/// music is muted or off (`HOW_MUSIC`, `HOW_SOUND`), so whatever plays music
/// can also stop when it is 0.
#[derive(Resource, Clone, Copy, PartialEq, Debug, Default)]
pub struct MusicVolume(pub f32);

/// The gain a percentage plays at: squared, so the slider's middle sounds
/// about half as loud rather than barely quieter.
pub(super) fn gain(percent: u8) -> f32 {
    let share = f32::from(percent.min(100)) / 100.0;
    share * share
}

/// The gains for everything and for music: (general, music).
pub(super) fn gains(prefs: AudioPrefs, switches: SoundSwitches) -> (f32, f32) {
    let general = match switches.sound && !prefs.general_muted {
        true => gain(prefs.general),
        false => 0.0,
    };
    let music = match switches.music_on() && !prefs.music_muted {
        true => gain(prefs.music),
        false => 0.0,
    };
    (general, music)
}

/// Sets `GlobalVolume` and `MusicVolume` when the preferences change (and at
/// the start).
pub(super) fn apply_volumes(
    preferences: Res<Preferences>,
    switches: Res<SoundSwitches>,
    global: Option<ResMut<GlobalVolume>>,
    mut music: ResMut<MusicVolume>,
    mut done: Local<bool>,
) {
    if *done && !preferences.is_changed() && !switches.is_changed() {
        return;
    }
    *done = true;
    let (general, music_gain) = gains(preferences.audio, *switches);
    if let Some(mut global) = global {
        global.volume = Volume::Linear(general);
    }
    music.set_if_neq(MusicVolume(music_gain));
}
