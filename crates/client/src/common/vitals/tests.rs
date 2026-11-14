//! The drawn Health follows the server's rule (`VitalNow::after`).

use super::*;
use shared::vitals::Points;

fn told(value: u32, refills_in: u32, restore: u32, drain: u32, ends_in: u32) -> VitalNow {
    VitalNow {
        value: Points::whole(value),
        most: 20,
        refills_in,
        refill: 3_000,
        restore,
        restore_ends_in: ends_in,
        drain,
    }
}

#[test]
fn drawn_health_matches_the_rule_with_poison_and_healing() {
    let cases = [
        told(20, 0, 0, 1_000, 600),
        told(5, 100, 1_000, 1_000, 400),
        told(3, 0, 0, 2_000, u32::MAX),
        told(12, 50, 2_000, 0, 300),
    ];
    for told in cases {
        for ticks in [0, 10, 99, 250, 401, 900, 5_000] {
            let drawn = refilled(told, ticks as f32);
            let rule = told.after(ticks).as_f32();
            assert!(
                (drawn - rule).abs() < 0.01,
                "{told:?} at {ticks}: {drawn} vs {rule}"
            );
        }
    }
}
