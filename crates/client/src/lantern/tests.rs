use shared::lantern::LanternForm;

use super::words;

#[test]
fn the_panel_says_how_much_oil_and_how_long() {
    let full = LanternForm { ml: 200, burnt: 0 };
    assert_eq!(words::oil(full), "200 ml of oil, of 200 ml.");
    assert_eq!(
        words::left(full),
        "Burns for 2 h more, held in a hand or tied to the belt."
    );
    assert_eq!(words::fill(full), "Full.");
    let some = LanternForm { ml: 120, burnt: 6 };
    // 120 × 36 − 6 seconds: 71 minutes and a bit, rounded up.
    assert_eq!(
        words::left(some),
        "Burns for 1 h 12 min more, held in a hand or tied to the belt."
    );
    assert_eq!(
        words::fill(some),
        "Drop a bottle of oil here to fill it: 80 ml goes in, the rest is spilt."
    );
    let empty = LanternForm::default();
    assert_eq!(words::oil(empty), "Empty: it holds 200 ml of oil.");
    assert_eq!(words::left(empty), "Without oil it gives no light.");
    assert_eq!(words::fill(empty), "Drop a bottle of oil here to fill it.");
}
