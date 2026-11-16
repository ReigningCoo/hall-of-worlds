use shared::debug_report::{Build, DebugFlags, DebugReport, Platform, ReportClock, Where, encode};
use shared::role::DebugTool;
use shared::world::{ClockRate, WorldSeed};

use super::button::BUTTON;
use super::load::ask_for;
use super::markup::{MOST_PROBLEM, PANEL};
use crate::common::DebugTools;

#[test]
fn the_markup_is_good() {
    BUTTON.assert_good();
    PANEL.assert_good();
}

#[test]
fn the_box_takes_the_most_a_description_may_have() {
    let wanted = format!("maxlength=\"{MOST_PROBLEM}\"");
    assert!(PANEL.source.contains(&wanted));
}

#[test]
fn the_button_sits_left_of_the_gear() {
    // The gear is 8 from the right and 34 across; 6 between them.
    assert!(BUTTON.source.contains(r#"right="48""#));
    assert!(BUTTON.source.contains(r#"top="8""#));
}

fn report(place: Option<Where>) -> DebugReport {
    DebugReport {
        build: Build {
            version: "0.1.0".to_owned(),
            protocol: 1,
            platform: Platform::Native,
            system: "windows".to_owned(),
        },
        utc: 0,
        who: None,
        place,
        planet: Some(9),
        clock: Some(ReportClock {
            minutes: 600.0,
            rate: ClockRate::NORMAL,
        }),
        camera: None,
        flags: DebugFlags::default(),
        dev: None,
        problem: "The door floats".to_owned(),
    }
}

#[test]
fn load_asks_for_the_place_and_the_time_only_if_wanted_and_allowed() {
    let place = Where::WorldMap {
        seed: WorldSeed::new(1).terrain(),
        x: 4,
        y: -5,
    };
    let string = encode(&report(Some(place)));
    let staff = DebugTools::of([DebugTool::LoadDebugString]);
    let (ask, read) = ask_for(&string, true, staff).unwrap();
    assert_eq!((ask.place, ask.planet, ask.clock), (place, Some(9), None));
    assert_eq!(read.problem, "The door floats");
    let admin = DebugTools::of([DebugTool::LoadDebugString, DebugTool::WorldTime]);
    assert!(ask_for(&string, true, admin).unwrap().0.clock.is_some());
    assert!(ask_for(&string, false, admin).unwrap().0.clock.is_none());
}

#[test]
fn load_refuses_strings_it_cant_use_and_those_not_allowed() {
    let staff = DebugTools::of([DebugTool::LoadDebugString]);
    assert!(ask_for("hello", false, staff).is_err());
    let nowhere = encode(&report(None));
    assert!(ask_for(&nowhere, false, staff).is_err());
    let place = Where::WorldMap {
        seed: WorldSeed::new(1).terrain(),
        x: 0,
        y: 0,
    };
    let string = encode(&report(Some(place)));
    let player = DebugTools::of([DebugTool::GiveItem]);
    assert!(ask_for(&string, false, player).is_err());
}
