use shared::ground::Load;
use shared::material::Material;

use super::name::{Found, bag_name, mark_of, name_of};
use crate::common::HoverName;

#[test]
fn names_a_creature_by_its_hover_name() {
    let wolf = HoverName("Wolf".into());
    let found = Found {
        token: Some((None, Some(&wolf))),
        place: Some("Stairs down"),
        ..Found::default()
    };
    assert_eq!(name_of(found).as_deref(), Some("Wolf"));
}

#[test]
fn the_sky_comes_before_the_map() {
    let found = Found {
        sky: Some("Sol"),
        bag: Some(Load::Half),
        place: Some("Inn"),
        ..Found::default()
    };
    assert_eq!(name_of(found).as_deref(), Some("Sol"));
}

#[test]
fn a_bag_comes_before_the_place_under_it() {
    let found = Found {
        bag: Some(Load::Half),
        place: Some("Stairs up"),
        ..Found::default()
    };
    assert_eq!(name_of(found).as_deref(), Some("Bag"));
    let place = Found {
        place: Some("Stairs up"),
        ..Found::default()
    };
    assert_eq!(name_of(place).as_deref(), Some("Stairs up"));
}

#[test]
fn nothing_hovered_shows_nothing() {
    assert_eq!(name_of(Found::default()), None);
}

#[test]
fn a_mark_shows_only_with_a_players_name() {
    use crate::common::{HoverName, StaffMark};
    let name = HoverName("Brin".into());
    let mark = StaffMark("Admin".into());
    let player = Found {
        token: Some((None, Some(&name))),
        ..Found::default()
    };
    assert_eq!(mark_of(player, Some(&mark)), Some("Admin"));
    assert_eq!(mark_of(player, None), None);
    let behind_the_sky = Found {
        sky: Some("Sol"),
        ..player
    };
    assert_eq!(mark_of(behind_the_sky, Some(&mark)), None);
}

#[test]
fn the_box_is_good_markup_starting_at_its_lowest() {
    super::spawn::HOVER.assert_good();
    let bottom = format!(r#"bottom="{}""#, super::spawn::LOWEST);
    assert!(super::spawn::HOVER.source.contains(&bottom));
}

#[test]
fn logs_and_casks_on_the_ground_are_named_by_their_wood() {
    let logs = |count| Load::Logs {
        wood: Material::Oak,
        count,
    };
    assert_eq!(bag_name(logs(1)), "Oak Log");
    assert_eq!(bag_name(logs(3)), "Oak Logs");
    assert_eq!(bag_name(Load::Flat), "Bag");
}
