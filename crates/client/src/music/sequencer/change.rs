//! Changing program (music by place) at the next bar line: everything the
//! old program had coming from that bar is taken out, notes still sounding
//! are let go on it, the new players take up their instruments, and a link
//! (`link`) leads into the new program's first tune, as between tunes.

use super::super::stream::MusicEvent;
use super::super::tune::{Signature, Tune};
use super::due::Due;
use super::{DRONE, MELODY, Program, SECOND, Sequencer, link};

/// Swaps in `program` at the next bar line of what plays.
pub(super) fn change(sequencer: &mut Sequencer, program: Program) {
    let now = sequencer.tick_now();
    let Some(playing) = &sequencer.playing else {
        return;
    };
    let Some(first) = program.tunes.first().cloned() else {
        return;
    };
    let tune = playing.program.tunes[playing.tune].clone();
    let (at, signature) = next_bar(&tune, now.saturating_sub(playing.pass_start));
    let cut = playing.pass_start + at;
    let mut stopped_drone = Vec::new();
    for due in sequencer.queue.cut_from(cut) {
        match due {
            Due::Midi(event) if event.command == 0x80 => sequencer.queue.push(cut, due),
            Due::Midi(event) if event.command == 0x90 && event.channel == DRONE => {
                stopped_drone.push(event.data1);
            }
            _ => {}
        }
    }
    for (channel, instrument) in [
        (MELODY, Some(program.players.melody)),
        (SECOND, program.players.second),
        (DRONE, program.players.drone),
    ] {
        if let Some(instrument) = instrument {
            let bank = MusicEvent::bank(channel, instrument.bank);
            sequencer.queue.push(cut, Due::Midi(bank));
            let chosen = MusicEvent::program(channel, instrument.program);
            sequencer.queue.push(cut, Due::Midi(chosen));
        }
    }
    let Some(playing) = &mut sequencer.playing else {
        return;
    };
    // Drone notes taken out before they began aren't sounding.
    playing.drone.retain(|key| !stopped_drone.contains(key));
    if program.players.drone.is_none() {
        for key in std::mem::take(&mut playing.drone) {
            let off = MusicEvent::note_off(DRONE, key);
            sequencer.queue.push(cut, Due::Midi(off));
        }
    }
    // The link's end moves on to the tune after this one: the first.
    playing.tune = program.tunes.len() - 1;
    playing.loops = 0;
    playing.program = program;
    link::link_into(sequencer, cut, signature, first);
}

/// The next bar line at or after `at` in a pass of `tune` (its end counts),
/// as ticks from the pass's start, and the signature its bars are in. Past
/// the end (in a link), bars of its last signature go on from the end.
pub(super) fn next_bar(tune: &Tune, at: u64) -> (u64, Signature) {
    if at < tune.length {
        let signature = tune.signature_at(at);
        let bar = signature.bar();
        let next = signature.at + (at - signature.at).div_ceil(bar) * bar;
        return (next.min(tune.length), signature);
    }
    let signature = tune.last_signature();
    let bar = signature.bar();
    (
        tune.length + (at - tune.length).div_ceil(bar) * bar,
        signature,
    )
}
