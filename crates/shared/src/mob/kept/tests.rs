use super::*;
use crate::site::Site;

fn farm(seed: u64) -> SiteLevel {
    Site::new(PoiKind::Farm, Seed::new(seed)).level(0)
}

/// Kept animals start penned inside their pens and pastures (or by the
/// pond), one to a cell, of kinds kept in the land's biome.
#[test]
fn kept_animals_start_in_their_pens() {
    let land = Habitat::typical(Biome::Grassland);
    let mut seen = 0;
    for seed in 0..30 {
        let level = farm(seed);
        let mobs = site_spawns(Seed::new(seed), 0, PoiKind::Farm, &level, &[], land);
        let pens = level.fields.iter().any(|f| f.livestock);
        if !pens {
            continue;
        }
        for (i, mob) in mobs.iter().enumerate() {
            let (x, y) = mob.position;
            assert!(mob.penned);
            let in_field = level.fields.iter().any(|f| f.contains(x, y));
            assert!(
                in_field || level.level.cell(x, y) == Cell::Ground,
                "{mob:?}"
            );
            assert!(
                kept_weight(mob.kind, Biome::Grassland) > 0,
                "{:?}",
                mob.kind
            );
            assert!(mobs[..i].iter().all(|other| other.position != mob.position));
            seen += 1;
        }
    }
    assert!(seen > 60, "only {seen} kept animals on 30 farms");
}

/// Towns keep their beggars and add kept animals numbered past them.
#[test]
fn towns_keep_their_beggars() {
    let land = Habitat::typical(Biome::Woodland);
    let level = Site::new(PoiKind::Village, Seed::new(3)).level(0);
    let mobs = site_spawns(Seed::new(3), 0, PoiKind::Village, &level, &[], land);
    assert!(mobs.iter().any(|mob| mob.kind == CreatureKind::Beggar));
    for mob in mobs.iter().filter(|mob| mob.penned) {
        assert!(mob.number >= TOWN_NUMBERS);
    }
}

/// Where nothing is kept that a field asks for, it is left empty: no
/// reindeer on the savanna.
#[test]
fn only_what_is_kept_here() {
    assert_eq!(choose(&[Reindeer], Biome::Savanna, 7, Reindeer), None);
    assert_eq!(
        choose(&[Reindeer], Biome::Tundra, 7, Reindeer),
        Some(Reindeer)
    );
}
