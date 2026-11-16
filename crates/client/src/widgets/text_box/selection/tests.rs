use super::*;

#[test]
fn the_band_is_paler_without_the_keyboard_but_still_seen() {
    let (with, without) = (band_colour(true).alpha(), band_colour(false).alpha());
    assert!(without < with);
    assert!(without > 0.1);
}

fn three_lines() -> Spots {
    use super::super::spots::Spot;
    let places = [
        (0, 0.0),
        (0, 10.0),
        (1, 0.0),
        (1, 10.0),
        (2, 0.0),
        (2, 10.0),
    ];
    Spots {
        corner: Vec2::ZERO,
        width: 40.0,
        line_height: 12.0,
        places: places.iter().map(|&(line, x)| Spot { line, x }).collect(),
    }
}

#[test]
fn a_selection_on_one_line_is_one_band() {
    assert_eq!(
        bands(&three_lines(), 2, 3),
        vec![Rect::new(0.0, 12.0, 10.0, 24.0)]
    );
}

#[test]
fn a_selection_over_lines_takes_the_rest_of_the_first_the_middle_and_the_last() {
    assert_eq!(
        bands(&three_lines(), 1, 5),
        vec![
            Rect::new(10.0, 0.0, 40.0, 12.0),
            Rect::new(0.0, 12.0, 40.0, 24.0),
            Rect::new(0.0, 24.0, 10.0, 36.0),
        ]
    );
    // Over two lines, no middle.
    assert_eq!(bands(&three_lines(), 1, 3).len(), 2);
}
