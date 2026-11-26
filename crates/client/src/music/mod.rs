//! Music: a SoundFont's instruments (`assets/music/music.sf2`) played by
//! rustysynth inside a custom Bevy audio source, at the Music volume (eased,
//! times General). The SoundFont and the tunes are loaded only once music is
//! on and audible, and in a browser after the first click, tap or key.
//!
//! Tunes are MIDI files named by playlists (`assets/music/playlists/`, which
//! mods add to), played by the sequencer inside the stream: a melody, a
//! second player, a drone and percussion, each tune twice and then linked
//! into the next. Each place has its own tunes and players (`place`,
//! `place_now`, `sets`), changing at the next bar line; the debug menu plays
//! every tune, one alone or by place (`PlayTunes`), or the test
//! (`PlayTestMusic`).

mod decoder;
mod font;
mod gesture;
mod library;
mod loaders;
mod midi;
mod place;
mod place_now;
mod play_tunes;
mod playlist;
mod sequencer;
mod sets;
mod start;
mod stream;
mod test_tune;
mod tune;

pub use library::TuneList;
pub use play_tunes::PlayTunes;
pub use stream::MusicEvent;
pub use test_tune::PlayTestMusic;

use bevy::audio::AddAudioSource;
use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<font::SoundFontAsset>()
            .init_asset_loader::<font::SoundFontLoader>()
            .init_asset::<loaders::TuneAsset>()
            .init_asset_loader::<loaders::TuneLoader>()
            .init_asset::<loaders::PlaylistAsset>()
            .init_asset_loader::<loaders::PlaylistLoader>()
            .add_audio_source::<stream::MusicStream>()
            .add_message::<MusicEvent>()
            .add_message::<PlayTestMusic>()
            .add_message::<PlayTunes>()
            .init_resource::<gesture::MayPlay>()
            .init_resource::<start::Music>()
            .init_resource::<library::Library>()
            .init_resource::<TuneList>()
            .init_resource::<play_tunes::Asked>()
            .init_resource::<place_now::PlaceNow>()
            .init_resource::<test_tune::TestTune>()
            .add_systems(
                Update,
                (
                    gesture::watch_for_gesture,
                    start::start_music,
                    library::load_library,
                    place_now::follow_place,
                    play_tunes::play_tunes,
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
