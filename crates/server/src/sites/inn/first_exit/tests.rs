use super::*;

#[test]
fn rolled_places_are_on_the_walkway_and_differ() {
    let a = rolled(Seed::new(1));
    let b = rolled(Seed::new(2));
    assert_ne!(a, b);
    for (x, y) in [a, b] {
        assert!((0..HALL_LENGTH).contains(&x));
        assert!(hall::cell(x, y).is_walkable(), "{x}, {y}");
    }
    assert_eq!(rolled(Seed::new(1)), a, "the same every time");
}

#[test]
fn chosen_places_are_read_and_kept_on_the_walkway() {
    assert_eq!(tile_from("40"), Some((40, 1)));
    assert_eq!(tile_from("40, 0"), Some((40, 0)));
    assert_eq!(tile_from("40,9"), Some((40, 2)));
    assert_eq!(tile_from("-1,1"), Some((HALL_LENGTH - 1, 1)));
    assert_eq!(tile_from("east"), None);
    assert_eq!(tile_from("1,2,3"), None);
}
