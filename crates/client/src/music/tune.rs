//! A tune as the sequencer plays it: one melody's notes in ticks of a fixed
//! length (`PPQ` a quarter, whatever the file used), its tempo changes, time
//! signatures (which may change in the middle: Stella splendens does), its
//! final, and where it goes round again. Read from a MIDI file by `midi`.

/// Ticks a quarter note, for every tune.
pub(super) const PPQ: u64 = 960;

/// One note of the melody.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Note {
    pub at: u64,
    pub length: u64,
    pub key: u8,
    pub velocity: u8,
}

/// A time signature from its tick on: `numerator` units of a
/// `denominator` note to the bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Signature {
    pub at: u64,
    pub numerator: u8,
    pub denominator: u8,
}

impl Signature {
    pub const COMMON: Self = Self {
        at: 0,
        numerator: 4,
        denominator: 4,
    };

    /// One unit: the note the denominator names (an eighth in 6/8).
    pub fn unit(&self) -> u64 {
        PPQ * 4 / u64::from(self.denominator.max(1))
    }

    pub fn bar(&self) -> u64 {
        self.unit() * u64::from(self.numerator.max(1))
    }

    /// Units a beat: three in compound metres (6/8, 9/8, 12/8), else one.
    pub fn beat_units(&self) -> u64 {
        match self.numerator.is_multiple_of(3) && self.numerator > 3 && self.denominator >= 8 {
            true => 3,
            false => 1,
        }
    }

    pub fn beat(&self) -> u64 {
        self.unit() * self.beat_units()
    }

    /// Where `at` falls in its bar, in ticks (bars counted from the
    /// signature's own tick, as signatures change at bar lines).
    pub fn in_bar(&self, at: u64) -> u64 {
        at.saturating_sub(self.at) % self.bar()
    }
}

/// A tune, ready to play.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Tune {
    pub title: String,
    /// In order of their start.
    pub notes: Vec<Note>,
    /// One time round: the "loop" marker, else the end of the music.
    pub length: u64,
    /// Tempo changes as (tick, quarters a minute), the first at 0.
    pub tempos: Vec<(u64, f64)>,
    /// The first at 0.
    pub signatures: Vec<Signature>,
    /// The mode's final as a pitch class (0 = C), from the "mode: ... on X"
    /// text; else the last note's.
    pub final_class: u8,
}

impl Tune {
    /// The tempo it starts at, in quarters a minute.
    pub fn tempo(&self) -> f64 {
        self.tempos.first().map_or(120.0, |&(_, qpm)| qpm)
    }

    pub fn signature_at(&self, at: u64) -> Signature {
        self.signatures
            .iter()
            .rev()
            .find(|signature| signature.at <= at)
            .copied()
            .unwrap_or(Signature::COMMON)
    }

    /// The signature it ends in, which a link into the next tune keeps.
    pub fn last_signature(&self) -> Signature {
        self.signatures.last().copied().unwrap_or(Signature::COMMON)
    }

    /// The final in the melody's own range: the octave nearest its first note.
    pub fn final_near_start(&self) -> u8 {
        let first = self.notes.first().map_or(60, |note| note.key);
        nearest_of_class(self.final_class, first)
    }
}

/// The key of this pitch class nearest to `near`.
pub(super) fn nearest_of_class(class: u8, near: u8) -> u8 {
    let near = i32::from(near);
    let below = near - (near - i32::from(class % 12)).rem_euclid(12);
    let key = match near - below > 6 {
        true => below + 12,
        false => below,
    };
    key.clamp(0, 127) as u8
}

#[cfg(test)]
mod tests;
