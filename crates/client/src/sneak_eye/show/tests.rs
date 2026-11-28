use super::*;

#[test]
fn closed_is_a_line_and_open_is_taller_with_a_pupil() {
    let (closed, no_pupil, _) = look(0.0, false);
    let (open, pupil, colour) = look(1.0, false);
    assert_eq!(closed, 2.0);
    assert_eq!(no_pupil, 0.0);
    assert!(open > closed && pupil > 0.0);
    assert_eq!(colour, WHITE);
}

#[test]
fn alert_opens_it_wide_whatever_the_rest() {
    let (wide, _, colour) = look(0.0, true);
    assert!(wide > look(1.0, false).0);
    assert_eq!(colour, ALERT);
}
