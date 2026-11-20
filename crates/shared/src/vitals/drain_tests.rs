//! Draining (poison): taken all the while, never below 1 point, and told to a
//! client the same way.

use super::*;

const SECOND: u64 = TICKS;

fn w(points: u32) -> Points {
    Points::whole(points)
}

fn poisoned(drain: u32, until: Option<u64>) -> Limits {
    Limits {
        health: 100,
        mana: 20,
        refill: REFILL_TICKS,
        restore: Restore {
            drain_health: drain,
            until,
            ..Restore::default()
        },
    }
}

#[test]
fn draining_goes_on_while_waiting_to_refill() {
    let mut vitals = Vitals::default();
    let limits = poisoned(1000, None);
    vitals.hurt(w(50), limits, 0, 0);
    // Waiting after the harm: no refilling, but 1 a second drained.
    assert_eq!(vitals.at(Vital::Health, limits, 5 * SECOND), w(45));
}

#[test]
fn refilling_offsets_draining_and_full_drains_too() {
    let vitals = Vitals::default();
    // 2 a second drained against 100 over 5 minutes (1/3 a second) refilled.
    let limits = poisoned(2000, None);
    let at = vitals.at(Vital::Health, limits, 30 * SECOND);
    assert_eq!(at, w(100 - 60).plus(w(10)));
}

#[test]
fn draining_never_takes_it_below_one() {
    let mut vitals = Vitals::default();
    let limits = poisoned(5000, None);
    vitals.hurt(w(97), limits, 0, 0);
    assert_eq!(vitals.at(Vital::Health, limits, 5 * SECOND), w(1));
    // Below 1 already (harm can), it stays where it is.
    let mut low = Vitals::default();
    low.hurt(Points(99_500), limits, 0, 0);
    assert_eq!(low.at(Vital::Health, limits, 5 * SECOND), Points(500));
}

#[test]
fn draining_stops_at_its_end_and_refilling_goes_on() {
    let mut vitals = Vitals::default();
    let limits = poisoned(1000, Some(20 * SECOND));
    vitals.hurt(w(50), limits, 0, 0);
    // 20 drained; refilling from CALM_AFTER: 10 s of it while drained, 20 s after.
    let refilled = |ticks| refilled(Points(0), 100, REFILL_TICKS, ticks);
    let at = vitals.at(Vital::Health, limits, 40 * SECOND);
    assert_eq!(at, w(30).plus(refilled(30 * SECOND)));
}

#[test]
fn a_client_drains_what_it_is_told_the_same_way() {
    let mut vitals = Vitals::default();
    let limits = poisoned(1500, Some(40 * SECOND));
    vitals.hurt(w(30), limits, 0, 0);
    let told = vitals.told(Vital::Health, limits, 3 * SECOND);
    for ticks in [0, 1, 7 * SECOND, 20 * SECOND, 37 * SECOND, 90 * SECOND] {
        assert_eq!(
            told.after(ticks),
            vitals.at(Vital::Health, limits, 3 * SECOND + ticks),
            "{ticks} ticks after"
        );
    }
}

#[test]
fn settling_keeps_what_was_drained() {
    let mut vitals = Vitals::default();
    let limits = poisoned(1000, None);
    vitals.hurt(w(50), limits, 0, 0);
    vitals.settle(limits, 4 * SECOND);
    assert_eq!(vitals.at(Vital::Health, limits, 4 * SECOND), w(46));
    assert_eq!(vitals.at(Vital::Health, limits, 6 * SECOND), w(44));
}

#[test]
fn a_deadly_drain_takes_it_to_nothing() {
    let mut vitals = Vitals::default();
    let mut limits = poisoned(5000, None);
    limits.restore.deadly = true;
    vitals.hurt(w(90), limits, 0, 0);
    assert_eq!(vitals.at(Vital::Health, limits, 5 * SECOND), Points::ZERO);
    // A client told of it drains it there too.
    let told = vitals.told(Vital::Health, limits, 0);
    assert!(told.deadly);
    assert_eq!(told.after(5 * SECOND), Points::ZERO);
    // Mana's draining keeps its floor.
    assert!(!limits.rate(Vital::Mana).deadly);
}
