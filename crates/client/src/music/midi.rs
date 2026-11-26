//! Reading a tune from a standard MIDI file with `midly`: every note not on
//! the drum channel, tempo and time signature changes, the track name as its
//! title, the "mode: ... on X" text for its final and the "loop" marker for
//! where it goes round. Ticks are scaled to `tune::PPQ`.

use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};

use super::tune::{Note, PPQ, Signature, Tune};

/// The drum channel (10, counted from 1), whose notes are no melody.
const DRUMS: u8 = 9;

/// A tune from a MIDI file's bytes, or why it can't be one.
pub(super) fn read_tune(bytes: &[u8]) -> Result<Tune, String> {
    let smf = Smf::parse(bytes).map_err(|error| format!("not a MIDI file: {error}"))?;
    let per_quarter = match smf.header.timing {
        Timing::Metrical(ticks) => u64::from(ticks.as_int().max(1)),
        Timing::Timecode(..) => return Err("timed in frames, not beats".into()),
    };
    let scale = |ticks: u64| ticks * PPQ / per_quarter;
    let mut read = Read::default();
    for track in &smf.tracks {
        let mut at = 0;
        // Notes sounding, by key: when each started and how loud.
        let mut sounding: Vec<(u8, u64, u8)> = Vec::new();
        for event in track {
            at += u64::from(event.delta.as_int());
            let tick = scale(at);
            read.end = read.end.max(tick);
            match event.kind {
                TrackEventKind::Midi { channel, message } if channel.as_int() != DRUMS => {
                    let (key, on) = match message {
                        MidiMessage::NoteOn { key, vel } => (key.as_int(), vel.as_int()),
                        MidiMessage::NoteOff { key, .. } => (key.as_int(), 0),
                        _ => continue,
                    };
                    if let Some(at) = sounding.iter().position(|s| s.0 == key) {
                        let (key, start, velocity) = sounding.remove(at);
                        read.note(key, start, tick, velocity);
                    }
                    if on > 0 {
                        sounding.push((key, tick, on));
                    }
                }
                TrackEventKind::Meta(meta) => read.meta(meta, tick),
                _ => {}
            }
        }
    }
    read.into_tune()
}

/// What has been read so far.
#[derive(Default)]
struct Read {
    title: Option<String>,
    notes: Vec<Note>,
    tempos: Vec<(u64, f64)>,
    signatures: Vec<Signature>,
    final_class: Option<u8>,
    looped_at: Option<u64>,
    end: u64,
}

impl Read {
    fn note(&mut self, key: u8, start: u64, end: u64, velocity: u8) {
        if end > start {
            let length = end - start;
            self.notes.push(Note {
                at: start,
                length,
                key,
                velocity,
            });
        }
    }

    fn meta(&mut self, meta: MetaMessage, at: u64) {
        match meta {
            MetaMessage::TrackName(name) if self.title.is_none() => {
                self.title = Some(String::from_utf8_lossy(name).trim().to_owned());
            }
            MetaMessage::Tempo(micros) => {
                let qpm = 60_000_000.0 / f64::from(micros.as_int().max(1));
                self.tempos.push((at, qpm));
            }
            MetaMessage::TimeSignature(numerator, power, ..) => {
                let denominator = 1u8.checked_shl(u32::from(power)).unwrap_or(4);
                self.signatures.push(Signature {
                    at,
                    numerator,
                    denominator,
                });
            }
            MetaMessage::Text(text) => {
                if let Some(class) = final_of(&String::from_utf8_lossy(text)) {
                    self.final_class = Some(class);
                }
            }
            MetaMessage::Marker(name) if name.eq_ignore_ascii_case(b"loop") => {
                self.looped_at = Some(at);
            }
            _ => {}
        }
    }

    fn into_tune(mut self) -> Result<Tune, String> {
        self.notes.sort_by_key(|note| (note.at, note.key));
        let last_end = self.notes.iter().map(|n| n.at + n.length).max();
        let length = match (self.looped_at, last_end) {
            (Some(at), _) if at > 0 => at,
            (_, Some(end)) => end.max(self.end),
            _ => return Err("no notes".into()),
        };
        self.notes.retain(|note| note.at < length);
        for note in &mut self.notes {
            note.length = note.length.min(length - note.at);
        }
        let Some(last) = self.notes.last() else {
            return Err("no notes".into());
        };
        let final_class = self.final_class.unwrap_or(last.key % 12);
        Ok(Tune {
            title: self.title.unwrap_or_default(),
            notes: self.notes,
            length,
            tempos: starting_at_0(self.tempos, |tempo| tempo.0, (0, 120.0)),
            signatures: starting_at_0(self.signatures, |s| s.at, Signature::COMMON),
            final_class,
        })
    }
}

/// In order, with one at tick 0 (the default if the file gave none there).
fn starting_at_0<T>(mut list: Vec<T>, at: impl Fn(&T) -> u64, default: T) -> Vec<T> {
    list.sort_by_key(&at);
    if list.first().is_none_or(|first| at(first) > 0) {
        list.insert(0, default);
    }
    list
}

/// The final's pitch class from "mode: Dorian on G" (a sharp or flat after
/// the letter counts).
pub(super) fn final_of(text: &str) -> Option<u8> {
    let rest = text.trim().strip_prefix("mode:")?;
    let after = &rest[rest.rfind(" on ")? + 4..];
    let mut letters = after.trim().chars();
    let base = match letters.next()?.to_ascii_uppercase() {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };
    let shift = match letters.next() {
        Some('#') => 1,
        Some('b') => 11,
        _ => 0,
    };
    Some((base + shift) % 12)
}
