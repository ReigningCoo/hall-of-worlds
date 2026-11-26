//! The music as a Bevy audio source: one endless stream that rustysynth
//! renders while Bevy's audio pulls samples (`decoder`). The game talks to it
//! through `Shared`: MIDI events queued, and the gain it should play at.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use bevy::audio::Decodable;
use bevy::prelude::*;
use rustysynth::Synthesizer;

use super::decoder::MusicDecoder;
use super::sequencer::Program;

/// One MIDI channel message for the synthesizer: a channel (0 to 15), a
/// command (`0x90` note on, `0x80` note off, `0xC0` program change, `0xB0`
/// control change, ...) and its two data bytes.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MusicEvent {
    pub channel: u8,
    pub command: u8,
    pub data1: u8,
    pub data2: u8,
}

impl MusicEvent {
    pub fn note_on(channel: u8, key: u8, velocity: u8) -> Self {
        Self {
            channel,
            command: 0x90,
            data1: key,
            data2: velocity,
        }
    }

    pub fn note_off(channel: u8, key: u8) -> Self {
        Self {
            channel,
            command: 0x80,
            data1: key,
            data2: 0,
        }
    }

    /// The bank a channel's next program comes from (control change 0): 0
    /// for General MIDI's instruments, 8 for their variations.
    pub fn bank(channel: u8, bank: u8) -> Self {
        Self {
            channel,
            command: 0xB0,
            data1: 0,
            data2: bank,
        }
    }

    /// The instrument a channel plays, by General MIDI's program from 0.
    pub fn program(channel: u8, program: u8) -> Self {
        Self {
            channel,
            command: 0xC0,
            data1: program,
            data2: 0,
        }
    }
}

/// What the sequencer is told to do, at the next block.
pub(super) enum Command {
    Play(Program),
    Stop,
}

/// What the game and the audio side share.
pub(super) struct Shared {
    /// Events waiting for the next block of samples.
    pub events: Mutex<Vec<MusicEvent>>,
    /// Commands for the sequencer, waiting likewise.
    pub commands: Mutex<Vec<Command>>,
    /// The gain to play at (music × general, linear), as `f32` bits.
    target_gain: AtomicU32,
}

impl Shared {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            commands: Mutex::new(Vec::new()),
            target_gain: AtomicU32::new(0),
        }
    }

    pub fn set_target_gain(&self, gain: f32) {
        self.target_gain.store(gain.to_bits(), Ordering::Relaxed);
    }

    pub fn target_gain(&self) -> f32 {
        f32::from_bits(self.target_gain.load(Ordering::Relaxed))
    }

    pub fn command(&self, command: Command) {
        if let Ok(mut commands) = self.commands.lock() {
            commands.push(command);
        }
    }

    pub fn send(&self, event: MusicEvent) {
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }
}

/// The music stream, played once by an `AudioPlayer`: it never ends.
#[derive(Asset, TypePath)]
pub(super) struct MusicStream {
    pub shared: Arc<Shared>,
    /// Taken by the first decoder Bevy asks for: there is only one.
    pub synth: Mutex<Option<Synthesizer>>,
}

impl Decodable for MusicStream {
    type Decoder = MusicDecoder;

    fn decoder(&self) -> MusicDecoder {
        let synth = self.synth.lock().ok().and_then(|mut synth| synth.take());
        MusicDecoder::new(synth, self.shared.clone())
    }
}
