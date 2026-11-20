//! Grinding: limits from skills, steps per herb, adding up, the oil's row,
//! the caps and Herbalism's cuts.

use super::*;
use crate::herb::{Effect, HerbDef, Herbs, Stat};
use crate::item::Quality;
use crate::potion::{Contents, Oil};

/// Skills at whole percents.
fn skills(alchemy: u32, herbalism: u32) -> BrewSkills {
    BrewSkills {
        alchemy: alchemy * 100,
        herbalism: herbalism * 100,
    }
}

/// A built-in herb given these effects and a key of its own.
fn herb(key: &str, effects: &[(Stat, i8)]) -> HerbDef {
    let mut def = Herbs::built_in().iter().next().unwrap().clone();
    def.key = key.into();
    def.effects = effects
        .iter()
        .map(|&(stat, change)| Effect { stat, change })
        .collect();
    def
}

fn common(def: &HerbDef) -> GroundHerb<'_> {
    GroundHerb::herb(def, Quality::Common)
}

fn doses(contents: &Contents) -> Vec<(Stat, i16)> {
    contents.doses().iter().map(|d| (d.stat, d.step)).collect()
}

#[test]
fn whole_roots() {
    let roots: Vec<u32> = [0, 1, 3, 4, 8, 9, 99, 100].map(square_root).to_vec();
    assert_eq!(roots, [0, 1, 1, 2, 2, 3, 9, 10]);
}

#[test]
fn limits_grow_with_alchemy() {
    let none = skills(0, 0);
    let full = skills(100, 100);
    assert_eq!((per_herb_cap(none), per_herb_cap(full)), (1, 11));
    assert_eq!((stat_cap(none), stat_cap(full)), (4, 44));
    assert_eq!((most_kinds(none), most_kinds(full)), (4, 8));
    assert_eq!((most_doses(none), most_doses(full)), (1, 5));
    assert_eq!((cut_steps(none), cut_steps(full)), (0, 4));
    assert_eq!(most_doses(skills(27, 0)), 4);
    assert_eq!(per_herb_cap(skills(25, 0)), 6);
}

#[test]
fn skills_above_full_count_as_full() {
    let over = BrewSkills {
        alchemy: 50_000,
        herbalism: 50_000,
    };
    assert_eq!(per_herb_cap(over), 11);
    assert_eq!(cut_steps(over), 4);
}

#[test]
fn herb_steps_scale_by_potency_and_cap() {
    let up = |change| Effect {
        stat: Stat::Strength,
        change,
    };
    let full = skills(100, 100);
    // Untrained: capped at 1 step a herb.
    assert_eq!(herb_steps(up(1), Quality::Legendary, skills(0, 0)), 1);
    assert_eq!(herb_steps(up(3), Quality::Common, skills(0, 100)), 1);
    // Trained: 3 × 200% = 6; 2 × 115% = 2.3 → 2; 1 × 175% = 1.75 → 2.
    assert_eq!(herb_steps(up(3), Quality::Artifact, full), 6);
    assert_eq!(herb_steps(up(2), Quality::Uncommon, full), 2);
    assert_eq!(herb_steps(up(1), Quality::Legendary, full), 2);
    assert_eq!(herb_steps(up(-3), Quality::Artifact, full), -6);
    // A poor herb's weakest effect still counts: 0.8 → 1.
    assert_eq!(herb_steps(up(1), Quality::Poor, full), 1);
    // Herbalism caps quality: Artifact with none is as weak as Poor (2.4 → 2).
    assert_eq!(herb_steps(up(3), Quality::Artifact, skills(100, 0)), 2);
}

#[test]
fn refusals() {
    let a = herb("a", &[(Stat::Strength, 1)]);
    let b = herb("b", &[(Stat::Agility, 1)]);
    let none = skills(0, 0);
    assert_eq!(grind(&[], Oil::Nut, 1, none), Err(Refused::NoHerbs));
    let twice = [common(&a), common(&a)];
    assert_eq!(grind(&twice, Oil::Nut, 1, none), Err(Refused::SameHerb));
    assert_eq!(grind(&[common(&a)], Oil::Nut, 0, none), Err(Refused::Doses));
    assert_eq!(grind(&[common(&a)], Oil::Nut, 2, none), Err(Refused::Doses));
    assert!(grind(&[common(&a), common(&b)], Oil::Nut, 5, skills(100, 0)).is_ok());
    assert_eq!(
        grind(&[common(&a)], Oil::Nut, 6, skills(100, 0)),
        Err(Refused::Doses)
    );
}

#[test]
fn the_same_herb_at_two_qualities_is_still_the_same_herb() {
    let a = herb("a", &[(Stat::Strength, 1)]);
    let mix = [common(&a), GroundHerb::herb(&a, Quality::Rare)];
    assert_eq!(
        grind(&mix, Oil::Nut, 1, skills(0, 0)),
        Err(Refused::SameHerb)
    );
}

#[test]
fn kinds_of_herb_limited_by_alchemy() {
    let defs: Vec<HerbDef> = (0..9)
        .map(|i| herb(&format!("h{i}"), &[(Stat::Will, 1)]))
        .collect();
    let mix: Vec<GroundHerb> = defs.iter().map(common).collect();
    assert!(grind(&mix[..4], Oil::Seed, 1, skills(0, 0)).is_ok());
    assert_eq!(
        grind(&mix[..5], Oil::Seed, 1, skills(0, 0)),
        Err(Refused::TooManyKinds)
    );
    assert!(grind(&mix[..8], Oil::Seed, 1, skills(100, 0)).is_ok());
    assert_eq!(
        grind(&mix, Oil::Seed, 1, skills(100, 0)),
        Err(Refused::TooManyKinds)
    );
}

#[test]
fn effects_add_and_opposites_cancel() {
    let a = herb(
        "a",
        &[(Stat::Strength, 3), (Stat::Will, 2), (Stat::Aura, -1)],
    );
    let b = herb(
        "b",
        &[(Stat::Strength, 2), (Stat::Will, -2), (Stat::Aura, -2)],
    );
    let made = grind(&[common(&a), common(&b)], Oil::Seed, 3, skills(100, 100)).unwrap();
    assert_eq!(made.doses, 3);
    // Strength 3 + 2; Will cancels; Aura -3 boosted by Seed Oil to -4, then
    // Herbalism's 4 steps take it away.
    assert_eq!(doses(&made.contents), [(Stat::Strength, 5)]);
}

#[test]
fn oil_boosts_its_row_only() {
    let a = herb(
        "a",
        &[(Stat::Health, 3), (Stat::Mana, 3), (Stat::Empathy, 3)],
    );
    let full = skills(100, 100);
    let by = |oil| doses(&grind(&[common(&a)], oil, 1, full).unwrap().contents);
    let h = Stat::Health;
    let m = Stat::Mana;
    let e = Stat::Empathy;
    assert_eq!(by(Oil::Neutral), [(e, 3), (h, 3), (m, 3)]);
    assert_eq!(by(Oil::Grain), [(e, 3), (h, 4), (m, 3)]);
    assert_eq!(by(Oil::Nut), [(e, 3), (h, 3), (m, 4)]);
    assert_eq!(by(Oil::Seed), [(e, 4), (h, 3), (m, 3)]);
}

#[test]
fn boost_is_a_quarter_at_least_one() {
    let got: Vec<i32> = [0, 1, 3, 4, 8, 11, -1, -8].map(boosted).to_vec();
    assert_eq!(got, [0, 2, 4, 5, 10, 13, -2, -10]);
}

#[test]
fn rows_of_stats_and_oils() {
    use crate::sheet::Row;
    assert_eq!(stat_row(Stat::Health), Row::Body);
    assert_eq!(stat_row(Stat::Mana), Row::Mind);
    assert_eq!(stat_row(Stat::Toughness), Row::Body);
    assert_eq!(stat_row(Stat::Perception), Row::Mind);
    assert_eq!(stat_row(Stat::Aura), Row::Spirit);
    let rows = Oil::ALL.map(oil_suits);
    assert_eq!(
        rows,
        [None, Some(Row::Body), Some(Row::Mind), Some(Row::Spirit)]
    );
}

#[test]
fn each_stat_capped() {
    let defs: Vec<HerbDef> = (0..4)
        .map(|i| herb(&format!("h{i}"), &[(Stat::Strength, 3)]))
        .collect();
    let mix: Vec<GroundHerb> = defs.iter().map(common).collect();
    // Untrained: each herb gives 1, four of them 4, Grain Oil makes 5,
    // capped at 4.
    let made = grind(&mix, Oil::Grain, 1, skills(0, 0)).unwrap();
    assert_eq!(doses(&made.contents), [(Stat::Strength, 4)]);
    // At 25% Alchemy: each 3 (cap 6), 12, boosted 15, cap 24.
    let made = grind(&mix, Oil::Grain, 1, skills(25, 100)).unwrap();
    assert_eq!(doses(&made.contents), [(Stat::Strength, 15)]);
}

#[test]
fn harmful_totals_are_capped_too() {
    let defs: Vec<HerbDef> = (0..4)
        .map(|i| herb(&format!("h{i}"), &[(Stat::Will, -3)]))
        .collect();
    let mix: Vec<GroundHerb> = defs.iter().map(common).collect();
    let made = grind(&mix, Oil::Nut, 1, skills(0, 0)).unwrap();
    assert_eq!(doses(&made.contents), [(Stat::Will, -4)]);
}

#[test]
fn herbalism_cuts_round_robin_from_the_largest() {
    let mut doses = vec![
        (Stat::Strength, 9),
        (Stat::Agility, -3),
        (Stat::Will, -1),
        (Stat::Aura, -2),
        (Stat::Mana, 2),
    ];
    cut_unwanted(&mut doses, 5);
    // Round 1: Agility, Aura, Will (to 0); round 2: Agility, Aura.
    assert_eq!(
        doses,
        [
            (Stat::Strength, 9),
            (Stat::Agility, -1),
            (Stat::Will, 0),
            (Stat::Aura, 0),
            (Stat::Mana, 2),
        ]
    );
}

#[test]
fn harmful_can_be_the_wanted_way() {
    let mut doses = vec![(Stat::Strength, 2), (Stat::Will, -6), (Stat::Aura, 1)];
    cut_unwanted(&mut doses, 2);
    assert_eq!(
        doses,
        [(Stat::Strength, 1), (Stat::Will, -6), (Stat::Aura, 0)]
    );
}

#[test]
fn cuts_stop_when_nothing_is_unwanted() {
    let mut doses = vec![(Stat::Strength, 5), (Stat::Will, -1)];
    cut_unwanted(&mut doses, 4);
    assert_eq!(doses, [(Stat::Strength, 5), (Stat::Will, 0)]);
    // A tie between ways counts as helpful.
    let mut tie = vec![(Stat::Strength, -3), (Stat::Will, 3)];
    cut_unwanted(&mut tie, 1);
    assert_eq!(tie, [(Stat::Strength, -2), (Stat::Will, 3)]);
    cut_unwanted(&mut [], 3);
}

#[test]
fn everything_cancelling_makes_empty_contents() {
    let a = herb("a", &[(Stat::Strength, 2)]);
    let b = herb("b", &[(Stat::Strength, -2)]);
    let made = grind(&[common(&a), common(&b)], Oil::Nut, 1, skills(100, 0)).unwrap();
    assert!(made.contents.is_empty());
}

#[test]
fn every_built_in_herb_grinds_within_the_caps() {
    let herbs = Herbs::built_in();
    let full = skills(100, 100);
    for def in herbs.iter() {
        for quality in Quality::ALL {
            for oil in Oil::ALL {
                let made = grind(&[GroundHerb::herb(def, quality)], oil, 5, full).unwrap();
                for dose in made.contents.doses() {
                    assert!(dose.step.abs() <= 44, "{} {dose:?}", def.key);
                }
            }
        }
    }
}

#[test]
fn grinding_ignores_the_order_of_herbs() {
    let a = herb("a", &[(Stat::Strength, 3), (Stat::Will, -1)]);
    let b = herb("b", &[(Stat::Agility, 2), (Stat::Aura, -2)]);
    let s = skills(64, 30);
    let ab = grind(&[common(&a), common(&b)], Oil::Nut, 2, s).unwrap();
    let ba = grind(&[common(&b), common(&a)], Oil::Nut, 2, s).unwrap();
    assert_eq!(ab, ba);
}
