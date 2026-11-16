use super::*;

/// A floor `size` cells square: `outside` round a walled box from `(lo, lo)` to
/// `(hi, hi)`, floor inside.
fn floor(size: i64, (lo, hi): (i64, i64), outside: Cell) -> Level {
    let mut level = Level::new(0, 0, size, size);
    for y in 0..size {
        for x in 0..size {
            let cell = if x < lo || x > hi || y < lo || y > hi {
                outside
            } else if x == lo || x == hi || y == lo || y == hi {
                Cell::Wall
            } else {
                Cell::Floor
            };
            level.set(x, y, cell);
        }
    }
    level
}

/// Two floors of the same box, a window in the upper one's south wall.
fn two_floors(size: i64) -> (Level, Level) {
    let ground = floor(size, (3, 10), Cell::Ground);
    let mut upper = floor(size, (3, 10), Cell::Air);
    upper.set(6, 10, Cell::Window);
    (ground, upper)
}

/// What a viewer at `at` on floor `viewer` sees of the others.
fn seen_from(storeys: &[&Level], viewer: usize, at: (i64, i64)) -> Vec<Sight> {
    let own = storeys[viewer];
    let mut seen = Sight::new(own);
    seen.add_view(own, at.0, at.1, None);
    other_floors(storeys, viewer, at, &seen)
}

#[test]
fn looking_out_of_a_window_shows_the_ground_below_but_not_the_wall_under_it() {
    let (ground, upper) = two_floors(16);
    let seen = seen_from(&[&ground, &upper], 1, (6, 9));
    assert!(seen[0].is_seen(6, 12), "the ground outside");
    assert!(!seen[0].is_seen(6, 10), "the wall under the window");
    assert!(!seen[0].is_seen(6, 8), "the room below");
    assert!(!seen[1].is_seen(6, 8), "nothing of the viewer's own floor");
}

#[test]
fn from_outside_the_upper_face_is_seen_but_not_the_rooms_behind_it() {
    let (ground, upper) = two_floors(16);
    let seen = seen_from(&[&ground, &upper], 0, (6, 13));
    assert!(seen[1].is_seen(6, 10), "the window");
    assert!(seen[1].is_seen(4, 10), "the wall beside it");
    assert!(seen[1].is_seen(6, 12), "the air in front");
    assert!(!seen[1].is_seen(6, 8), "the room behind the window");
    assert!(!seen[1].is_seen(6, 3), "the far side");
}

#[test]
fn a_floor_set_back_shows_its_face_behind_the_air() {
    let ground = floor(16, (3, 10), Cell::Ground);
    let upper = floor(16, (4, 9), Cell::Air);
    let seen = seen_from(&[&ground, &upper], 0, (6, 13));
    assert!(seen[1].is_seen(6, 10), "the air over the wall below");
    assert!(seen[1].is_seen(6, 9), "the face set back");
    assert!(!seen[1].is_seen(6, 8), "the room behind it");
}

#[test]
fn holes_show_straight_down() {
    let (ground, mut upper) = two_floors(16);
    upper.set(6, 6, Cell::Hole);
    let seen = seen_from(&[&ground, &upper], 1, (6, 7));
    assert!(seen[0].is_seen(6, 6), "under the hole");
    assert!(!seen[0].is_seen(7, 6), "beside it");
}

#[test]
fn the_ground_is_only_seen_so_far() {
    let (ground, upper) = two_floors(60);
    let seen = seen_from(&[&ground, &upper], 1, (6, 9));
    assert!(seen[0].is_seen(6, 9 + GROUND_REACH));
    assert!(!seen[0].is_seen(6, 10 + GROUND_REACH));
}

#[test]
fn all_floors_above_are_seen_full_height() {
    let ground = floor(16, (3, 10), Cell::Ground);
    let middle = floor(16, (3, 10), Cell::Air);
    let top = floor(16, (3, 10), Cell::Air);
    let seen = seen_from(&[&ground, &middle, &top], 0, (6, 13));
    assert!(seen[1].is_seen(6, 10));
    assert!(seen[2].is_seen(6, 10));
}
