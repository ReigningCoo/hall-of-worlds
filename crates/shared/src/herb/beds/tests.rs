use super::*;
use crate::climate::Biome;
use crate::herb::{choose, wild_herbs};
use crate::poi::PoiKind;
use crate::site::{Generator, Site};

fn habitat(biome: Biome, celsius: i32) -> Habitat {
    Habitat {
        biome,
        metres: 100,
        celsius,
    }
}

fn hut_ground(n: u64) -> Level {
    let kind = PoiKind::HerbalistHut;
    let generator = Generator::for_poi(kind);
    let site = Site::with_generator(kind, Seed::new(n), generator.version(), generator).unwrap();
    site.level(0).level
}

/// Every bed cell gets a herb that grows wild there, one herb a strip, and the
/// same every time.
#[test]
fn beds_grow_herbs_of_their_place() {
    let herbs = Herbs::built_in();
    let place = habitat(Biome::Grassland, 12);
    for n in 0..20 {
        let mut level = hut_ground(n);
        plant_beds(&mut level, &herbs, &place, Seed::new(n));
        let (min_x, min_y, width, height) = level.bounds();
        let cells = (min_y..min_y + height)
            .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
            .filter(|&(x, y)| level.cell(x, y) == Cell::HerbBed)
            .count();
        assert!(cells > 0);
        assert_eq!(level.herbs().count(), cells);
        for ((x, y), patch) in level.herbs() {
            let def = herbs.get(&patch.herb).unwrap();
            assert!(def.suits(&place), "{}", def.key);
            for (bx, by) in bed_at(&level, (x, y)) {
                assert_eq!(level.herb_at(bx, by).unwrap().herb, patch.herb);
            }
        }
        let mut again = hut_ground(n);
        plant_beds(&mut again, &herbs, &place, Seed::new(n));
        assert_eq!(again, level);
    }
}

/// Somewhere no herb grows wild, a herbalist grows those nearest its warmth.
#[test]
fn beds_grow_something_anywhere() {
    let herbs = Herbs::built_in();
    for place in [habitat(Biome::Tundra, -40), habitat(Biome::Desert, 60)] {
        assert!(wild_herbs(&herbs, &place).is_empty());
        let mut level = hut_ground(3);
        plant_beds(&mut level, &herbs, &place, Seed::new(3));
        assert!(level.herbs().count() > 0);
    }
}

/// Marsh herbs with no other biome grow in the wettest until there are marshes.
#[test]
fn marsh_herbs_grow_somewhere() {
    let herbs = Herbs::built_in();
    let marsh = herbs.get("purple_loosestrife").unwrap();
    assert!(marsh.biomes.is_empty());
    assert!(marsh.suits(&habitat(Biome::SeasonalForest, 10)));
    assert!(!marsh.suits(&habitat(Biome::Grassland, 10)));
    let place = habitat(Biome::TemperateRainforest, 10);
    let mut rng = Seed::new(1).rng();
    let wild = wild_herbs(&herbs, &place);
    let chosen: BTreeSet<&str> = (0..2000)
        .map(|_| choose(&wild, Abundance::wild_weight, &mut rng).unwrap())
        .map(|def| def.key.as_str())
        .collect();
    assert!(chosen.contains("purple_loosestrife"));
}
