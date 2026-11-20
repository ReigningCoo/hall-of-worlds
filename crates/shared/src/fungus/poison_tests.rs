//! Poisons eaten (fungi plan PR 6): onsets, deadly drains, false recoveries,
//! the ink cap's window and alcohol, recognising them.

use super::tests::EATER;
use super::*;
use crate::bonus::{self, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::sheet::Attribute;
use crate::vitals::Vital;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

fn def(key: &str) -> &'static FungusDef {
    Fungi::built_in().get(key).expect("a built-in fungus")
}

fn poisons(effects: &[bonus::Effect]) -> Vec<&bonus::Effect> {
    (effects.iter())
        .filter(|e| matches!(e.source, Source::FungusPoison(_)))
        .collect()
}

#[test]
fn onsets_are_in_game_time() {
    assert_eq!(onset_ticks(Onset::Soon), 0);
    assert_eq!(onset_ticks(Onset::Delayed), 10 * GAME_MINUTE);
    assert_eq!(onset_ticks(Onset::Late), 60 * GAME_MINUTE);
    // Fly agaric comes on at once; sulphur tuft about 10 game minutes later.
    let fly = eating(def("fly_agaric"), 5, EATER);
    assert_eq!(poisons(&fly)[0].start, 5);
    let tuft = eating(def("sulphur_tuft"), 5, EATER);
    let tuft = poisons(&tuft)[0];
    assert_eq!(tuft.start, 5 + 10 * GAME_MINUTE);
    assert_eq!(
        tuft.end,
        Some(tuft.start + poison_ticks(Severity::Moderate))
    );
    assert_eq!(tuft.ends_on, EndsOn::NOTHING);
}

#[test]
fn a_deadly_poison_drains_to_death() {
    let effects = eating(def("deadly_webcap"), 0, EATER);
    let worst = poisons(&effects);
    assert_eq!(worst.len(), 1, "no false recovery");
    let deadly = bonus::Stat::DeadlyDrain(Vital::Health);
    let drain = (worst[0].bonuses.iter())
        .find(|b| b.stat == deadly)
        .expect("a deadly drain");
    let ticks = poison_ticks(Severity::Deadly);
    // Three times 20 Health over 120 s: 0.5 a second.
    assert_eq!(drain.amount, bonus::Amount::Flat(500));
    assert_eq!(deadly_drain(EATER, ticks), 500);
    // Its Health effect became the deadly drain; its Toughness stays.
    let plain = bonus::Stat::Drain(Vital::Health);
    assert!(worst[0].bonuses.iter().all(|b| b.stat != plain));
    let toughness = bonus::Stat::Attribute(Attribute::Toughness);
    assert!(worst[0].bonuses.iter().any(|b| b.stat == toughness));
    // Easy halves it, still killing in two thirds of its time.
    let easy = Eater {
        easy: true,
        ..EATER
    };
    assert_eq!(deadly_drain(easy, ticks), 250);
    assert!(u64::from(deadly_drain(easy, ticks)) * ticks / SECOND > 20_000);
}

#[test]
fn the_death_cap_eases_before_the_worst() {
    let effects = eating(def("death_cap"), 10, EATER);
    let phases = poisons(&effects);
    assert_eq!(phases.len(), 2);
    let (sick, worst) = (phases[0], phases[1]);
    assert_eq!(sick.start, 10 + 60 * GAME_MINUTE);
    assert_eq!(sick.end, Some(sick.start + SICK_TICKS));
    let deadly = bonus::Stat::DeadlyDrain(Vital::Health);
    assert!(sick.bonuses.iter().all(|b| b.stat != deadly), "not yet");
    assert_eq!(worst.start, sick.start + SICK_TICKS + EASED_TICKS);
    assert!(worst.bonuses.iter().any(|b| b.stat == deadly));
}

#[test]
fn the_ink_cap_gives_a_window_alcohol_turns_to_sickness() {
    let ink_cap = def("common_ink_cap");
    let window = eating(ink_cap, 0, EATER);
    assert_eq!(window[0].end, Some(WINDOW_TICKS));
    assert_eq!(WINDOW_TICKS, 3 * 24 * 60 * GAME_MINUTE);
    let will = bonus::Stat::Attribute(Attribute::Will);
    assert!(window[0].bonuses.iter().any(|b| b.stat == will));
    let on = bonus::Effects(window);
    let sick = with_alcohol(&on, 100, EATER);
    assert_eq!(sick.len(), 1);
    assert_eq!(
        sick[0].source,
        Source::FungusPoison(FungusTag::of("common_ink_cap"))
    );
    assert_eq!(sick[0].start, 100, "at once");
    assert!(with_alcohol(&on, WINDOW_TICKS, EATER).is_empty(), "over");
}

#[test]
fn confusing_poisons_are_the_three() {
    let confusing: Vec<&str> = (Fungi::built_in().iter())
        .filter(|def| def.poison.as_ref().is_some_and(|p| p.confuses))
        .map(|def| def.key.as_str())
        .collect();
    assert_eq!(confusing, ["fly_agaric", "panther_cap", "dreamcap"]);
}

#[test]
fn a_poison_is_recognised_if_known_or_by_healing() {
    let def = def("death_cap");
    let mut knows = FungusKnowledge::default();
    assert!(!recognised(&knows, def, 0, 0));
    assert!(recognised(&knows, def, 30, 29));
    assert!(!recognised(&knows, def, 30, 30));
    assert!(!recognised(&knows, def, 500, 95), "never certain");
    assert!(knows.learn_poison("death_cap"));
    assert!(recognised(&knows, def, 0, 99));
}
