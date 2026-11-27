use super::place_words;
use crate::music::place::{Band, Daylight, Place};

#[test]
fn places_are_written_readably() {
    let places = [
        Place::Wilderness(Band::Cold, Daylight::Night),
        Place::Village,
        Place::Town,
    ];
    assert_eq!(
        place_words(&places),
        "Wilderness: cold, night · Village · Town"
    );
    assert_eq!(
        place_words(&[Place::Wilderness(Band::Any, Daylight::Day)]),
        "Wilderness: day"
    );
    assert_eq!(
        place_words(&[Place::Wilderness(Band::Any, Daylight::Any), Place::Inn]),
        "Wilderness · The Way Inn"
    );
}

#[test]
fn no_places_is_everywhere() {
    assert_eq!(place_words(&[]), "Everywhere");
}
