use super::*;

fn on_one_line(xs: &[f32]) -> Vec<Spot> {
    xs.iter().map(|&x| Spot { line: 0, x }).collect()
}

fn spots(places: &[(usize, f32)]) -> Spots {
    Spots {
        corner: Vec2::new(100.0, 50.0),
        width: 40.0,
        line_height: 10.0,
        places: places.iter().map(|&(line, x)| Spot { line, x }).collect(),
    }
}

#[test]
fn places_fall_between_the_characters_after_the_prefix() {
    let runs = [
        Run::Prefix(0.0, 10.0),
        Run::Char(0, 0, 10.0, 18.0),
        Run::Char(1, 0, 18.0, 22.0),
        Run::Other,
    ];
    assert_eq!(
        spots_of(runs, &[false; 2], 2),
        on_one_line(&[10.0, 18.0, 22.0])
    );
}

#[test]
fn an_empty_box_has_one_place_after_its_prefix() {
    assert_eq!(
        spots_of([Run::Prefix(0.0, 7.0)], &[], 0),
        on_one_line(&[7.0])
    );
    assert_eq!(spots_of([], &[], 0), on_one_line(&[0.0]));
}

#[test]
fn a_character_not_laid_out_yet_takes_no_room() {
    let runs = [Run::Char(0, 0, 0.0, 5.0), Run::Char(2, 0, 5.0, 9.0)];
    assert_eq!(
        spots_of(runs, &[false; 3], 3),
        on_one_line(&[0.0, 5.0, 5.0, 9.0])
    );
}

#[test]
fn a_line_break_starts_the_next_line_even_an_empty_one() {
    // "a\n\nb": the breaks have no runs.
    let runs = [Run::Char(0, 0, 0.0, 6.0), Run::Char(3, 2, 0.0, 7.0)];
    let found = spots_of(runs, &[false, true, true, false], 4);
    let wanted = [(0, 0.0), (0, 6.0), (1, 0.0), (2, 0.0), (2, 7.0)];
    assert_eq!(found, spots(&wanted).places);
}

#[test]
fn a_place_at_a_wrap_is_at_the_next_lines_start() {
    // "ab cd" wrapped after the space.
    let runs = [
        Run::Char(0, 0, 0.0, 5.0),
        Run::Char(1, 0, 5.0, 10.0),
        Run::Char(2, 0, 10.0, 13.0),
        Run::Char(3, 1, 0.0, 5.0),
        Run::Char(4, 1, 5.0, 10.0),
    ];
    let found = spots_of(runs, &[false; 5], 5);
    let wanted = [(0, 0.0), (0, 5.0), (0, 10.0), (1, 0.0), (1, 5.0), (1, 10.0)];
    assert_eq!(found, spots(&wanted).places);
}

#[test]
fn the_nearest_place_is_on_the_line_pointed_at() {
    let found = spots(&[(0, 0.0), (0, 8.0), (0, 12.0), (1, 0.0), (1, 20.0)]);
    assert_eq!(found.nearest(Vec2::new(95.0, 55.0)), 0);
    assert_eq!(found.nearest(Vec2::new(109.0, 55.0)), 1);
    assert_eq!(found.nearest(Vec2::new(111.0, 55.0)), 2);
    assert_eq!(found.nearest(Vec2::new(111.0, 65.0)), 4);
    // Above the writing, the first line; below it, the last.
    assert_eq!(found.nearest(Vec2::new(200.0, 0.0)), 2);
    assert_eq!(found.nearest(Vec2::new(103.0, 500.0)), 3);
}

#[test]
fn up_and_down_keep_the_column_as_near_as_they_can() {
    let found = spots(&[
        (0, 0.0),
        (0, 10.0),
        (0, 20.0),
        (1, 0.0),
        (2, 0.0),
        (2, 10.0),
        (2, 20.0),
    ]);
    // From the end of the first line down to the empty one, then on down,
    // back to the same column.
    let (to, goal) = found.vertical(2, None, true);
    assert_eq!((to, goal), (3, 20.0));
    assert_eq!(found.vertical(to, Some(goal), true), (6, 20.0));
    // Off the first line to the start, off the last to the end.
    assert_eq!(found.vertical(1, None, false), (0, 10.0));
    assert_eq!(found.vertical(5, None, true), (6, 10.0));
}

#[test]
fn home_and_end_go_to_the_lines_ends() {
    let found = spots(&[(0, 0.0), (0, 10.0), (1, 0.0), (1, 10.0), (1, 20.0)]);
    assert_eq!(found.line_ends(3), (2, 4));
    assert_eq!(found.line_ends(0), (0, 1));
}
