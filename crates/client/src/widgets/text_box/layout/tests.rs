use super::*;

#[test]
fn a_run_is_on_the_line_its_middle_is_level_with() {
    assert_eq!(line_of(0.0, 12.0, 12.0), 0);
    assert_eq!(line_of(12.0, 24.0, 12.0), 1);
    // An item a little taller than the line is still on it.
    assert_eq!(line_of(-1.0, 13.0, 12.0), 0);
}
