use super::*;
use crate::creature::CreatureKind;
use crate::dungeon::Level;
use crate::livestock::{Stage, stage_of};
use crate::sheet::Trait;

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

/// Open ground with a pond in the middle.
fn pond_block() -> Level {
    let mut level = open_block();
    for y in 20..44 {
        for x in 20..44 {
            level.set(x, y, Cell::Water);
        }
    }
    level
}

fn land(biome: Biome, metres: i32, celsius: i32) -> Habitat {
    Habitat {
        biome,
        metres,
        celsius,
    }
}

/// The herds of `count` blocks in a row.
fn herds(seed: u64, level: &Level, land: Habitat, count: i64) -> Vec<Vec<Spawn>> {
    (0..count)
        .map(|n| wild_spawns(Seed::new(seed), BlockPos { x: n, y: 3 }, level, land))
        .filter(|herd| !herd.is_empty())
        .collect()
}

#[test]
fn blocks_have_one_kind_each_and_the_same_every_time() {
    let seed = Seed::new(7);
    let level = open_block();
    let boreal = Habitat::typical(Biome::BorealForest);
    let mut lived = 0;
    for n in 0..60 {
        let block = BlockPos { x: n, y: 3 };
        let herd = wild_spawns(seed, block, &level, boreal);
        assert_eq!(herd, wild_spawns(seed, block, &level, boreal));
        assert!(herd.len() <= (MOST_TOGETHER + MOST_YOUNG) as usize);
        let kind = herd.first().map(|mob| mob.kind);
        assert!(herd.iter().all(|mob| Some(mob.kind) == kind));
        lived += usize::from(!herd.is_empty());
    }
    assert!((20..=45).contains(&lived), "{lived} of 60 blocks");
}

#[test]
fn wolves_still_rule_the_forests_and_deserts_are_sparse() {
    let level = open_block();
    let forest = herds(5, &level, Habitat::typical(Biome::BorealForest), 300);
    let wolves = forest
        .iter()
        .filter(|herd| herd[0].kind == CreatureKind::Wolf)
        .count();
    assert!(
        wolves * 100 / forest.len() >= 30,
        "{wolves} of {}",
        forest.len()
    );
    let desert = herds(5, &level, Habitat::typical(Biome::Desert), 300);
    assert!((20..=120).contains(&desert.len()), "{}", desert.len());
    assert!(
        desert
            .iter()
            .flatten()
            .all(|mob| mob.kind != CreatureKind::Wolf)
    );
}

#[test]
fn water_birds_only_by_water() {
    let grass = land(Biome::Grassland, 0, 10);
    let water_bird = |kind| {
        matches!(
            kind,
            CreatureKind::Duck | CreatureKind::MuscovyDuck | CreatureKind::Goose
        )
    };
    let dry = herds(9, &open_block(), grass, 300);
    assert!(dry.iter().flatten().all(|mob| !water_bird(mob.kind)));
    let wet = herds(9, &pond_block(), grass, 300);
    let birds = wet.iter().filter(|herd| water_bird(herd[0].kind)).count();
    assert!(birds > 10, "{birds} of {}", wet.len());
}

#[test]
fn herds_with_young_have_a_grown_mother_while_they_are_small() {
    let level = open_block();
    let big: Vec<Vec<Spawn>> = herds(13, &level, land(Biome::Grassland, 0, 10), 600)
        .into_iter()
        .filter(|herd| herd.len() > MOST_TOGETHER as usize)
        .collect();
    assert!(big.len() > 5, "{} herds with young", big.len());
    let seed = Seed::new(13);
    let mut young_seen = 0;
    for herd in &big {
        assert_eq!(herd[0].gender, Gender::Female);
        let mother = wild_mob_id(seed, herd[0].number);
        for hours in (0..20_000u64).step_by(97) {
            let minutes = hours * 60;
            let stage = |mob: &Spawn| stage_of(mob.kind, wild_mob_id(seed, mob.number), minutes);
            // The last is one of the young: while it is small, its mother is grown.
            if stage(&herd[herd.len() - 1]) == Stage::Young {
                young_seen += 1;
                assert_eq!(stage_of(herd[0].kind, mother, minutes), Stage::Adult);
            }
            // Never a herd of only small ones.
            if herd.iter().any(|mob| stage(mob) == Stage::Young) {
                assert!(herd.iter().any(|mob| stage(mob) == Stage::Adult));
            }
        }
    }
    assert!(young_seen > 0);
}

#[test]
fn aurochs_are_rare_and_look_it() {
    let level = open_block();
    let mut aurochs = 0;
    let mut cattle = 0;
    for herd in herds(17, &level, land(Biome::Grassland, 0, 10), 6000) {
        for mob in herd.iter().filter(|mob| mob.kind == CreatureKind::Cattle) {
            cattle += 1;
            let coat = mob.appearance.choice(mob.kind, Trait::CoatColour).unwrap();
            if coat.name == "Aurochs" {
                aurochs += 1;
                assert_ne!(mob.gender, Gender::Neutral);
            }
        }
    }
    assert!(aurochs > 0 && aurochs < cattle, "{aurochs} of {cattle}");
}

#[test]
fn coo_herds_live_in_cold_grassland() {
    let level = open_block();
    let cold = land(Biome::Grassland, 1500, 0);
    let herds = herds(11, &level, cold, 300);
    let coos = herds
        .iter()
        .filter(|herd| herd[0].kind == CreatureKind::Coo)
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
