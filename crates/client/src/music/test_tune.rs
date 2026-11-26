//! Play Test Music (debug menu): a short phrase on each instrument the
//! SoundFont keeps, then the drums, to hear the synthesizer and SoundFont.

use bevy::prelude::*;

use super::start::Music;
use super::stream::MusicEvent;
use crate::common::SoundSwitches;
use crate::sound::MusicVolume;

/// Ask for the test music, from the start.
#[derive(Message, Clone, Copy, Debug, Default)]
pub struct PlayTestMusic;

/// The instruments the test plays, in order, as (bank, program from 0):
/// General MIDI's in bank 0, the hurdy-gurdy and cornemuse in bank 8.
pub(super) const INSTRUMENTS: [(u8, u8); 18] = [
    (0, 74),
    (0, 110),
    (8, 110),
    (0, 24),
    (0, 46),
    (0, 15),
    (0, 111),
    (0, 56),
    (0, 57),
    (0, 109),
    (8, 109),
    (0, 19),
    (0, 20),
    (0, 53),
    (0, 14),
    (0, 112),
    (0, 47),
    (0, 116),
];

/// The phrase each plays: D Dorian up and down, in seconds a note.
const PHRASE: [u8; 8] = [62, 64, 65, 67, 69, 67, 65, 64];
const NOTE: f32 = 0.22;
/// Drum keys played one after another on channel 10.
const DRUMS: [u8; 12] = [36, 38, 41, 45, 54, 60, 61, 62, 64, 75, 76, 81];

/// The test as timed events: seconds from its start, and the event.
pub(super) fn test_tune() -> Vec<(f32, MusicEvent)> {
    let mut events = Vec::new();
    let mut at = 0.0;
    for (bank, program) in INSTRUMENTS {
        events.push((at, MusicEvent::bank(0, bank)));
        events.push((at, MusicEvent::program(0, program)));
        for key in PHRASE {
            events.push((at, MusicEvent::note_on(0, key, 100)));
            events.push((at + NOTE * 0.9, MusicEvent::note_off(0, key)));
            at += NOTE;
        }
        // The last note held a little, then a breath.
        events.push((at, MusicEvent::note_on(0, 62, 100)));
        events.push((at + NOTE * 3.0, MusicEvent::note_off(0, 62)));
        at += NOTE * 4.0;
    }
    for key in DRUMS {
        events.push((at, MusicEvent::note_on(9, key, 110)));
        events.push((at + NOTE, MusicEvent::note_off(9, key)));
        at += NOTE * 2.0;
    }
    events
}

/// The test playing: events left, and when it started (once music plays).
#[derive(Resource, Default)]
pub(super) struct TestTune {
    events: Vec<(f32, MusicEvent)>,
    started: Option<f32>,
    next: usize,
}

/// Starts the test when asked (once the music is playing) and sends its
/// events as their times come.
pub(super) fn play_test_tune(
    mut asked: MessageReader<PlayTestMusic>,
    mut tune: ResMut<TestTune>,
    music: Res<Music>,
    switches: Res<SoundSwitches>,
    volume: Res<MusicVolume>,
    time: Res<Time>,
    mut out: MessageWriter<MusicEvent>,
) {
    if asked.read().count() > 0 {
        if !switches.music_on() || volume.0 == 0.0 {
            warn!(
                "Play Test Music: music is off or silent (HOW_SOUND, HOW_MUSIC, Settings, Audio)"
            );
        }
        *tune = TestTune {
            events: test_tune(),
            started: None,
            next: 0,
        };
        out.write(MusicEvent {
            channel: 0,
            command: 0xB0,
            data1: 123,
            data2: 0,
        });
    }
    if tune.next >= tune.events.len() || music.shared().is_none() {
        return;
    }
    let now = time.elapsed_secs();
    let started = *tune.started.get_or_insert(now);
    while let Some((at, event)) = tune.events.get(tune.next) {
        if *at > now - started {
            break;
        }
        out.write(*event);
        tune.next += 1;
    }
}
