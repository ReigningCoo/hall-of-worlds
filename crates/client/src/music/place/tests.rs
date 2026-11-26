use shared::climate::Biome;
use shared::poi::PoiKind;
use shared::protocol::{LevelInfo, LevelKind};
use shared::spot::Spot;

use super::*;

const COLD_NIGHT: Place = Place::Wilderness(Band::Cold, Daylight::Night);

fn level(kind: LevelKind, index: u32, levels: u32, basement: bool, building: bool) -> LevelInfo {
    LevelInfo {
        kind,
        ruined: false,
        building,
        index,
        levels,
        basement,
        bounds: (0, 0, 8, 8),
        arrival: Spot::default(),
        entering_site: true,
        place_name: None,
        ground: Default::default(),
    }
}

#[test]
fn any_takes_in_every_band_and_time() {
    let place = Place::Wilderness(Band::Dry, Daylight::Night);
    assert!(place.within(Place::Wilderness(Band::Any, Daylight::Night)));
    assert!(place.within(Place::Wilderness(Band::Dry, Daylight::Any)));
    assert!(place.within(Place::Wilderness(Band::Any, Daylight::Any)));
    assert!(!place.within(Place::Wilderness(Band::Dry, Daylight::Day)));
    assert!(!place.within(Place::Wilderness(Band::Cold, Daylight::Any)));
    assert!(!place.within(Place::Cave));
    assert!(Place::Town.within(Place::Town));
    assert!(!Place::Town.within(Place::City));
}

#[test]
fn bands_follow_the_biome() {
    assert_eq!(Band::of(Biome::Tundra), Band::Cold);
    assert_eq!(Band::of(Biome::BorealForest), Band::Cold);
    assert_eq!(Band::of(Biome::Woodland), Band::Temperate);
    assert_eq!(Band::of(Biome::Desert), Band::Dry);
    assert_eq!(Band::of(Biome::Savanna), Band::Hot);
    assert_eq!(Band::of(Biome::TropicalRainforest), Band::Hot);
}

#[test]
fn levels_are_places_by_their_kind() {
    let site = |kind: PoiKind| level(LevelKind::Poi(kind), 0, 2, false, true);
    let of = |info: &LevelInfo| Place::of_level(info, COLD_NIGHT);
    assert_eq!(of(&site(PoiKind::Hamlet)), Place::Village);
    assert_eq!(of(&site(PoiKind::City)), Place::City);
    assert_eq!(of(&site(PoiKind::Fort)), Place::Castle);
    assert_eq!(of(&site(PoiKind::WoodenTower)), Place::Tower);
    assert_eq!(of(&site(PoiKind::Crypt)), Place::Cave);
    assert_eq!(of(&site(PoiKind::HerbalistHut)), Place::Herbalist);
    // Huts and ruins are the wilderness round them.
    assert_eq!(of(&site(PoiKind::StoneCottage)), COLD_NIGHT);
    assert_eq!(
        of(&level(LevelKind::Wilderness, 0, 1, false, false)),
        COLD_NIGHT
    );
    assert_eq!(of(&level(LevelKind::Inn, 1, 2, false, true)), Place::Inn);
    assert_eq!(
        of(&level(LevelKind::Hall { origin: 4 }, 0, 1, false, false)),
        Place::Hall
    );
}

#[test]
fn cellars_and_caves_are_caves() {
    let castle_cellar = level(LevelKind::Poi(PoiKind::Castle), 2, 3, true, true);
    assert_eq!(Place::of_level(&castle_cellar, COLD_NIGHT), Place::Cave);
    let castle_top = level(LevelKind::Poi(PoiKind::Castle), 1, 3, true, true);
    assert_eq!(Place::of_level(&castle_top, COLD_NIGHT), Place::Castle);
    let ruin_depths = level(LevelKind::Poi(PoiKind::Ruins), 1, 3, false, false);
    assert_eq!(Place::of_level(&ruin_depths, COLD_NIGHT), Place::Cave);
}
