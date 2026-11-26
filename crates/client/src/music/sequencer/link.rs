//! Links between tunes, instead of rests: over 1 to 3 bars the tempo glides
//! a unit at a time to the next tune's, the drone moves to its final half a
//! bar in, and the melody holds a bridging note on that final while the old
//! tune's last notes die away. Then the next tune starts (`pass`).

use std::sync::Arc;

use super::super::stream::MusicEvent;
use super::super::tune::{PPQ, Signature, Tune};
use super::due::Due;
use super::{DRONE, MELODY, Sequencer, pass};

/// The drone's loudness.
const DRONE_VELOCITY: u8 = 45;
/// The bridging note's.
const BRIDGE_VELOCITY: u8 = 45;
/// The most bars a link lasts.
const MOST_BARS: u64 = 3;

/// Starts the link at `start` from the tune playing into the next.
pub(super) fn begin_link(sequencer: &mut Sequencer, start: u64) {
    let Some(playing) = &sequencer.playing else {
        return;
    };
    let tunes = &playing.program.tunes;
    let next = tunes[(playing.tune + 1) % tunes.len()].clone();
    let signature = tunes[playing.tune].last_signature();
    link_into(sequencer, start, signature, next);
}

/// A link at `start` in bars of `signature` into `next`, which starts when
/// it ends (`end_link`: the program's tune after the one playing).
pub(super) fn link_into(
    sequencer: &mut Sequencer,
    start: u64,
    signature: Signature,
    next: Arc<Tune>,
) {
    let Some(playing) = &sequencer.playing else {
        return;
    };
    let (bar, unit) = (signature.bar(), signature.unit());
    let bar_seconds = bar as f64 / PPQ as f64 * 60.0 / sequencer.rate();
    let wanted = f64::from(playing.program.players.link_seconds) / bar_seconds / 2.0;
    let bars = (wanted.round() as u64).clamp(1, MOST_BARS);
    let length = bars * bar;
    // The tempo glides a unit at a time, arriving as the next tune starts.
    let units = length / unit;
    let from = sequencer.tempo;
    for step in 1..=units {
        let tempo = from + (next.tempo() - from) * step as f64 / units as f64;
        sequencer.queue.push(start + step * unit, Due::Tempo(tempo));
    }
    move_drone(sequencer, start + bar / 2, next.final_class);
    let bridge = next.final_near_start();
    let held = length * 85 / 100;
    sequencer.note(MELODY, bridge, BRIDGE_VELOCITY, start + bar / 4, held);
    sequencer.queue.push(start + length, Due::LinkEnd);
}

/// The link is over: the next tune starts, its other parts coming back in.
pub(super) fn end_link(sequencer: &mut Sequencer) {
    let Some(playing) = &mut sequencer.playing else {
        return;
    };
    playing.tune = (playing.tune + 1) % playing.program.tunes.len();
    playing.loops = 0;
    let start = sequencer.tick;
    pass::start_pass(sequencer, start, true);
}

/// Moves the drone (if the players have one) to the final and fifth of
/// `class` at `at`, a third octave's final; nothing if it is there already.
pub(super) fn move_drone(sequencer: &mut Sequencer, at: u64, class: u8) {
    let Some(playing) = &mut sequencer.playing else {
        return;
    };
    if playing.program.players.drone.is_none() {
        return;
    }
    let root = 48 + class % 12;
    let keys = vec![root, root + 7];
    if playing.drone == keys {
        return;
    }
    let old = std::mem::replace(&mut playing.drone, keys.clone());
    for key in old {
        let off = MusicEvent::note_off(DRONE, key);
        sequencer.queue.push(at, Due::Midi(off));
    }
    for key in keys {
        let on = MusicEvent::note_on(DRONE, key, DRONE_VELOCITY);
        sequencer.queue.push(at, Due::Midi(on));
    }
}
