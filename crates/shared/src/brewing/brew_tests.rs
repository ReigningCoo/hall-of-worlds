//! Brewing with each water, the potion threshold, quality and how long it
//! lasts, and ointment on the skin.

use super::*;
use crate::herb::{Effect, Herbs, Stat};
use crate::item::Quality;
use crate::potion::{Contents, Dose, Oil, Water};

fn contents(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

fn doses(contents: &Contents) -> Vec<(Stat, i16)> {
    contents.doses().iter().map(|d| (d.stat, d.step)).collect()
}

const S: Stat = Stat::Strength;
const A: Stat = Stat::Agility;
const W: Stat = Stat::Will;
const H: Stat = Stat::Health;

#[test]
fn plain_water_changes_nothing() {
    let mix = contents(&[(S, 5), (W, -3)]);
    assert_eq!(watered(&mix, Water::Plain), mix);
}

#[test]
fn cave_water_strengthens_harmful_doses() {
    let mix = contents(&[(S, 5), (W, -3), (H, -8)]);
    assert_eq!(
        doses(&watered(&mix, Water::Cave)),
        [(S, 5), (W, -4), (H, -10)]
    );
}

#[test]
fn moon_water_strengthens_helpful_doses() {
    let mix = contents(&[(S, 5), (W, -3), (H, 1)]);
    assert_eq!(
        doses(&watered(&mix, Water::Moon)),
        [(S, 6), (W, -3), (H, 2)]
    );
}

#[test]
fn salt_water_takes_the_strongest_helpful_dose() {
    let mix = contents(&[(S, 5), (A, 7), (W, -9)]);
    assert_eq!(doses(&watered(&mix, Water::Salt)), [(S, 5), (W, -9)]);
    // A tie: the first in `Stat`'s order goes.
    let tie = contents(&[(S, 5), (A, 5)]);
    assert_eq!(doses(&watered(&tie, Water::Salt)), [(A, 5)]);
    // Nothing helpful: nothing goes.
    let harmful = contents(&[(W, -5)]);
    assert_eq!(watered(&harmful, Water::Salt), harmful);
}

#[test]
fn rain_water_takes_the_strongest_harmful_dose() {
    let mix = contents(&[(S, 5), (W, -3), (H, -6)]);
    assert_eq!(doses(&watered(&mix, Water::Rain)), [(S, 5), (W, -3)]);
    // A tie: the first in `Stat`'s order goes.
    let tie = contents(&[(W, -4), (H, -4)]);
    assert_eq!(doses(&watered(&tie, Water::Rain)), [(H, -4)]);
    // Nothing harmful: nothing goes.
    let helpful = contents(&[(S, 5)]);
    assert_eq!(watered(&helpful, Water::Rain), helpful);
}

#[test]
fn a_potion_needs_a_dose_of_four() {
    assert_eq!(brew(&contents(&[(S, 3), (W, -3)]), Water::Plain), None);
    assert_eq!(brew(&Contents::EMPTY, Water::Moon), None);
    let made = brew(&contents(&[(S, 4), (A, 1), (W, -2)]), Water::Plain).unwrap();
    // Doses under 2 are dropped.
    assert_eq!(doses(&made), [(S, 4), (W, -2)]);
    let harmful = brew(&contents(&[(W, -4)]), Water::Plain).unwrap();
    assert_eq!(doses(&harmful), [(W, -4)]);
}

#[test]
fn the_water_counts_before_the_threshold() {
    let mix = contents(&[(S, 3), (W, -1)]);
    assert_eq!(brew(&mix, Water::Plain), None);
    assert_eq!(doses(&brew(&mix, Water::Moon).unwrap()), [(S, 4)]);
    // Cave Water makes the harmful dose 2, kept beside a strong one.
    let mix = contents(&[(S, 6), (W, -1)]);
    assert_eq!(doses(&brew(&mix, Water::Cave).unwrap()), [(S, 6), (W, -2)]);
    // Salt Water can leave nothing strong enough.
    let mix = contents(&[(S, 6), (W, -3)]);
    assert_eq!(brew(&mix, Water::Salt), None);
}

#[test]
fn quality_from_purity() {
    let q = |d: &[(Stat, i16)]| quality(&contents(d));
    assert_eq!(q(&[]), Quality::Poor);
    assert_eq!(q(&[(S, 8)]), Quality::Poor);
    assert_eq!(q(&[(S, 9)]), Quality::Common);
    assert_eq!(q(&[(S, 14)]), Quality::Common);
    assert_eq!(q(&[(S, 15)]), Quality::Uncommon);
    assert_eq!(q(&[(S, 21)]), Quality::Rare);
    assert_eq!(q(&[(S, 27)]), Quality::Epic);
    assert_eq!(q(&[(S, 33)]), Quality::Legendary);
    assert_eq!(q(&[(S, 44)]), Quality::Legendary);
    // The next strongest counts whichever way it goes.
    assert_eq!(q(&[(S, 20), (W, -11)]), Quality::Common);
    assert_eq!(q(&[(S, -20), (W, 2)]), Quality::Uncommon);
    assert_eq!(q(&[(S, 20), (A, 20)]), Quality::Poor);
}

#[test]
fn durations_by_quality() {
    let secs: Vec<u32> = Quality::ALL.map(duration_seconds).to_vec();
    assert_eq!(secs, [30, 60, 120, 240, 480, 960, 1920]);
    assert_eq!(duration_ticks(Quality::Common), 60 * 20);
}

#[test]
fn ointment_on_the_skin_is_a_quarter() {
    let mix = contents(&[(S, 9), (A, 3), (W, -5), (H, -4)]);
    assert_eq!(doses(&on_skin(&mix)), [(S, 2), (W, -1), (H, -1)]);
    assert!(on_skin(&Contents::EMPTY).is_empty());
}

#[test]
fn herbs_to_potion() {
    let mut def = Herbs::built_in().iter().next().unwrap().clone();
    def.key = "strong".into();
    def.effects = vec![
        Effect { stat: S, change: 3 },
        Effect {
            stat: W,
            change: -1,
        },
    ];
    let skills = BrewSkills {
        alchemy: 10_000,
        herbalism: 10_000,
    };
    let herb = GroundHerb::herb(&def, Quality::Artifact);
    let ointment = grind(&[herb], Oil::Grain, 5, skills).unwrap();
    // 6 boosted to 7; Will -2 cut away by Herbalism.
    assert_eq!(doses(&ointment.contents), [(S, 7)]);
    let potion = brew(&ointment.contents, Water::Moon).unwrap();
    assert_eq!(doses(&potion), [(S, 8)]);
    assert_eq!(quality(&potion), Quality::Poor);
}
