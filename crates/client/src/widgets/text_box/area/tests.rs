use super::*;

#[test]
fn the_area_scrolls_only_as_far_as_it_must_to_show_the_caret() {
    // In sight: stays.
    assert_eq!(follow(0.0, 12.0, 24.0, 48.0, 100.0), 0.0);
    // Below: just enough to show its line.
    assert_eq!(follow(0.0, 60.0, 72.0, 48.0, 100.0), 24.0);
    // Above: its line at the top.
    assert_eq!(follow(40.0, 12.0, 24.0, 48.0, 100.0), 12.0);
    // Never past where it can go.
    assert_eq!(follow(0.0, 200.0, 212.0, 48.0, 100.0), 100.0);
}

#[test]
fn a_box_of_rows_is_as_tall_as_its_lines_and_edges() {
    let one = text_area_height(TextSize::Medium, 1);
    assert_eq!(text_area_height(TextSize::Medium, 0), one);
    let four = text_area_height(TextSize::Medium, 4);
    assert!((four - one - 3.0 * line_height(TextSize::Medium)).abs() < 1e-4);
}
