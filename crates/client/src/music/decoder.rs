//! The samples Bevy's audio pulls: rustysynth's output a block at a time,
//! left and right interleaved, at a gain eased towards its target so volume
//! changes never click.

use std::num::NonZero;
use std::sync::Arc;
use std::time::Duration;

use bevy::audio::Source;
use rustysynth::Synthesizer;

use super::stream::Shared;

/// Samples a second the synthesizer renders at.
pub(super) const SAMPLE_RATE: u32 = 44_100;

/// How far the gain moves towards its target each sample: about 30 ms to
/// get most of the way at 44.1 kHz.
const EASING: f32 = 0.000_75;

pub struct MusicDecoder {
    synth: Option<Synthesizer>,
    shared: Arc<Shared>,
    left: Vec<f32>,
    right: Vec<f32>,
    /// The next sample in the block (left and right count separately).
    at: usize,
    gain: f32,
}

impl MusicDecoder {
    pub(super) fn new(synth: Option<Synthesizer>, shared: Arc<Shared>) -> Self {
        let block = synth.as_ref().map_or(64, Synthesizer::get_block_size);
        Self {
            synth,
            shared,
            left: vec![0.0; block],
            right: vec![0.0; block],
            at: 2 * block,
            gain: 0.0,
        }
    }

    /// Plays the events waiting, then renders the next block (or silence
    /// while the music is silent, to save the work).
    fn next_block(&mut self) {
        self.at = 0;
        let Some(synth) = self.synth.as_mut() else {
            return;
        };
        let events = match self.shared.events.lock() {
            Ok(mut events) => std::mem::take(&mut *events),
            Err(_) => Vec::new(),
        };
        for event in events {
            synth.process_midi_message(
                i32::from(event.channel),
                i32::from(event.command),
                i32::from(event.data1),
                i32::from(event.data2),
            );
        }
        if self.gain == 0.0 && self.shared.target_gain() == 0.0 {
            self.left.fill(0.0);
            self.right.fill(0.0);
        } else {
            synth.render(&mut self.left, &mut self.right);
        }
    }
}

impl Iterator for MusicDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.at >= 2 * self.left.len() {
            self.next_block();
        }
        let index = self.at / 2;
        let sample = match self.at % 2 {
            0 => {
                // Once per pair, so both sides play at the same gain.
                let target = self.shared.target_gain();
                self.gain = eased(self.gain, target);
                self.left[index]
            }
            _ => self.right[index],
        };
        self.at += 1;
        Some(sample * self.gain)
    }
}

/// The gain one sample further towards its target, landing on it when close.
pub(super) fn eased(gain: f32, target: f32) -> f32 {
    let next = gain + (target - gain) * EASING;
    match (target - next).abs() < 0.000_1 {
        true => target,
        false => next,
    }
}

impl Source for MusicDecoder {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> NonZero<u16> {
        NonZero::new(2).expect("two channels")
    }

    fn sample_rate(&self) -> NonZero<u32> {
        NonZero::new(SAMPLE_RATE).expect("a sample rate")
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
