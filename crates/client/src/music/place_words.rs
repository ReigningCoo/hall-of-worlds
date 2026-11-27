//! A tune's places as words for the debug panel: "Wilderness: cold, night ·
//! Village · Town", or "Everywhere" when it names none.

use super::place::{Band, Daylight, Place};

/// The places a playlist names for a tune, readably.
pub(super) fn place_words(places: &[Place]) -> String {
    if places.is_empty() {
        return "Everywhere".to_owned();
    }
    let words: Vec<String> = places.iter().map(|&place| place_word(place)).collect();
    words.join(" · ")
}

fn place_word(place: Place) -> String {
    let name = match place {
        Place::Wilderness(band, time) => return wilderness(band, time),
        Place::WorldMap => "World map",
        Place::Hall => "Hall",
        Place::Inn => "The Way Inn",
        Place::Village => "Village",
        Place::Town => "Town",
        Place::City => "City",
        Place::Castle => "Castle",
        Place::Tower => "Tower",
        Place::Cave => "Cave",
        Place::Farm => "Farm",
        Place::Herbalist => "Herbalist's hut",
        Place::Oasis => "Oasis",
    };
    name.to_owned()
}

/// "Wilderness", with its band and time when it names them.
fn wilderness(band: Band, time: Daylight) -> String {
    let band = match band {
        Band::Any => None,
        Band::Cold => Some("cold"),
        Band::Temperate => Some("temperate"),
        Band::Dry => Some("dry"),
        Band::Hot => Some("hot"),
    };
    let time = match time {
        Daylight::Any => None,
        Daylight::Day => Some("day"),
        Daylight::Night => Some("night"),
    };
    let named: Vec<&str> = [band, time].into_iter().flatten().collect();
    match named.is_empty() {
        true => "Wilderness".to_owned(),
        false => format!("Wilderness: {}", named.join(", ")),
    }
}

#[cfg(test)]
mod tests;
