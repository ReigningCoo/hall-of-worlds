use super::*;
use shared::poi::PoiKind;

#[test]
fn a_found_poi_is_named_in_words() {
    assert_eq!(found(PoiKind::Village), "Found: Village");
    assert!(found(PoiKind::HerbalistHut).starts_with("Found: Herbalist"));
}

#[test]
fn a_harvest_names_the_herb() {
    let table = HerbTable::default();
    assert_eq!(picked(3, "roseroot", &table), "Picked 3 Roseroot");
    assert_eq!(picked(1, "modded", &table), "Picked 1 modded");
}

#[test]
fn a_fungus_is_named_only_once_known() {
    let lang = Lang::new(shared::lang::Language::English);
    let mut sheet = Sheet::new(shared::creature::CreatureKind::Human);
    let said = |sheet: &Sheet, named| picked_fungus("morel", 2, named, (sheet, &lang));
    assert_eq!(said(&sheet, false), "Picked 2 Unknown mushroom");
    assert_eq!(said(&sheet, true), "Picked 2 Morel, and learned its name");
    sheet.fungi.learn_name("morel");
    assert_eq!(said(&sheet, false), "Picked 2 Morel");
}

#[test]
fn the_notice_is_good_markup() {
    NOTICES.assert_good();
}

#[test]
fn a_locked_door_shows_a_notice_that_goes_by_itself() {
    use std::time::Duration;

    use bevy::time::TimeUpdateStrategy;

    use crate::dom::{test_app, test_find};

    let mut app = test_app();
    app.add_message::<FromServer>()
        .init_resource::<HerbTable>()
        .init_resource::<CharacterSheet>()
        .init_resource::<Wording>()
        .add_plugins(NoticesPlugin);
    // Each frame is a second long.
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(1)));
    app.update();
    let row = test_find(&mut app, "notices/row");
    let hidden = |app: &App| app.world().get::<Node>(row).unwrap().display == Display::None;
    assert!(hidden(&app));

    let locked = ServerMessage::DoorRefused {
        x: 0,
        y: 0,
        locked: true,
    };
    app.world_mut().write_message(FromServer(locked));
    app.update();
    app.update();
    assert!(!hidden(&app));
    let words = test_find(&mut app, "notices/words");
    let said = app.world().get::<Children>(words).unwrap()[0];
    assert_eq!(app.world().get::<Text>(said).unwrap().0, "Locked");

    // (A frame's time is capped well under a second.)
    for _ in 0..40 {
        app.update();
    }
    assert!(hidden(&app));
}
