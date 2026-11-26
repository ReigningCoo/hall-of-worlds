//! The sequencer: plays tunes through the synthesizer from inside the audio
//! stream (`decoder`), so every note lands on its sample. It keeps a queue
//! of what is due at which tick and a clock turning ticks into samples at
//! the tempo of the moment; tempo glides change it a unit at a time.
//!
//! A program is tunes and players. Each tune is played twice (`LOOPS`), then
//! linked into the next over 1 to 3 bars: the tempo glides to the next
//! tune's, the drone moves to its final, the melody holds a bridging note on
//! it, and the second player and percussion come back in over the new
//! tune's first two bars. There are never silent rests (owner's decision).
//! A program of one tune plays it round for good. A new program (another
//! place's) comes in at the next bar line, linked from what played
//! (`change`).
//!
//! When the player is attacked (`attacked`), combat parts join on the next
//! beat over whatever plays (`combat`): a stinger, the tempo a fifth faster,
//! a drum ostinato and a louder drone, easing back after a quiet spell.

mod change;
mod combat;
mod due;
mod link;
mod pass;

use std::sync::Arc;

use due::{Due, Queue};

use super::playlist::Players;
use super::stream::MusicEvent;
use super::tune::{PPQ, Tune};

/// Times each tune goes round before the link into the next.
pub(super) const LOOPS: u32 = 2;

/// Channels: melody, second player, drone; drums on General MIDI's 10th.
const MELODY: u8 = 0;
const SECOND: u8 = 1;
const DRONE: u8 = 2;
const DRUMS: u8 = 9;
/// The combat stinger's instrument.
const STINGER: u8 = 3;
const CHANNELS: [u8; 5] = [MELODY, SECOND, DRONE, STINGER, DRUMS];

/// What to play.
#[derive(Clone, Debug)]
pub(super) struct Program {
    pub tunes: Vec<Arc<Tune>>,
    pub players: Players,
}

/// Where the program is.
struct Playing {
    program: Program,
    /// The tune playing (or being linked from), by its place in the program.
    tune: usize,
    /// Times it has gone round.
    loops: u32,
    /// The tick its present pass started on.
    pass_start: u64,
    /// The drone's keys sounding.
    drone: Vec<u8>,
}

pub(super) struct Sequencer {
    sample_rate: f64,
    queue: Queue,
    playing: Option<Playing>,
    /// The tick the clock was last at, and the sample it fell on.
    tick: u64,
    tick_sample: f64,
    /// Samples played since the program started.
    now: f64,
    /// Quarters a minute, as the tune and links set it.
    tempo: f64,
    /// Combat's share on top of that (1 when calm).
    factor: f64,
    fight: combat::Fight,
}

impl Sequencer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: f64::from(sample_rate),
            queue: Queue::default(),
            playing: None,
            tick: 0,
            tick_sample: 0.0,
            now: 0.0,
            tempo: 120.0,
            factor: 1.0,
            fight: combat::Fight::default(),
        }
    }

    /// Starts a program from its first tune, stopping what played.
    pub fn play(&mut self, program: Program, out: &mut Vec<MusicEvent>) {
        self.stop(out);
        let Some(first) = program.tunes.first().cloned() else {
            return;
        };
        (self.tick, self.tick_sample, self.now) = (0, 0.0, 0.0);
        self.tempo = first.tempo();
        self.factor = 1.0;
        self.fight = combat::Fight::default();
        for (channel, instrument) in [
            (MELODY, Some(program.players.melody)),
            (SECOND, program.players.second),
            (DRONE, program.players.drone),
        ] {
            if let Some(instrument) = instrument {
                out.push(MusicEvent::bank(channel, instrument.bank));
                out.push(MusicEvent::program(channel, instrument.program));
            }
        }
        self.playing = Some(Playing {
            program,
            tune: 0,
            loops: 0,
            pass_start: 0,
            drone: Vec::new(),
        });
        link::move_drone(self, 0, first.final_class);
        pass::start_pass(self, 0, false);
    }

    /// Plays another program, coming in at the next bar line with a link
    /// from what plays; at once if nothing does.
    pub fn change(&mut self, program: Program, out: &mut Vec<MusicEvent>) {
        match self.playing.is_some() {
            true => change::change(self, program),
            false => self.play(program, out),
        }
    }

    /// The player was attacked: combat parts join, or keep on.
    pub fn attacked(&mut self) {
        combat::attacked(self);
    }

    /// Stops at once, letting every note go.
    pub fn stop(&mut self, out: &mut Vec<MusicEvent>) {
        self.queue.clear();
        if self.playing.take().is_some() {
            for channel in CHANNELS {
                out.push(all_notes_off(channel));
            }
        }
    }

    /// Samples to play before the next event is due (`usize::MAX` with
    /// nothing playing).
    pub fn samples_to_next(&self) -> usize {
        match self.queue.next_tick() {
            Some(tick) => (self.sample_of(tick) - self.now).max(0.0).ceil() as usize,
            None => usize::MAX,
        }
    }

    /// Moves the clock on by `samples`, giving what fell due.
    pub fn advance(&mut self, samples: usize, out: &mut Vec<MusicEvent>) {
        self.now += samples as f64;
        while let Some(tick) = self.queue.next_tick() {
            let at = self.sample_of(tick);
            if at > self.now + 1e-6 {
                break;
            }
            (self.tick, self.tick_sample) = (tick, at);
            while let Some(due) = self.queue.pop_by(tick) {
                self.happen(due, out);
            }
        }
    }

    fn happen(&mut self, due: Due, out: &mut Vec<MusicEvent>) {
        match due {
            Due::Midi(event) => out.push(event),
            Due::Tempo(tempo) => self.tempo = tempo,
            Due::PassEnd => pass::end_pass(self),
            Due::LinkEnd => link::end_link(self),
            Due::Factor { factor, fight } => combat::set_factor(self, factor, fight, out),
            Due::FightBar { fight } => combat::bar(self, fight),
        }
    }

    /// The tick the clock is at now (between events), at the present tempo.
    fn tick_now(&self) -> u64 {
        let samples = (self.now - self.tick_sample).max(0.0);
        let ticks = samples * self.rate() * PPQ as f64 / (self.sample_rate * 60.0);
        self.tick + ticks.floor() as u64
    }

    /// The sample a tick falls on at the present tempo.
    fn sample_of(&self, tick: u64) -> f64 {
        let ticks = tick.saturating_sub(self.tick) as f64;
        self.tick_sample + ticks * self.sample_rate * 60.0 / (self.rate() * PPQ as f64)
    }

    /// The tempo played, combat's share and all.
    fn rate(&self) -> f64 {
        self.tempo * self.factor
    }

    fn note(&mut self, channel: u8, key: u8, velocity: u8, at: u64, length: u64) {
        if velocity == 0 || length == 0 {
            return;
        }
        let on = MusicEvent::note_on(channel, key, velocity);
        self.queue.push(at, Due::Midi(on));
        let off = MusicEvent::note_off(channel, key);
        self.queue.push(at + length, Due::Midi(off));
    }

    #[cfg(test)]
    fn tempo(&self) -> f64 {
        self.tempo
    }

    #[cfg(test)]
    fn factor(&self) -> f64 {
        self.factor
    }
}

/// Control change 123: every note on a channel let go.
fn all_notes_off(channel: u8) -> MusicEvent {
    MusicEvent {
        channel,
        command: 0xB0,
        data1: 123,
        data2: 0,
    }
}

#[cfg(test)]
mod change_tests;
#[cfg(test)]
mod combat_tests;
#[cfg(test)]
mod tests;
