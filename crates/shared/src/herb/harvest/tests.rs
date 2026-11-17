use super::*;
use crate::herb::{EARLY_STAGE, HerbDef, Herbs};
use crate::rng::Seed;

fn def(taken: u8, strength: u8) -> HerbDef {
    let mut def = Herbs::built_in().iter().next().unwrap().clone();
    def.early.taken = taken;
    def.early.strength = strength;
    def
}

/// Full grown gives 2 to 4; early, the herb's share of that, or nothing.
#[test]
fn early_harvests_give_their_share() {
    let mut counts = std::collections::BTreeSet::new();
    for n in 0..200 {
        let mut rng = Seed::new(n).rng();
        let full = harvest(&def(50, 50), STAGES, 0, &mut rng).unwrap();
        counts.insert(full.count);
        let mut rng = Seed::new(n).rng();
        let early = harvest(&def(50, 50), EARLY_STAGE, 0, &mut rng).unwrap();
        assert_eq!(early.count, (full.count * 50 + 50) / 100);
        let mut rng = Seed::new(n).rng();
        assert_eq!(harvest(&def(0, 100), EARLY_STAGE, 0, &mut rng), None);
    }
    assert_eq!(counts.into_iter().collect::<Vec<_>>(), [2, 3, 4]);
}

/// Early harvests are weaker: as if picked with less Foraging.
#[test]
fn early_harvests_are_poorer() {
    let better = |stage: u8| {
        (0..2000)
            .filter(|&n| {
                let mut rng = Seed::new(n).rng();
                let got = harvest(&def(100, 0), stage, 10_000, &mut rng).unwrap();
                got.quality > Quality::Poor
            })
            .count()
    };
    assert!(better(STAGES) > better(EARLY_STAGE) + 300);
}
