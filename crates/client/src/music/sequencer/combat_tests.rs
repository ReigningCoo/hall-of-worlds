use super::combat::State;
use super::tests::{RATE, players, run, started, tune};
use super::*;

fn stinger_ons(heard: &[(usize, MusicEvent)]) -> Vec<u8> {
    heard
        .iter()
        .filter(|(_, e)| e.channel == STINGER && e.command == 0x90)
        .map(|(_, e)| e.data1)
        .collect()
}

fn combat_drums(heard: &[(usize, MusicEvent)]) -> usize {
    heard
        .iter()
        .filter(|(_, e)| e.channel == DRUMS && e.command == 0x90 && e.data1 == 45)
        .count()
}

/// A tune of one bar in 4/4 at 120 (a beat half a second), playing.
fn playing() -> Sequencer {
    let mut sequencer = Sequencer::new(RATE);
    started(
        &mut sequencer,
        vec![tune(&[62, 64, 65, 67], 120.0)],
        players(),
    );
    run(&mut sequencer, 0.75);
    sequencer
}

#[test]
fn being_attacked_brings_a_stinger_drums_a_faster_tempo_and_a_louder_drone() {
    let mut sequencer = playing();
    sequencer.attacked();
    let heard = run(&mut sequencer, 3.0);
    // The trumpet a fifth over the final, D: A.
    assert_eq!(stinger_ons(&heard), [69]);
    assert!(combat_drums(&heard) > 6, "a roll, then the ostinato");
    assert!((sequencer.factor() - 1.2).abs() < 1e-9);
    assert!(
        (sequencer.tempo() - 120.0).abs() < 1e-9,
        "the tune's own tempo stays"
    );
    let loud = MusicEvent {
        channel: DRONE,
        command: 0xB0,
        data1: 7,
        data2: 127,
    };
    assert!(heard.iter().any(|(_, e)| *e == loud));
    assert_eq!(sequencer.fight.state, State::Fighting);
}

#[test]
fn quiet_for_a_while_it_eases_back_and_goes_calm() {
    let mut sequencer = playing();
    sequencer.attacked();
    run(&mut sequencer, 16.0);
    assert_eq!(sequencer.fight.state, State::Calm);
    assert!((sequencer.factor() - 1.0).abs() < 1e-9);
    let later = run(&mut sequencer, 4.0);
    assert_eq!(combat_drums(&later), 0, "the combat parts have gone");
}

#[test]
fn soon_after_calming_down_a_fight_comes_back_with_no_stinger() {
    let mut sequencer = playing();
    sequencer.attacked();
    run(&mut sequencer, 16.0);
    sequencer.attacked();
    let heard = run(&mut sequencer, 3.0);
    assert!(stinger_ons(&heard).is_empty());
    assert!(combat_drums(&heard) > 0);
    assert_eq!(sequencer.fight.state, State::Fighting);
}

#[test]
fn attacked_while_easing_off_it_goes_straight_back_in() {
    let mut sequencer = playing();
    sequencer.attacked();
    run(&mut sequencer, 11.5);
    assert!(matches!(sequencer.fight.state, State::Easing(_)));
    sequencer.attacked();
    let heard = run(&mut sequencer, 3.0);
    assert!(stinger_ons(&heard).is_empty());
    assert_eq!(sequencer.fight.state, State::Fighting);
    assert!((sequencer.factor() - 1.2).abs() < 1e-9);
}

#[test]
fn players_with_no_combat_parts_never_fight() {
    let mut sequencer = Sequencer::new(RATE);
    let calm = Players {
        combat: None,
        ..players()
    };
    started(&mut sequencer, vec![tune(&[62, 64, 65, 67], 120.0)], calm);
    run(&mut sequencer, 0.75);
    sequencer.attacked();
    let heard = run(&mut sequencer, 3.0);
    assert!(stinger_ons(&heard).is_empty());
    assert_eq!(combat_drums(&heard), 0);
    assert_eq!(sequencer.fight.state, State::Calm);
}

#[test]
fn a_change_to_a_place_with_no_combat_parts_calms_down() {
    let mut sequencer = playing();
    sequencer.attacked();
    run(&mut sequencer, 2.0);
    let calm = Players {
        combat: None,
        ..players()
    };
    let program = Program {
        tunes: vec![tune(&[69, 67, 65, 64], 100.0)],
        players: calm,
        listed: Vec::new(),
    };
    sequencer.change(program, &mut Vec::new());
    run(&mut sequencer, 6.0);
    assert_eq!(sequencer.fight.state, State::Calm);
    assert!((sequencer.factor() - 1.0).abs() < 1e-9);
}

#[test]
fn the_ostinato_is_strong_on_the_downbeat_and_lighter_after() {
    let common = crate::music::tune::Signature::COMMON;
    let unit = common.unit();
    let shares: Vec<f32> = (0..4)
        .map(|n| combat::ostinato(&common, n * unit, false))
        .collect();
    assert_eq!(shares, [1.0, 0.6, 0.6, 0.6]);
    let compound = crate::music::tune::Signature {
        at: 0,
        numerator: 6,
        denominator: 8,
    };
    let unit = compound.unit();
    assert_eq!(combat::ostinato(&compound, unit, false), 0.3);
    assert_eq!(combat::ostinato(&compound, unit, true), 0.55);
    assert_eq!(combat::ostinato(&compound, 3 * unit, false), 0.6);
}

#[test]
fn attacked_again_and_again_as_forced_combat_does_it_never_eases_off() {
    let mut sequencer = playing();
    for _ in 0..40 {
        sequencer.attacked();
        run(&mut sequencer, 0.5);
    }
    assert_eq!(sequencer.fight.state, State::Fighting);
    // Let go, it eases back after the quiet hold.
    run(&mut sequencer, 16.0);
    assert_eq!(sequencer.fight.state, State::Calm);
}
