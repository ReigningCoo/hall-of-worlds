//! Combat music, as the owner approved it in the model: when the player is
//! attacked, on the next beat a stinger (a drum roll through the beat, then a
//! horn, trumpet or bell, or the drum, on the next), the tempo a fifth faster
//! over a bar, a drum ostinato in the first rhythmic mode with fills at
//! phrase ends, and the drone louder, the key unchanged. After `HOLD` with no
//! hostile action, at a bar line, two bars easing back while the combat
//! parts fade. Attacked again while easing off, or within `RECENT` of calm:
//! straight back in, no stinger. Places whose players have no combat parts
//! (The Way Inn, the world map) never fight.

use super::super::playlist::Combat;
use super::super::stream::MusicEvent;
use super::super::tune::{Signature, Tune};
use super::due::Due;
use super::{DRONE, DRUMS, STINGER, Sequencer};

/// How much faster the music plays in combat.
pub(super) const RISE: f64 = 1.2;
/// Seconds with no hostile action before the music eases off.
pub(super) const HOLD: f64 = 10.0;
/// Seconds after calming down within which a new fight has no stinger.
pub(super) const RECENT: f64 = 30.0;
/// Bars the easing off takes.
const EASING_BARS: u32 = 2;
/// Bars a phrase, the last ending in fills.
const PHRASE_BARS: u32 = 4;
/// The drone's channel volume, calm and in combat.
const DRONE_CALM: f64 = 100.0;
const DRONE_LOUD: f64 = 127.0;
/// The ostinato's downbeat.
const DRUM_VELOCITY: f32 = 110.0;

/// Where a fight is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum State {
    #[default]
    Calm,
    Fighting,
    /// Bars of easing off still to come.
    Easing(u32),
}

/// The combat parts' state, kept by the sequencer.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Fight {
    pub state: State,
    /// Numbers each tempo ramp, so steps of one replaced are dropped.
    ramp: u32,
    /// Numbers each run of combat parts, so a stale bar marker is dropped.
    run: u32,
    /// The sample of the last hostile action.
    last_hit: f64,
    /// The sample the last fight ended on.
    calm_at: Option<f64>,
    /// Whole bars played in this fight, for the phrases' fills.
    bars: u32,
}

/// The player was attacked.
pub(super) fn attacked(sequencer: &mut Sequencer) {
    let Some(combat) = combat_of(sequencer) else {
        return;
    };
    let seconds = |samples: f64| samples / sequencer.sample_rate;
    sequencer.fight.last_hit = sequencer.now;
    let now = sequencer.tick_now();
    let (tune, at) = position(sequencer, now);
    let signature = signature_of(&tune, at);
    let beat = start_of(sequencer, &tune, at, signature.beat());
    match sequencer.fight.state {
        State::Fighting => {}
        State::Easing(_) => {
            // Full again from the next bar line, where the parts' next bar is due.
            sequencer.fight.state = State::Fighting;
            ramp(sequencer, beat, RISE, signature.bar(), signature.unit());
        }
        State::Calm => {
            let recent = sequencer
                .fight
                .calm_at
                .is_some_and(|calm| seconds(sequencer.now - calm) < RECENT);
            sequencer.fight.state = State::Fighting;
            let number = new_run(sequencer);
            sequencer.fight.bars = 0;
            ramp(sequencer, beat, RISE, signature.bar(), signature.unit());
            let parts = match recent {
                true => beat,
                false => {
                    stinger(sequencer, combat, &tune, beat, signature.beat());
                    beat + signature.beat()
                }
            };
            sequencer.queue.push(parts, Due::FightBar { fight: number });
        }
    }
}

/// The combat parts from `tick` to the next bar line: the ostinato at its
/// level, easing off when it is time, and the next bar's marker.
pub(super) fn bar(sequencer: &mut Sequencer, fight: u32) {
    if fight != sequencer.fight.run || sequencer.fight.state == State::Calm {
        return;
    }
    let Some(combat) = combat_of(sequencer) else {
        return;
    };
    let tick = sequencer.tick;
    let (tune, at) = position(sequencer, tick);
    let signature = signature_of(&tune, at);
    let end = start_of(sequencer, &tune, at + 1, signature.bar());
    let on_bar_line = start_of(sequencer, &tune, at, signature.bar()) == tick;
    let bar_start = bar_before(&tune, at, signature.bar());
    let quiet = (sequencer.now - sequencer.fight.last_hit) / sequencer.sample_rate;
    if on_bar_line && sequencer.fight.state == State::Fighting && quiet > HOLD {
        sequencer.fight.state = State::Easing(EASING_BARS);
        let over = u64::from(EASING_BARS) * signature.bar();
        ramp(sequencer, tick, 1.0, over, signature.unit());
    }
    let level = match sequencer.fight.state {
        State::Easing(0) | State::Calm => {
            sequencer.fight.state = State::Calm;
            sequencer.fight.calm_at = Some(sequencer.now);
            return;
        }
        State::Easing(left) => left as f32 / (EASING_BARS + 1) as f32,
        State::Fighting => 1.0,
    };
    let phrase_end = sequencer.fight.bars % PHRASE_BARS == PHRASE_BARS - 1;
    let mut unit_at = tick;
    while unit_at < end {
        let in_bar = (at + unit_at - tick).saturating_sub(bar_start) % signature.bar();
        let share = ostinato(&signature, in_bar, phrase_end);
        let velocity = scaled(DRUM_VELOCITY * share * level);
        sequencer.note(DRUMS, combat.drum, velocity, unit_at, signature.unit() / 2);
        if phrase_end && signature.beat_units() == 1 && in_bar != 0 {
            let fill = scaled(DRUM_VELOCITY * 0.4 * level);
            let half = signature.unit() / 2;
            sequencer.note(DRUMS, combat.drum, fill, unit_at + half, half / 2);
        }
        unit_at += signature.unit();
    }
    if on_bar_line {
        sequencer.fight.bars += 1;
        if let State::Easing(left) = &mut sequencer.fight.state {
            *left -= 1;
        }
    }
    let run = sequencer.fight.run;
    sequencer.queue.push(end, Due::FightBar { fight: run });
}

/// Combat's share of the tempo changes (with the drone's loudness), unless
/// the ramp belongs to a fight since replaced.
pub(super) fn set_factor(
    sequencer: &mut Sequencer,
    factor: f64,
    fight: u32,
    out: &mut Vec<MusicEvent>,
) {
    if fight != sequencer.fight.ramp {
        return;
    }
    sequencer.factor = factor;
    let share = ((factor - 1.0) / (RISE - 1.0)).clamp(0.0, 1.0);
    let volume = DRONE_CALM + (DRONE_LOUD - DRONE_CALM) * share;
    out.push(volume_of(DRONE, volume.round() as u8));
}

/// A change of place: the fight goes on with the new players' combat parts
/// from `cut`, or (with none) the music calms down over a bar.
pub(super) fn changed(sequencer: &mut Sequencer, cut: u64, signature: Signature) {
    if sequencer.fight.state == State::Calm && sequencer.factor == 1.0 {
        return;
    }
    let fighting = sequencer.fight.state != State::Calm;
    match combat_of(sequencer) {
        Some(_) if fighting => {
            let run = new_run(sequencer);
            sequencer.queue.push(cut, Due::FightBar { fight: run });
        }
        _ => {
            sequencer.fight.state = State::Calm;
            sequencer.fight.calm_at = Some(sequencer.now);
            ramp(sequencer, cut, 1.0, signature.bar(), signature.unit());
        }
    }
}

/// A new run of combat parts: bar markers queued before are dropped.
fn new_run(sequencer: &mut Sequencer) -> u32 {
    sequencer.fight.run = sequencer.fight.run.wrapping_add(1);
    sequencer.fight.run
}

/// Combat's share of the tempo moving to `to` a unit at a time over `over`
/// ticks from `from`, replacing any ramp still under way.
fn ramp(sequencer: &mut Sequencer, from: u64, to: f64, over: u64, unit: u64) {
    sequencer.fight.ramp = sequencer.fight.ramp.wrapping_add(1);
    let fight = sequencer.fight.ramp;
    let start = sequencer.factor;
    let units = (over / unit.max(1)).max(1);
    for step in 1..=units {
        let factor = start + (to - start) * step as f64 / units as f64;
        let at = from + (step - 1) * unit;
        sequencer.queue.push(at, Due::Factor { factor, fight });
    }
}

/// A roll through the beat at `at`, then the accent on the next.
fn stinger(sequencer: &mut Sequencer, combat: Combat, tune: &Tune, at: u64, beat: u64) {
    for hit in 0..6u64 {
        let velocity = scaled(DRUM_VELOCITY * (0.3 + 0.12 * hit as f32));
        sequencer.note(DRUMS, combat.drum, velocity, at + hit * beat / 6, beat / 12);
    }
    let accent = at + beat;
    match combat.stinger {
        Some(stinger) => {
            let channel_bank = MusicEvent::bank(STINGER, stinger.instrument.bank);
            sequencer.queue.push(accent, Due::Midi(channel_bank));
            let program = MusicEvent::program(STINGER, stinger.instrument.program);
            sequencer.queue.push(accent, Due::Midi(program));
            let key = 60 + tune.final_class % 12 + stinger.above;
            sequencer.note(STINGER, key.min(127), 115, accent, beat * 3 / 2);
        }
        None => sequencer.note(DRUMS, combat.drum, 127, accent, beat / 2),
    }
}

/// The ostinato's share of full loudness on a tick of the bar: the
/// downbeat, lighter beats, and the units between (long, short) lighter
/// still, filled at phrase ends.
pub(super) fn ostinato(signature: &Signature, in_bar: u64, phrase_end: bool) -> f32 {
    match (in_bar, in_bar.is_multiple_of(signature.beat())) {
        (0, _) => 1.0,
        (_, true) => 0.6,
        _ if phrase_end => 0.55,
        _ => 0.3,
    }
}

fn combat_of(sequencer: &Sequencer) -> Option<Combat> {
    sequencer.playing.as_ref()?.program.players.combat
}

/// The tune at `tick` (the one playing, or linked from) and how far into
/// its pass `tick` is.
fn position(sequencer: &Sequencer, tick: u64) -> (std::sync::Arc<Tune>, u64) {
    let playing = sequencer.playing.as_ref().expect("combat while playing");
    let tune = playing.program.tunes[playing.tune].clone();
    (tune, tick.saturating_sub(playing.pass_start))
}

fn signature_of(tune: &Tune, at: u64) -> Signature {
    match at < tune.length {
        true => tune.signature_at(at),
        false => tune.last_signature(),
    }
}

/// The tick of the next multiple of `step` (a beat or bar) at or after `at`
/// in the pass, counted from the signature (or past the end, from the end).
fn start_of(sequencer: &Sequencer, tune: &Tune, at: u64, step: u64) -> u64 {
    let pass_start = sequencer.playing.as_ref().map_or(0, |p| p.pass_start);
    let from = match at < tune.length {
        true => tune.signature_at(at).at,
        false => tune.length,
    };
    let next = from + (at - from).div_ceil(step.max(1)) * step.max(1);
    let next = match at < tune.length {
        true => next.min(tune.length),
        false => next,
    };
    pass_start + next
}

/// The bar line at or before `at` in the pass, counted as `start_of` does.
fn bar_before(tune: &Tune, at: u64, bar: u64) -> u64 {
    let from = match at < tune.length {
        true => tune.signature_at(at).at,
        false => tune.length,
    };
    from + (at - from) / bar.max(1) * bar.max(1)
}

/// Control change 7: a channel's volume.
fn volume_of(channel: u8, volume: u8) -> MusicEvent {
    MusicEvent {
        channel,
        command: 0xB0,
        data1: 7,
        data2: volume.min(127),
    }
}

fn scaled(velocity: f32) -> u8 {
    velocity.round().clamp(0.0, 127.0) as u8
}
