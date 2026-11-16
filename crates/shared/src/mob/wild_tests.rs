use super::*;
use crate::dungeon::Level;

/// A block of open ground.
fn open_block() -> Level {
    let mut level = Level::new(0, 0, 64, 64);
    for y in 0..64 {
        for x in 0..64 {
            level.set(x, y, Cell::Grass);
        }
    }
    level
}

#[test]
fn blocks_have_packs_or_herds_by_biome_and_the_same_every_time() {
    let seed = Seed::new(7);
    let level = open_block();
    let blocks = (0..60).map(|n| BlockPos { x: n, y: 3 });
    let mut lived = 0;
    for block in blocks {
        let boreal = Habitat::typical(Biome::BorealForest);
        let herd = wild_spawns(seed, block, &level, boreal);
        assert_eq!(herd, wild_spawns(seed, block, &level, boreal));
        assert!(herd.len() <= MOST_TOGETHER as usize);
        let kind = herd.first().map(|mob| mob.kind);
        assert!(herd.iter().all(|mob| Some(mob.kind) == kind));
        assert!(kind.is_none_or(|kind| matches!(kind, CreatureKind::Wolf | CreatureKind::Coo)));
        let desert = Habitat::typical(Biome::Desert);
        assert!(wild_spawns(seed, block, &level, desert).is_empty());
        lived += usize::from(!herd.is_empty());
    }
    assert!((20..=45).contains(&lived), "{lived} of 60 blocks");
}

fn land(biome: Biome, metres: i32, celsius: i32) -> Habitat {
    Habitat {
        biome,
        metres,
        celsius,
    }
}

#[test]
fn coos_are_more_on_cold_high_ground_and_none_where_hot() {
    assert_eq!(coo_percent(land(Biome::Grassland, 0, 15)), 15);
    assert_eq!(coo_percent(land(Biome::Tundra, 1500, -10)), 40);
    let middling = coo_percent(land(Biome::Woodland, 600, 8));
    assert!((20..35).contains(&middling), "{middling}");
    for hot in [Biome::Savanna, Biome::Desert, Biome::TropicalRainforest] {
        assert_eq!(coo_percent(land(hot, 1500, -10)), 0);
        let shares = shares(land(hot, 0, 25));
        assert!(shares.iter().all(|&(kind, _)| kind != CreatureKind::Coo));
    }
    // Others shrink to make room: always 100% in all.
    let shares = shares(land(Biome::Grassland, 800, 4));
    assert_eq!(shares.iter().map(|&(_, share)| share).sum::<u64>(), 10_000);
    assert_eq!(shares[0].0, CreatureKind::Coo);
}

#[test]
fn coo_herds_live_in_cold_grassland() {
    let seed = Seed::new(11);
    let level = open_block();
    let cold = land(Biome::Grassland, 1500, 0);
    let herds: Vec<CreatureKind> = (0..300)
        .filter_map(|n| {
            let block = BlockPos { x: n, y: 9 };
            Some(wild_spawns(seed, block, &level, cold).first()?.kind)
        })
        .collect();
    let coos = herds
        .iter()
        .filter(|&&kind| kind == CreatureKind::Coo)
        .count();
    let share = coos * 100 / herds.len();
    assert!(
        (25..=55).contains(&share),
        "{share}% of {} herds",
        herds.len()
    );
}

#[test]
fn wild_ids_are_the_same_for_the_same_number_only() {
    let seed = Seed::new(7);
    assert_eq!(wild_mob_id(seed, 5), wild_mob_id(seed, 5));
    assert_ne!(wild_mob_id(seed, 5), wild_mob_id(seed, 6));
    assert_ne!(wild_mob_id(seed, 5), wild_mob_id(Seed::new(8), 5));
}
