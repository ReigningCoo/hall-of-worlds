use super::super::playlist::{Instrument, PlaylistFile};
use super::super::test_tune::INSTRUMENTS;
use super::*;

const NIGHT: Daylight = Daylight::Night;
const DAY: Daylight = Daylight::Day;

/// Every place the player can be in.
fn every_place() -> Vec<Place> {
    let mut places = vec![
        Place::WorldMap,
        Place::Hall,
        Place::Inn,
        Place::Village,
        Place::Town,
        Place::City,
        Place::Castle,
        Place::Tower,
        Place::Cave,
        Place::Farm,
        Place::Herbalist,
        Place::Oasis,
    ];
    for band in [Band::Cold, Band::Temperate, Band::Dry, Band::Hot] {
        for time in [DAY, NIGHT] {
            places.push(Place::Wilderness(band, time));
        }
    }
    places
}

fn game() -> PlaylistFile {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/music/playlists/game.playlist.ron"
    );
    let text = std::fs::read_to_string(path).expect("the game's playlist");
    ron::from_str(&text).expect("it reads")
}

#[test]
fn a_place_gets_the_tunes_naming_it_and_those_for_everywhere() {
    let places = vec![
        vec![Place::Town],
        vec![Place::Wilderness(Band::Any, NIGHT)],
        vec![],
        vec![Place::Wilderness(Band::Cold, DAY), Place::Cave],
    ];
    assert_eq!(tunes_for(Place::Town, &places), [0, 2]);
    assert_eq!(tunes_for(Place::Cave, &places), [2, 3]);
    let cold_night = Place::Wilderness(Band::Cold, NIGHT);
    assert_eq!(tunes_for(cold_night, &places), [1, 2]);
    assert_eq!(
        tunes_for(Place::Wilderness(Band::Cold, DAY), &places),
        [2, 3]
    );
}

#[test]
fn a_place_no_tune_names_borrows() {
    let places = vec![
        vec![Place::Wilderness(Band::Cold, DAY)],
        vec![Place::Wilderness(Band::Temperate, NIGHT)],
        vec![Place::Inn],
    ];
    // The wilderness at the same time of day, any band; then any at all.
    assert_eq!(tunes_for(Place::Wilderness(Band::Dry, DAY), &places), [0]);
    assert_eq!(tunes_for(Place::Wilderness(Band::Dry, NIGHT), &places), [1]);
    let only_inn = vec![vec![Place::Inn]];
    let dry_day = Place::Wilderness(Band::Dry, DAY);
    assert_eq!(tunes_for(dry_day, &only_inn), [0]);
    // Anywhere else, every tune.
    assert_eq!(tunes_for(Place::Farm, &places), [0, 1, 2]);
}

#[test]
fn the_last_ensemble_naming_a_place_plays() {
    let file = game();
    let players = file.players.clone().unwrap_or_default();
    let harp = players_for(Place::WorldMap, &file.ensembles, &players);
    assert_eq!(harp.melody.program, 46);
    let cornemuse = players_for(
        Place::Wilderness(Band::Cold, DAY),
        &file.ensembles,
        &players,
    );
    assert_eq!((cornemuse.melody.bank, cornemuse.melody.program), (8, 109));
    let mut ensembles = file.ensembles.clone();
    ensembles.push(Ensemble {
        places: vec![Place::Wilderness(Band::Any, Daylight::Any)],
        players: Players::default(),
    });
    let modded = players_for(Place::Wilderness(Band::Cold, DAY), &ensembles, &players);
    assert_eq!(modded, Players::default());
}

#[test]
fn every_place_has_tunes_and_players_of_its_own() {
    let file = game();
    let places: Vec<Vec<Place>> = file.tunes.iter().map(|t| t.places.clone()).collect();
    for place in every_place() {
        let own = places
            .iter()
            .filter(|listed| listed.iter().any(|&named| place.within(named)))
            .count();
        assert!(own >= 3, "{place:?} has only {own} tunes");
        let named = file
            .ensembles
            .iter()
            .any(|e| e.places.iter().any(|&p| place.within(p)));
        assert!(named, "{place:?} has no ensemble");
    }
}

#[test]
fn larger_towns_have_more_players() {
    let file = game();
    let players = file.players.clone().unwrap_or_default();
    let count = |place| {
        let p = players_for(place, &file.ensembles, &players);
        1 + usize::from(p.second.is_some())
            + usize::from(p.drone.is_some())
            + usize::from(p.percussion.is_some())
    };
    assert!(count(Place::Village) < count(Place::Town));
    assert!(count(Place::Town) <= count(Place::City));
}

#[test]
fn every_ensembles_instruments_are_in_the_soundfont() {
    let file = game();
    let known = |i: Instrument| INSTRUMENTS.contains(&(i.bank, i.program));
    for ensemble in &file.ensembles {
        let p = &ensemble.players;
        let parts = [Some(p.melody), p.second, p.drone];
        for instrument in parts.into_iter().flatten() {
            assert!(known(instrument), "{:?}: {instrument:?}", ensemble.places);
        }
    }
}
