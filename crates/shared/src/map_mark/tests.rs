use super::*;

fn mark(n: u128, auto: bool) -> MapMark {
    MapMark {
        id: Id::for_test(n),
        at: (0, 0),
        icon: MarkIcon::Flag,
        colour: MarkColour::Red,
        auto,
    }
}

#[test]
fn keys_read_back() {
    for icon in MarkIcon::ALL {
        assert_eq!(MarkIcon::from_key(icon.key()), Some(icon));
    }
    for colour in MarkColour::ALL {
        assert_eq!(MarkColour::from_key(colour.key()), Some(colour));
    }
    assert_eq!(MarkIcon::from_key("dragon"), None);
}

#[test]
fn a_point_marks_its_middle() {
    assert_eq!(middle_of_point((2, -1)), (40, -8));
    assert_eq!(point_of(middle_of_point((2, -1))), (2, -1));
    assert_eq!(point_of((-1, 15)), (-1, 0));
}

#[test]
fn places_wrap_round_the_planet_and_stop_at_the_ice() {
    let points = 900;
    let width = points * SCALE;
    assert_eq!(place((-3, 5), points), Some((width - 3, 5)));
    assert_eq!(place((width + 7, 5), points), Some((7, 5)));
    // 2/9 of the way round north or south is past the ice.
    assert_eq!(place((0, 200 * SCALE), points), None);
    assert_eq!(place((0, -200 * SCALE), points), None);
    assert!(place((0, 199 * SCALE), points).is_some());
}

#[test]
fn only_hand_marks_count_towards_the_limit() {
    let mut marks: Vec<MapMark> = (0..MOST_HAND_MARKS as u128)
        .map(|n| mark(n, false))
        .collect();
    assert!(!may_add_by_hand(&marks));
    marks.pop();
    marks.extend((1000..1100).map(|n| mark(n, true)));
    assert_eq!(hand_marks(&marks), MOST_HAND_MARKS - 1);
    assert!(may_add_by_hand(&marks));
}
