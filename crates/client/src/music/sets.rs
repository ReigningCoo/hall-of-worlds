//! A place's tunes and players, from the playlists. A tune naming no places
//! plays everywhere (a mod's simplest tune). A place no tune names borrows:
//! the wilderness from any band at the same time of day, then from any, and
//! anywhere else, every tune.

use super::place::{Band, Daylight, Place};
use super::playlist::{Ensemble, Players};

/// The tunes for `place`, by their place in the list of each tune's places.
pub(super) fn tunes_for(place: Place, places: &[Vec<Place>]) -> Vec<usize> {
    let mut wanted = vec![place];
    if let Place::Wilderness(_, time) = place {
        wanted.push(Place::Wilderness(Band::Any, time));
        wanted.push(Place::Wilderness(Band::Any, Daylight::Any));
    }
    for place in wanted {
        let found = matching(place, places);
        // Tunes for everywhere don't count: a place needs its own.
        if found.iter().any(|&at| !places[at].is_empty()) {
            return found;
        }
    }
    (0..places.len()).collect()
}

/// The tunes naming a place this one is within, or none at all.
fn matching(place: Place, places: &[Vec<Place>]) -> Vec<usize> {
    let suits = |listed: &Vec<Place>| {
        listed.is_empty() || listed.iter().any(|&named| any_within(place, named))
    };
    (0..places.len()).filter(|&at| suits(&places[at])).collect()
}

/// Like `Place::within`, but a wilderness asked for with `Any` (when
/// borrowing) takes in every tune of the wilderness it covers.
fn any_within(place: Place, named: Place) -> bool {
    match (place, named) {
        (Place::Wilderness(band, time), Place::Wilderness(named_band, named_time)) => {
            let band_ok = band == Band::Any || named_band == Band::Any || band == named_band;
            let time_ok =
                time == Daylight::Any || named_time == Daylight::Any || time == named_time;
            band_ok && time_ok
        }
        _ => place.within(named),
    }
}

/// Who plays at `place`: the last ensemble naming it, else the playlist's
/// players.
pub(super) fn players_for(place: Place, ensembles: &[Ensemble], players: &Players) -> Players {
    let named = |ensemble: &&Ensemble| ensemble.places.iter().any(|&p| place.within(p));
    match ensembles.iter().rev().find(named) {
        Some(ensemble) => ensemble.players.clone(),
        None => players.clone(),
    }
}

#[cfg(test)]
mod tests;
