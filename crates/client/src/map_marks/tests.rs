use shared::id::Id;
use shared::lang::{Lang, Language};
use shared::map_mark::{MapMark, MarkColour, MarkIcon, middle_of_point};
use shared::spot::Spot;

use super::press::travel_target;
use crate::common::{MapMarks, in_cells, in_points, mark_name, near_way_round, point_at};
use crate::minimap::mark_pixels;

fn mark(n: u128, at: (i64, i64)) -> MapMark {
    MapMark {
        id: Id::for_test(n),
        at,
        icon: MarkIcon::Camp,
        colour: MarkColour::Blue,
        auto: false,
        pile: None,
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

#[test]
fn a_piles_mark_shows_on_the_pile_and_is_named_by_it() {
    // A planet 100 points round: 1,600 cells.
    let pile = Spot::new(5 * 240 + 30, 7 * 240 + 200);
    let mark = MapMark::for_pile(Id::for_test(1), pile, 100, true).unwrap();
    let (x, y) = in_cells(&mark);
    assert!((x - 4.625).abs() < 1e-9 && (y - 7.333).abs() < 0.001);
    assert_eq!(point_at((x, y)), (5, 7));
    let marks = MapMarks(vec![mark]);
    // Found from across the seam, in cells.
    assert!(marks.near_cell((1_599.0, 7.0), 8.0, 100).is_some());
    assert!(marks.near_cell((40.0, 7.0), 8.0, 100).is_none());

    let lang = Lang::new(Language::English);
    assert_eq!(mark_name(&lang, &mark), "Log pile");
    let bag = MapMark::for_pile(Id::for_test(2), pile, 100, false).unwrap();
    assert_eq!(mark_name(&lang, &bag), "Pile");
    assert_eq!(mark_name(&lang, &mark_at(3)), "Your mark: Camp");
}

fn mark_at(n: u128) -> MapMark {
    mark(n, (0, 0))
}

#[test]
fn travel_here_goes_to_the_point_or_the_cell_the_near_way_round() {
    let camp = mark(1, middle_of_point((3, 4)));
    assert_eq!(travel_target(&camp, None), (3, 4));
    // In the wilderness of a planet 100 points round, from just west of the seam.
    assert_eq!(travel_target(&camp, Some((1_590, 100))), (1_600 + 56, 72));
    assert_eq!(travel_target(&camp, Some((40, 100))), (56, 72));
}
