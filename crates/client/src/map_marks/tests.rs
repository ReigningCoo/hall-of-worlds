use shared::id::Id;
use shared::map_mark::{MapMark, MarkColour, MarkIcon, middle_of_point};

use crate::common::{MapMarks, in_points, near_way_round, point_at};
use crate::minimap::mark_pixels;

fn mark(n: u128, at: (i64, i64)) -> MapMark {
    MapMark {
        id: Id::for_test(n),
        at,
        icon: MarkIcon::Camp,
        colour: MarkColour::Blue,
        auto: false,
    }
}

#[test]
fn a_mark_placed_on_a_point_shows_on_it() {
    for point in [(0, 0), (5, -3), (-7, 12)] {
        let at = in_points(middle_of_point(point));
        assert!((at.0 - point.0 as f64).abs() < 0.1 && (at.1 - point.1 as f64).abs() < 0.1);
        assert_eq!(point_at(at), point);
    }
}

#[test]
fn the_nearest_mark_is_found_the_near_way_round() {
    let width = 1000;
    let marks = MapMarks(vec![
        mark(1, middle_of_point((999, 4))),
        mark(2, middle_of_point((3, 4))),
    ]);
    // Just east of the seam, the mark just west of it is the nearer.
    let found = marks.near((0.2, 4.0), 2.0, width).map(|mark| mark.id);
    assert_eq!(found, Some(Id::for_test(1)));
    assert_eq!(marks.near((500.0, 4.0), 2.0, width), None);
    assert_eq!(near_way_round(990.0, width), -10.0);
}

#[test]
fn a_mark_is_a_black_disc_with_its_colour_inside() {
    let size = 32;
    let pixels = mark_pixels((MarkIcon::Bag, MarkColour::Red), size);
    let at = |x: u32, y: u32| {
        let i = ((y * size + x) * 4) as usize;
        [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
    };
    assert_eq!(at(0, 0)[3], 0, "outside the disc is clear");
    assert_eq!(at(16, 18), [232, 52, 44, 255], "the bag's middle is red");
    assert_eq!(
        at(9, 10)[..3],
        [0, 0, 0],
        "between the edge and the icon is black"
    );
    assert!(at(16, 2)[0] > 200 && at(16, 2)[1] < 80, "the edge is red");
}
