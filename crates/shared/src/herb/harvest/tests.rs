use super::*;
use crate::herb::{Abundance, EARLY_STAGE, HerbDef, Herbs};
use crate::rng::Seed;

fn def(taken: u8) -> HerbDef {
    let mut def = Herbs::built_in().iter().next().unwrap().clone();
    def.early.taken = taken;
    def
}

/// Hundredths of a percent, as `skill_value` gives.
fn percent(n: u32) -> u32 {
    n * 100
}

/// Every number a full-grown patch gave over many picks.
fn counts(skills: (u32, u32)) -> Vec<u32> {
    let mut counts = std::collections::BTreeSet::new();
    for n in 0..500 {
        let mut rng = Seed::new(n).rng();
        counts.insert(harvest(&def(50), STAGES, skills, &mut rng).unwrap().count);
    }
    counts.into_iter().collect()
}

#[test]
fn the_owners_examples_hold() {
    assert_eq!(harvest_range(0, 0), (1, 3));
    assert_eq!(harvest_range(percent(100), 0), (1, 7));
    assert_eq!(harvest_range(0, percent(100)), (5, 7));
    assert_eq!(harvest_range(percent(100), percent(100)), (5, 11));
    // A little skill already counts: 1% adds one, 8% two, 27% three.
    assert_eq!(harvest_range(percent(1), 0), (1, 4));
    assert_eq!(harvest_range(percent(8), percent(8)), (3, 7));
    assert_eq!(harvest_range(percent(27), 0), (1, 6));
    assert_eq!(harvest_range(99, 0), (1, 3), "under 1% adds nothing");
    assert_eq!(harvest_range(percent(200), 0), (1, 7), "100% at most");
}

#[test]
fn every_number_in_the_range_comes_up() {
    assert_eq!(counts((0, 0)), [1, 2, 3]);
    assert_eq!(counts((0, percent(100))), [5, 6, 7]);
    assert_eq!(
        counts((percent(100), percent(100))),
        (5..=11).collect::<Vec<_>>()
    );
}

#[test]
fn cube_roots_are_whole_and_rounded_down() {
    let roots: Vec<u32> = [0, 1, 7, 8, 26, 27, 63, 64, 100].map(cube_root).into();
    assert_eq!(roots, [0, 1, 1, 2, 2, 3, 3, 4, 4]);
}

/// Early, the herb's share of the number rolled, or nothing.
#[test]
fn early_harvests_give_their_share() {
    for n in 0..200 {
        let skills = (percent(100), percent(100));
        let mut rng = Seed::new(n).rng();
        let full = harvest(&def(50), STAGES, skills, &mut rng).unwrap();
        let mut rng = Seed::new(n).rng();
        let early = harvest(&def(50), EARLY_STAGE, skills, &mut rng).unwrap();
        assert_eq!(early.count, (full.count * 50 + 50) / 100);
        let mut rng = Seed::new(n).rng();
        assert_eq!(harvest(&def(0), EARLY_STAGE, skills, &mut rng), None);
    }
}

/// Quality is the plant's rarity, early or not, whatever the skill.
#[test]
fn quality_is_the_plants_rarity() {
    for (abundance, quality) in [
        (Abundance::Common, Quality::Common),
        (Abundance::Uncommon, Quality::Uncommon),
        (Abundance::Rare, Quality::Rare),
    ] {
        let mut def = def(100);
        def.abundance = abundance;
        for stage in [EARLY_STAGE, STAGES] {
            for skills in [(0, 0), (percent(100), percent(100))] {
                let mut rng = Seed::new(3).rng();
                let got = harvest(&def, stage, skills, &mut rng).unwrap();
                assert_eq!(got.quality, quality);
            }
        }
    }
}
