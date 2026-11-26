use std::sync::Arc;

use super::pass::drum_level;
use super::*;
use crate::music::playlist::{Instrument, Percussion};
use crate::music::tune::{Note, Signature};

pub(super) const RATE: u32 = 44_100;

/// A tune of quarter notes in 4/4, its final D.
pub(super) fn tune(keys: &[u8], tempo: f64) -> Arc<Tune> {
    let notes = keys
        .iter()
        .enumerate()
        .map(|(at, &key)| Note {
            at: at as u64 * PPQ,
            length: PPQ,
            key,
            velocity: 80,
        })
        .collect();
    Arc::new(Tune {
        title: "Test".into(),
        notes,
        length: keys.len() as u64 * PPQ,
        tempos: vec![(0, tempo)],
        signatures: vec![Signature::COMMON],
        final_class: 2,
    })
}

pub(super) fn players() -> Players {
    let instrument = Instrument {
        bank: 0,
        program: 74,
    };
    Players {
        melody: instrument,
        second: Some(instrument),
        drone: Some(instrument),
        percussion: Some(Percussion {
            strong: 61,
            light: 60,
        }),
        link_seconds: 4.0,
    }
}

/// Runs the sequencer for `seconds`, the way the stream does, giving each
/// event with the sample it fell on.
pub(super) fn run(sequencer: &mut Sequencer, seconds: f64) -> Vec<(usize, MusicEvent)> {
    let end = (seconds * f64::from(RATE)) as usize;
    let (mut now, mut heard, mut out) = (0, Vec::new(), Vec::new());
    loop {
        let samples = sequencer.samples_to_next();
        if samples == usize::MAX || now + samples > end {
            return heard;
        }
        now += samples;
        sequencer.advance(samples, &mut out);
        heard.extend(out.drain(..).map(|event| (now, event)));
    }
}

pub(super) fn started(sequencer: &mut Sequencer, tunes: Vec<Arc<Tune>>, players: Players) {
    let mut out = Vec::new();
    sequencer.play(Program { tunes, players }, &mut out);
    assert!(out.contains(&MusicEvent::program(MELODY, 74)));
}

pub(super) fn melody_ons(heard: &[(usize, MusicEvent)]) -> Vec<(usize, u8, u8)> {
    heard
        .iter()
        .filter(|(_, e)| e.channel == MELODY && e.command == 0x90)
        .map(|(at, e)| (*at, e.data1, e.data2))
        .collect()
}

#[test]
fn notes_land_on_their_samples() {
    let mut sequencer = Sequencer::new(RATE);
    let alone = Players {
        second: None,
        drone: None,
        percussion: None,
        ..players()
    };
    started(&mut sequencer, vec![tune(&[62, 64, 65, 67], 120.0)], alone);
    let heard = run(&mut sequencer, 1.9);
    let at: Vec<_> = melody_ons(&heard).iter().map(|on| on.0).collect();
    // At 120 a minute a quarter is half a second: 22,050 samples.
    assert_eq!(at, [0, 22_050, 44_100, 66_150]);
}

#[test]
fn one_tune_goes_round_for_good() {
    let mut sequencer = Sequencer::new(RATE);
    started(
        &mut sequencer,
        vec![tune(&[62, 64, 65, 67], 120.0)],
        players(),
    );
    let ons = melody_ons(&run(&mut sequencer, 2.0 * 5.0 - 0.01));
    assert_eq!(ons.len(), 4 * 5, "five times round, no links");
}

#[test]
fn two_tunes_are_linked_after_twice_round() {
    let mut sequencer = Sequencer::new(RATE);
    let (a, b) = (
        tune(&[62, 64, 65, 67], 120.0),
        tune(&[69, 67, 65, 64], 90.0),
    );
    let b = Arc::new(Tune {
        final_class: 9,
        ..(*b).clone()
    });
    started(&mut sequencer, vec![a, b], players());
    let heard = run(&mut sequencer, 12.0);
    let ons = melody_ons(&heard);
    // Twice round A (8 notes), then the bridging note on B's final, A (69).
    assert!(ons[..8].iter().all(|on| on.2 == 80));
    let bridge = ons[8];
    assert_eq!((bridge.1, bridge.2), (69, 45));
    assert!(bridge.0 > 4 * 22_050 * 2, "after the second time round");
    // The drone moves to A and E.
    let drone: Vec<_> = heard
        .iter()
        .filter(|(_, e)| e.channel == DRONE && e.command == 0x90)
        .map(|(_, e)| e.data1)
        .collect();
    assert_eq!(drone, [50, 57, 57, 64]);
    // B starts once the tempo has glided to its own.
    assert_eq!(ons[9].1, 69);
    assert!((sequencer.tempo() - 90.0).abs() < 1e-9);
}

#[test]
fn the_other_parts_come_back_in_after_a_link() {
    let mut sequencer = Sequencer::new(RATE);
    let (a, b) = (tune(&[62, 64, 65, 67], 120.0), tune(&[62; 16], 120.0));
    started(&mut sequencer, vec![a, b], players());
    let heard = run(&mut sequencer, 30.0);
    let second: Vec<u8> = heard
        .iter()
        .filter(|(_, e)| e.channel == SECOND && e.command == 0x90)
        .map(|(_, e)| e.data2)
        .collect();
    // A at full twice, then B growing over two bars (8 quarters) to full.
    assert!(second[..8].iter().all(|&velocity| velocity == 56));
    assert!(second[8] < second[11] && second[11] < second[15]);
    assert_eq!(second[15], 56);
}

#[test]
fn stopping_lets_every_note_go() {
    let mut sequencer = Sequencer::new(RATE);
    started(&mut sequencer, vec![tune(&[62, 64], 120.0)], players());
    run(&mut sequencer, 0.7);
    let mut out = Vec::new();
    sequencer.stop(&mut out);
    assert_eq!(out.len(), CHANNELS.len());
    assert!(out.iter().all(|e| e.command == 0xB0 && e.data1 == 123));
    assert_eq!(sequencer.samples_to_next(), usize::MAX);
}

#[test]
fn every_note_started_is_let_go() {
    let mut sequencer = Sequencer::new(RATE);
    let tunes = vec![tune(&[62, 64, 65, 67], 120.0), tune(&[69, 67], 100.0)];
    started(&mut sequencer, tunes, players());
    let heard = run(&mut sequencer, 60.0);
    for channel in [MELODY, SECOND, DRUMS] {
        let count = |command| {
            let of = |e: &&(usize, MusicEvent)| e.1.channel == channel && e.1.command == command;
            heard.iter().filter(of).count() as i64
        };
        assert!((count(0x90) - count(0x80)).abs() <= 2, "channel {channel}");
    }
}

#[test]
fn drums_follow_the_first_rhythmic_mode() {
    let six_eight = Signature {
        at: 0,
        numerator: 6,
        denominator: 8,
    };
    let levels: Vec<f32> = (0..6).map(|unit| drum_level(&six_eight, unit)).collect();
    assert_eq!(levels, [1.0, 0.0, 0.45, 0.7, 0.0, 0.45]);
    let three_four = Signature {
        numerator: 3,
        ..Signature::COMMON
    };
    let levels: Vec<f32> = (0..3).map(|unit| drum_level(&three_four, unit)).collect();
    assert_eq!(levels, [1.0, 0.0, 0.35]);
}
