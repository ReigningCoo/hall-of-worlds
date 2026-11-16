use shared::debug_report::{DevPlace, ReportClock, Where};
use shared::facing::Heading;
use shared::poi::{ChunkPos, Poi, PoiKind};
use shared::protocol::{ClientMessage, DevGoToPlace, PlaceAnswer, PlaceRefusal, ServerMessage};
use shared::spot::Spot;
use shared::world::{ClockRate, WorldSeed};

use super::go_to_place::{find_world, go_to_place};
use super::tests::TestClient;
use crate::storage::SiteId;
use crate::{ServerWorld, ServerWorlds};

const SEED: WorldSeed = WorldSeed::new(12345);

/// Sends the ask as a client would, then has it done as at the end of the
/// tick, with the test's world as the server's only one. The answers.
fn load(test: &mut TestClient, ask: DevGoToPlace) -> Vec<ServerMessage> {
    test.send(&[ClientMessage::DevGoToPlace(ask)]);
    if let Some(ask) = test.client.going_place.take() {
        let world = std::mem::replace(&mut test.world, ServerWorld::new(WorldSeed::new(2)));
        let mut worlds = ServerWorlds::with_home(world);
        let options = test.options();
        assert!(go_to_place(&mut test.client, &mut worlds, ask, &options));
        test.world = worlds.into_home();
    }
    std::iter::from_fn(|| test.end.receive().unwrap()).collect()
}

fn answer(answers: &[ServerMessage]) -> Option<PlaceAnswer> {
    answers.iter().find_map(|message| match message {
        ServerMessage::DevPlace(answer) => Some(*answer),
        _ => None,
    })
}

/// A poi of one of `kinds` near where the test's player starts.
fn poi_near(test: &TestClient, kinds: &[PoiKind]) -> Poi {
    let (x, y) = test.client.world_position();
    let start = test.world.pois.chunk_at(x, y);
    (1..20)
        .flat_map(|ring| (-ring..=ring).flat_map(move |dy| (-ring..=ring).map(move |dx| (dx, dy))))
        .filter_map(|(dx, dy)| {
            let chunk = ChunkPos {
                x: start.x + dx,
                y: start.y + dy,
            };
            test.world.pois.in_chunk(chunk)
        })
        .find(|poi| kinds.contains(&poi.kind))
        .expect("such a poi near the start")
}

fn site(poi: &Poi, level: u32, at: Spot) -> Where {
    Where::Site {
        seed: SEED.terrain(),
        x: poi.x,
        y: poi.y,
        chunk: poi.chunk,
        kind: poi.kind,
        ruined: false,
        level,
        levels: 9,
        at,
        heading: Heading::EAST,
    }
}

fn ask(place: Where) -> DevGoToPlace {
    DevGoToPlace {
        place,
        planet: None,
        dev: None,
        clock: None,
    }
}

#[test]
fn a_site_level_is_gone_to_facing_the_way_it_was() {
    let mut test = TestClient::of(ServerWorld::new(SEED));
    let caves = [PoiKind::RockCave, PoiKind::EarthCave, PoiKind::GrassyCave];
    let cave = poi_near(&test, &caves);
    let answers = load(&mut test, ask(site(&cave, 1, Spot::new(0, 0))));
    let heading = Heading::EAST;
    assert_eq!(answer(&answers), Some(PlaceAnswer::Arrived { heading }));
    assert_eq!(test.client.heading, heading);
    let visit = test.client.visit.as_ref().expect("in the site");
    assert_eq!(visit.id(), SiteId::Poi(cave.chunk));
    // Spot (0, 0) is the level's corner, where nothing stands: the nearest
    // walkable cell instead.
    assert!(
        visit
            .cells()
            .is_walkable(visit.position().0, visit.position().1)
    );
}

#[test]
fn another_kind_of_poi_there_is_refused() {
    let mut test = TestClient::of(ServerWorld::new(SEED));
    let cave = poi_near(&test, &[PoiKind::RockCave, PoiKind::EarthCave]);
    let wrong = Poi {
        kind: PoiKind::Castle,
        ..cave
    };
    let answers = load(&mut test, ask(site(&wrong, 0, Spot::new(0, 0))));
    let refused = PlaceAnswer::Refused(PlaceRefusal::NoSuchPoi);
    assert_eq!(answer(&answers), Some(refused));
    assert!(test.client.visit.is_none());
}

#[test]
fn a_world_the_server_has_not_got_is_refused() {
    let mut test = TestClient::of(ServerWorld::new(SEED));
    let elsewhere = Where::WorldMap {
        seed: WorldSeed::new(77).terrain(),
        x: 3,
        y: 4,
    };
    let answers = load(&mut test, ask(elsewhere));
    let refused = PlaceAnswer::Refused(PlaceRefusal::UnknownWorld);
    assert_eq!(answer(&answers), Some(refused));
    // A planet that isn't the one the terrain seed says is no help.
    let wrong_planet = DevGoToPlace {
        planet: Some(78),
        ..ask(elsewhere)
    };
    assert_eq!(answer(&load(&mut test, wrong_planet)), Some(refused));
}

#[test]
fn a_debug_tools_site_is_made_again_at_its_level() {
    let mut test = TestClient::of(ServerWorld::new(SEED));
    let made = Poi {
        kind: PoiKind::Castle,
        ..poi_near(&test, &[PoiKind::RockCave, PoiKind::EarthCave])
    };
    let place = DevGoToPlace {
        dev: Some(DevPlace::Site {
            kind: PoiKind::Castle,
            seed: 1,
            ruin: 0,
        }),
        ..ask(site(&made, 0, Spot::new(0, 0)))
    };
    let answers = load(&mut test, place);
    assert!(matches!(
        answer(&answers),
        Some(PlaceAnswer::Arrived { .. })
    ));
    let visit = test.client.visit.as_ref().expect("in the site");
    let id = SiteId::Dev {
        kind: PoiKind::Castle,
        seed: 1,
        ruin: 0,
    };
    assert_eq!(visit.id(), id);
    assert_eq!(test.client.world_position(), (made.x, made.y));
}

#[test]
fn the_time_is_set_only_with_world_times_permission() {
    let mut test = TestClient::of(ServerWorld::new(SEED));
    let place = DevGoToPlace {
        clock: Some(ReportClock {
            minutes: 5_000.0,
            rate: ClockRate::NORMAL,
        }),
        ..ask(Where::WorldMap {
            seed: SEED.terrain(),
            x: 10,
            y: 10,
        })
    };
    // Given the debug tools by the server, which leaves out World Time.
    test.client.host = false;
    load(&mut test, place);
    assert!(test.world.minutes() < 5_000.0);
    assert_eq!(test.client.world_position(), (10, 10));
    // The single player's own has every tool.
    test.client.host = true;
    load(&mut test, place);
    assert!(test.world.minutes() >= 5_000.0);
}

#[test]
fn without_the_permission_nothing_happens() {
    let mut test = TestClient::of(ServerWorld::new(SEED));
    test.client.host = false;
    test.dev_tools = false;
    let place = ask(Where::WorldMap {
        seed: SEED.terrain(),
        x: 10,
        y: 10,
    });
    assert_eq!(answer(&load(&mut test, place)), None);
    assert_ne!(test.client.world_position(), (10, 10));
}

#[test]
fn worlds_are_found_by_their_terrain_seed_open_or_a_hall_planet() {
    let worlds = ServerWorlds::with_home(ServerWorld::new(SEED));
    assert_eq!(find_world(&worlds, SEED.terrain(), None), Some(SEED));
    let planet = WorldSeed::new(77);
    let found = find_world(&worlds, planet.terrain(), Some(77));
    assert_eq!(found, Some(planet));
    assert_eq!(find_world(&worlds, planet.terrain(), Some(78)), None);
}
