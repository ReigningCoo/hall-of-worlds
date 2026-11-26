//! Music: a SoundFont's instruments (`assets/music/music.sf2`) played by
//! rustysynth inside a custom Bevy audio source, at the Music volume (eased,
//! times General). The SoundFont is loaded only once music is on and audible,
//! and in a browser after the first click, tap or key. Tunes come with the
//! sequencer (music plan PR 3); for now the game sends MIDI events
//! (`MusicEvent`), and the debug menu's Play Test Music plays a test.

mod decoder;
mod font;
mod gesture;
mod start;
mod stream;
mod test_tune;

pub use stream::MusicEvent;
pub use test_tune::PlayTestMusic;

use bevy::audio::AddAudioSource;
use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<font::SoundFontAsset>()
            .init_asset_loader::<font::SoundFontLoader>()
            .add_audio_source::<stream::MusicStream>()
            .add_message::<MusicEvent>()
            .add_message::<PlayTestMusic>()
            .init_resource::<gesture::MayPlay>()
            .init_resource::<start::Music>()
            .init_resource::<test_tune::TestTune>()
            .add_systems(
                Update,
                (
                    gesture::watch_for_gesture,
                    start::start_music,
                    test_tune::play_test_tune,
                    start::send_events,
                    start::follow_volume,
                    start::sink_at_full,
                )
                    .chain(),
            );
    }
}

#[cfg(test)]
mod tests;
