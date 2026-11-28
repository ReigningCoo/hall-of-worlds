use super::*;
use crate::id::Id;
use crate::sheet::Attributes;
use crate::spot::Spot;

fn player() -> Id {
    Id::for_test(7)
}

#[test]
fn a_median_mind_reacts_in_a_second_and_a_half() {
    assert_eq!(reaction_ticks(&Attributes::all(100)), REACTION_TICKS);
    assert_eq!(REACTION_TICKS, 30);
}

#[test]
fn a_sharper_mind_reacts_sooner_within_bounds() {
    assert_eq!(reaction_ticks(&Attributes::all(150)), 20);
    assert_eq!(reaction_ticks(&Attributes::all(1000)), QUICKEST_TICKS);
    assert_eq!(reaction_ticks(&Attributes::all(10)), SLOWEST_TICKS);
    assert_eq!(reaction_ticks(&Attributes::all(0)), SLOWEST_TICKS);
}

#[test]
fn hearing_makes_it_alert_and_it_turns_after_its_reaction() {
    let mut wary = Wariness::default();
    assert!(wary.is_unaware());
    let at = Spot::middle_of((3, 4));
    wary.heard(at, Some(player()), 100, 30);
    assert_eq!(wary.stage(), Alertness::Alert);
    assert_eq!(wary.alert_to(), Some(player()));
    assert_eq!(wary.turn_due(129), None);
    // Still reacting: caught off guard.
    assert!(wary.off_guard());
    assert_eq!(wary.turn_due(130), Some(at));
    assert!(!wary.off_guard());
    assert_eq!(wary.turn_due(131), None);
}

#[test]
fn it_turns_to_the_first_noise_heard() {
    let mut wary = Wariness::default();
    let first = Spot::middle_of((1, 1));
    wary.heard(first, None, 100, 30);
    wary.heard(Spot::middle_of((9, 9)), None, 110, 30);
    assert_eq!(wary.turn_due(130), Some(first));
}

#[test]
fn seeing_someone_makes_it_aware_and_losing_them_alert() {
    let mut wary = Wariness::default();
    wary.heard(Spot::middle_of((1, 1)), None, 100, 30);
    wary.sees(Some(player()), 105);
    assert_eq!(wary.stage(), Alertness::Aware);
    assert_eq!(wary.alert_to(), None);
    assert_eq!(wary.about(), Some(player()));
    // No turn once it sees them.
    assert_eq!(wary.turn_due(200), None);
    // Noises change nothing while it sees someone.
    wary.heard(Spot::middle_of((5, 5)), None, 106, 30);
    assert_eq!(wary.stage(), Alertness::Aware);
    wary.sees(None, 107);
    assert_eq!(wary.stage(), Alertness::Alert);
    assert_eq!(wary.alert_to(), Some(player()));
}

#[test]
fn it_settles_back_to_unaware_after_a_quiet_while() {
    let mut wary = Wariness::default();
    wary.heard(Spot::middle_of((1, 1)), None, 100, 30);
    wary.sees(None, 130);
    assert!(wary.turn_due(130).is_some());
    wary.sees(None, 100 + ALERT_TICKS - 1);
    assert_eq!(wary.stage(), Alertness::Alert);
    wary.sees(None, 100 + ALERT_TICKS);
    assert!(wary.is_unaware());
    assert_eq!(wary.about(), None);
}
