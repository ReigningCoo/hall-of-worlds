//! Starting the music: the SoundFont is loaded (in a browser, downloaded)
//! only once music is on, audible and allowed to play; then a synthesizer is
//! made from it and its stream played for good: natively by Bevy's audio, in
//! a browser by the music's own Web Audio output (`web_out`).

use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;

use bevy::asset::LoadState;
use bevy::audio::{AudioSinkPlayback, GlobalVolume, Volume};
use bevy::prelude::*;
use rustysynth::{Synthesizer, SynthesizerSettings};

#[cfg(target_arch = "wasm32")]
use super::decoder::MusicDecoder;
use super::decoder::SAMPLE_RATE;
use super::font::{PATH, SoundFontAsset};
use super::gesture::MayPlay;
use super::stream::{MusicEvent, MusicStream, Shared};
use crate::common::SoundSwitches;
use crate::sound::MusicVolume;

/// Most notes sounding at once: fewer in a browser, which has less time.
const VOICES: usize = match cfg!(target_arch = "wasm32") {
    true => 24,
    false => 64,
};

/// Where the music is.
#[derive(Resource, Default)]
pub(super) enum Music {
    /// Not wanted yet (off, silent, or no click yet in a browser).
    #[default]
    Waiting,
    Loading(Handle<SoundFontAsset>),
    Playing(Arc<Shared>),
    /// The SoundFont couldn't be read: no music this run.
    Failed,
}

impl Music {
    pub fn shared(&self) -> Option<&Arc<Shared>> {
        match self {
            Self::Playing(shared) => Some(shared),
            _ => None,
        }
    }
}

/// The entity playing the music stream.
#[derive(Component)]
pub(super) struct MusicPlayer;

/// Moves the music on: loading when wanted, playing once loaded.
#[allow(clippy::too_many_arguments)]
pub(super) fn start_music(
    mut music: ResMut<Music>,
    switches: Res<SoundSwitches>,
    volume: Res<MusicVolume>,
    may_play: Res<MayPlay>,
    asset_server: Res<AssetServer>,
    fonts: Res<Assets<SoundFontAsset>>,
    mut streams: ResMut<Assets<MusicStream>>,
    mut commands: Commands,
) {
    match &*music {
        Music::Waiting if switches.music_on() && volume.0 > 0.0 && may_play.0 => {
            *music = Music::Loading(asset_server.load(PATH));
        }
        Music::Loading(handle) => {
            if let Some(font) = fonts.get(handle) {
                *music = play(font, &mut streams, &mut commands);
            } else if let LoadState::Failed(error) = asset_server.load_state(handle) {
                warn!("No music: the SoundFont {PATH} couldn't be loaded: {error}");
                *music = Music::Failed;
            }
        }
        _ => {}
    }
}

/// A synthesizer from the SoundFont, its stream played.
fn play(
    font: &SoundFontAsset,
    streams: &mut Assets<MusicStream>,
    commands: &mut Commands,
) -> Music {
    let mut settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
    settings.maximum_polyphony = VOICES;
    settings.enable_reverb_and_chorus = !cfg!(target_arch = "wasm32");
    let synth = match Synthesizer::new(&font.0, &settings) {
        Ok(synth) => synth,
        Err(error) => {
            warn!("No music: the synthesizer couldn't start: {error:?}");
            return Music::Failed;
        }
    };
    let shared = Arc::new(Shared::new());
    if !output(synth, &shared, streams, commands) {
        return Music::Failed;
    }
    info!("Music started");
    Music::Playing(shared)
}

/// Natively, the stream played by Bevy's audio.
#[cfg(not(target_arch = "wasm32"))]
fn output(
    synth: Synthesizer,
    shared: &Arc<Shared>,
    streams: &mut Assets<MusicStream>,
    commands: &mut Commands,
) -> bool {
    let stream = streams.add(MusicStream {
        shared: shared.clone(),
        synth: Mutex::new(Some(synth)),
    });
    commands.spawn((
        Name::new("Music"),
        MusicPlayer,
        AudioPlayer(stream),
        PlaybackSettings::ONCE,
    ));
    true
}

/// In a browser, the decoder played by the music's own Web Audio output
/// (`web_out`): Bevy's keeps too little queued.
#[cfg(target_arch = "wasm32")]
fn output(
    synth: Synthesizer,
    shared: &Arc<Shared>,
    _streams: &mut Assets<MusicStream>,
    _commands: &mut Commands,
) -> bool {
    super::web_out::start(MusicDecoder::new(Some(synth), shared.clone()))
}

/// Bevy sets a new sound's volume from `GlobalVolume` only when it starts,
/// so the music's sink stays at full and the stream eases its own gain
/// (music × general) instead.
pub(super) fn sink_at_full(
    mut sinks: Query<&mut AudioSink, (Added<AudioSink>, With<MusicPlayer>)>,
) {
    for mut sink in &mut sinks {
        sink.set_volume(Volume::Linear(1.0));
    }
}

/// Tells the stream the gain to ease towards.
pub(super) fn follow_volume(
    music: Res<Music>,
    volume: Res<MusicVolume>,
    global: Res<GlobalVolume>,
) {
    if let Some(shared) = music.shared() {
        shared.set_target_gain(volume.0 * global.volume.to_linear());
    }
}

/// Passes the game's MIDI events to the stream; dropped while nothing plays.
pub(super) fn send_events(music: Res<Music>, mut events: MessageReader<MusicEvent>) {
    let shared = music.shared();
    for event in events.read() {
        if let Some(shared) = shared {
            shared.send(*event);
        }
    }
}
