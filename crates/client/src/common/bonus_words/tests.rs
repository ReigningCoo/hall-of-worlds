use super::*;

const MINUS: char = '\u{2212}';

#[test]
fn numbers_keep_one_place_at_most() {
    assert_eq!(number(40.0), "40");
    assert_eq!(number(12.34), "12.3");
    assert_eq!(number(0.6), "0.6");
    assert_eq!(signed(40_000), "+40");
    assert_eq!(signed(-8_000), format!("{MINUS}8"));
    assert_eq!(signed(-600), format!("{MINUS}0.6"));
}
