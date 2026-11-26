use std::sync::Arc;

use super::change::next_bar;
use super::tests::{RATE, melody_ons, players, run, started, tune};
use super::*;
use crate::music::playlist::Instrument;
use crate::music::tune::Signature;

/// Two bars of D quarters, then a tune on G (its final G) for the new place.
fn old_and_new() -> (Arc<Tune>, Arc<Tune>) {
    let old = tune(&[62, 64, 65, 67, 69, 67, 65, 64], 120.0);
    let mut new = (*tune(&[67, 71, 74, 71, 67, 72, 71, 67], 120.0)).clone();
    new.final_class = 7;
    (old, Arc::new(new))
}

/// The sample a tick falls on at 120 quarters a minute from the start.
fn sample_at(tick: u64) -> usize {
    (tick as f64 * f64::from(RATE) * 0.5 / PPQ as f64) as usize
}

#[test]
fn bars_are_found_from_the_signature_and_the_end() {
    let (old, _) = old_and_new();
    let bar = Signature::COMMON.bar();
    assert_eq!(next_bar(&old, 0).0, 0);
    assert_eq!(next_bar(&old, 1).0, bar);
    assert_eq!(next_bar(&old, bar + 5).0, 2 * bar);
    // In a link past the end, bars go on from the end.
    assert_eq!(next_bar(&old, 2 * bar + 1).0, 3 * bar);
}

#[test]
fn a_new_place_comes_in_at_the_next_bar_line() {
    let (old, new) = old_and_new();
    let mut sequencer = Sequencer::new(RATE);
    started(&mut sequencer, vec![old], players());
    let before = run(&mut sequencer, 0.5);
    assert!(!melody_ons(&before).is_empty());
    let mut organ = players();
    organ.melody = Instrument {
        bank: 0,
        program: 19,
    };
    let mut out = Vec::new();
    sequencer.change(
        Program {
            tunes: vec![new],
            players: organ,
        },
        &mut out,
    );
    assert!(out.is_empty(), "nothing at once: it waits for the bar");
    let heard = run(&mut sequencer, 12.0);
    let bar = sample_at(Signature::COMMON.bar());
    let ons = melody_ons(&heard);
    // The old tune's notes stop at the bar line (the first run ended on the
    // note half a second in, so the bar line is 1.5 s on), apart from the
    // link's bridging note.
    let first_bar = bar - (0.5 * f64::from(RATE)) as usize;
    let old_after: Vec<_> = ons
        .iter()
        .filter(|&&(at, key, _)| at > first_bar + 10 && key != 67)
        .filter(|&&(_, key, _)| [62, 64, 65, 69].contains(&key))
        .collect();
    assert!(
        old_after.is_empty(),
        "old notes after the bar: {old_after:?}"
    );
    // The new players take up their instruments on the bar line.
    let organ_at = heard
        .iter()
        .find(|(_, e)| *e == MusicEvent::program(MELODY, 19))
        .map(|(at, _)| *at);
    assert_eq!(organ_at, Some(first_bar));
    // The new tune's notes come after a link of at least a bar.
    assert!(
        ons.iter()
            .any(|&(at, key, _)| key == 71 && at > first_bar + bar)
    );
    // And the drone moves to G.
    let drone_g = heard
        .iter()
        .any(|(_, e)| e.channel == DRONE && e.command == 0x90 && e.data1 == 55);
    assert!(drone_g);
}

#[test]
fn a_new_place_without_a_drone_lets_the_old_one_go() {
    let (old, new) = old_and_new();
    let mut sequencer = Sequencer::new(RATE);
    started(&mut sequencer, vec![old], players());
    run(&mut sequencer, 0.5);
    let mut quiet = players();
    quiet.drone = None;
    let program = Program {
        tunes: vec![new],
        players: quiet,
    };
    sequencer.change(program, &mut Vec::new());
    let heard = run(&mut sequencer, 4.0);
    let offs = |key: u8| {
        heard
            .iter()
            .any(|(_, e)| e.channel == DRONE && e.command == 0x80 && e.data1 == key)
    };
    assert!(offs(50) && offs(57), "the D drone let go");
    let ons = heard
        .iter()
        .any(|(_, e)| e.channel == DRONE && e.command == 0x90);
    assert!(!ons, "no new drone");
}

#[test]
fn changing_with_nothing_playing_starts_at_once() {
    let (_, new) = old_and_new();
    let mut sequencer = Sequencer::new(RATE);
    let mut out = Vec::new();
    let program = Program {
        tunes: vec![new],
        players: players(),
    };
    sequencer.change(program, &mut out);
    assert!(out.contains(&MusicEvent::program(MELODY, 74)));
    let heard = run(&mut sequencer, 0.1);
    assert_eq!(melody_ons(&heard).first().map(|on| on.1), Some(67));
}
