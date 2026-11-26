//! Once round a tune: the melody's notes, the second player on its beats an
//! octave down (heterophony), and percussion from the rhythmic modes, all
//! queued when the pass starts; then whether it goes round again or links.

use super::super::tune::{Signature, Tune};
use super::due::Due;
use super::{DRUMS, LOOPS, MELODY, SECOND, Sequencer, link};

/// How loud the second player is, against the melody.
const SECOND_SHARE: f32 = 0.7;
/// Bars over which the second player and percussion come back in after a link.
const ENTRY_BARS: u64 = 2;
/// How loud the percussion's downbeat is.
const DRUM_VELOCITY: f32 = 100.0;

/// Queues a pass of the tune playing from `start`; `entering` after a link,
/// so the second player and percussion fade in.
pub(super) fn start_pass(sequencer: &mut Sequencer, start: u64, entering: bool) {
    let Some(playing) = &sequencer.playing else {
        return;
    };
    let tune = playing.program.tunes[playing.tune].clone();
    let players = playing.program.players.clone();
    let fade = |at: u64| match entering {
        true => entry_share(&tune.signatures[0], at),
        false => 1.0,
    };
    for &(at, tempo) in &tune.tempos {
        sequencer.queue.push(start + at, Due::Tempo(tempo));
    }
    for (index, note) in tune.notes.iter().enumerate() {
        let played = note.length - note.length / 50;
        sequencer.note(MELODY, note.key, note.velocity, start + note.at, played);
        let signature = tune.signature_at(note.at);
        if players.second.is_some() && signature.in_bar(note.at) % signature.beat() == 0 {
            let held = held_to_next_beat(&tune, index, &signature);
            let velocity = scaled(note.velocity, SECOND_SHARE * fade(note.at));
            let key = note.key.saturating_sub(12).max(24);
            sequencer.note(SECOND, key, velocity, start + note.at, held);
        }
    }
    if let Some(drums) = players.percussion {
        let mut at = 0;
        while at < tune.length {
            let signature = tune.signature_at(at);
            let unit = signature.unit();
            let level = drum_level(&signature, signature.in_bar(at) / unit) * fade(at);
            let key = match signature.in_bar(at) == 0 {
                true => drums.strong,
                false => drums.light,
            };
            sequencer.note(
                DRUMS,
                key,
                scaled(1, DRUM_VELOCITY * level),
                start + at,
                unit / 2,
            );
            at += unit;
        }
    }
    sequencer.queue.push(start + tune.length, Due::PassEnd);
}

/// The tune has been round: again, or (twice round, with more tunes) a link.
pub(super) fn end_pass(sequencer: &mut Sequencer) {
    let start = sequencer.tick;
    let Some(playing) = &mut sequencer.playing else {
        return;
    };
    playing.loops += 1;
    if playing.loops >= LOOPS && playing.program.tunes.len() > 1 {
        link::begin_link(sequencer, start);
    } else {
        start_pass(sequencer, start, false);
    }
}

/// How long the second player holds a note on a beat: until the next beat
/// a melody note starts on (at most four beats), and never less than the
/// note itself or past the end.
fn held_to_next_beat(tune: &Tune, index: usize, signature: &Signature) -> u64 {
    let note = tune.notes[index];
    let beat = signature.beat();
    let starts_at = |at: u64| tune.notes[index + 1..].iter().any(|later| later.at == at);
    let mut held = beat;
    while held < 4 * beat && note.at + held < tune.length && !starts_at(note.at + held) {
        held += beat;
    }
    let held = held.max(note.length).min(tune.length - note.at);
    held - held / 50
}

/// The share of full loudness at `at` while coming back in after a link.
fn entry_share(signature: &Signature, at: u64) -> f32 {
    let unit = signature.unit();
    let units = ENTRY_BARS * u64::from(signature.numerator.max(1));
    ((at / unit + 1) as f32 / units as f32).min(1.0)
}

/// The percussion's level on a unit of the bar (0 for none): the first
/// rhythmic mode, long then short, strong on the downbeat and lighter after.
pub(super) fn drum_level(signature: &Signature, unit: u64) -> f32 {
    let meter = u64::from(signature.numerator);
    match (unit, signature.beat_units()) {
        (0, _) => 1.0,
        (unit, 3) if unit % 3 == 2 => 0.45,
        (unit, 3) if unit % 3 == 0 => 0.7,
        (_, 3) => 0.0,
        (unit, _) if unit + 1 == meter => 0.35,
        (unit, _) if unit % 2 == 0 => 0.55,
        _ => 0.0,
    }
}

/// A velocity times a share, kept to MIDI's range (0 drops the note).
fn scaled(velocity: u8, share: f32) -> u8 {
    (f32::from(velocity) * share).round().clamp(0.0, 127.0) as u8
}
