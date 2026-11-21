use super::*;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::material::Material;
use crate::spot::Spot;

#[test]
fn kindling_burns_five_to_ten_minutes_by_its_wood() {
    assert_eq!(burn_seconds(Material::Baobab), 300);
    assert_eq!(burn_seconds(Material::Mopane), 600);
    let oak = burn_seconds(Material::Oak);
    assert!(oak > burn_seconds(Material::Pine) && oak < 600, "{oak}");
    // Every wood within the plan's 5 to 10 minutes.
    for wood in crate::material::WOODS.iter() {
        assert!((300..=600).contains(&burn_seconds(wood.material)));
    }
}

#[test]
fn a_fire_burns_its_pieces_one_after_another() {
    let mut fire = Campfire::lit(1000, Material::Baobab);
    assert!(fire.is_lit(1299));
    assert_eq!(fire.pieces_left(1000), 1);
    fire.feed(1100, Material::Baobab).unwrap();
    assert_eq!(fire.ends(), 1600);
    assert_eq!(fire.pieces_left(1350), 1);
    assert!(!fire.is_lit(1600));
    assert_eq!(fire.feed(1600, Material::Oak), Err(FeedRefused::Out));
    assert_eq!(fire.ash(), 2);
}

#[test]
fn a_fire_holds_at_most_five_pieces() {
    let mut fire = Campfire::lit(0, Material::Pine);
    for _ in 1..MAX_KINDLING {
        fire.feed(10, Material::Pine).unwrap();
    }
    assert_eq!(fire.feed(10, Material::Pine), Err(FeedRefused::Full));
    // Once one has burnt away, another goes on.
    let first = u64::from(burn_seconds(Material::Pine));
    fire.feed(first, Material::Pine).unwrap();
    assert_eq!(fire.ash(), 6);
}

#[test]
fn settling_keeps_the_end_and_the_ash() {
    let mut fire = Campfire::lit(0, Material::Baobab);
    fire.feed(0, Material::Baobab).unwrap();
    fire.feed(0, Material::Baobab).unwrap();
    let (ends, ash) = (fire.ends(), fire.ash());
    fire.settle(650);
    assert_eq!(fire.burns.len(), 1);
    assert_eq!((fire.ends(), fire.ash()), (ends, ash));
    assert_eq!(fire.since, 600);
}

#[test]
fn fires_go_on_bare_ground_and_floors_only() {
    let mut level = Level::test_room();
    assert!(can_hold_fire(&level, 0, 0));
    assert!(!can_hold_fire(&level, -2, 0), "a wall");
    level.set(1, 0, Cell::WoodenBoards);
    assert!(!can_hold_fire(&level, 1, 0));
    level.set(2, 0, Cell::Grass);
    assert!(can_hold_fire(&level, 2, 0));
    level.set(3, 0, Cell::Water);
    assert!(!can_hold_fire(&level, 3, 0));
}

#[test]
fn the_cell_faced_is_lit_and_its_fire_fed() {
    let level = Level::test_room();
    let at = Spot::middle_of((0, 0));
    let free = |x: i64, y: i64| (x, y) != (0, 0);
    assert_eq!(place_faced(&level, at, Heading::EAST, free), Some((1, 0)));
    let fire_at = |x: i64, y: i64| (x, y) == (1, 0);
    assert_eq!(
        campfire_faced(&level, at, Heading::EAST, fire_at),
        Some((1, 0))
    );
    assert_eq!(campfire_faced(&level, at, Heading::WEST, fire_at), None);
}
