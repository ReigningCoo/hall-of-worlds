//! Drinking: an effect per attribute, Health and Mana at once, cooldowns.

use super::*;
use crate::bonus::Source;
use crate::herb::Stat;
use crate::item::Quality;
use crate::potion::{Contents, Dose};
use crate::vitals::{PER_POINT, Vital};

fn contents(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

#[test]
fn each_attribute_is_its_own_effect_and_vitals_come_at_once() {
    let potion = contents(&[(Stat::Strength, 4), (Stat::Will, -2), (Stat::Health, 3)]);
    let drunk = drunk(&potion, Quality::Common, 100);
    let sources: Vec<_> = drunk.effects.iter().map(|it| it.source).collect();
    assert_eq!(
        sources,
        [Source::Potion(Stat::Strength), Source::Potion(Stat::Will)]
    );
    for effect in &drunk.effects {
        assert_eq!(effect.end, Some(100 + duration_ticks(Quality::Common)));
    }
    let health = 3 * VITAL_PER_STEP * PER_POINT as i32;
    assert_eq!(drunk.vitals, [(Vital::Health, health)]);
}

#[test]
fn better_potions_last_longer() {
    let potion = contents(&[(Stat::Agility, 4)]);
    let poor = drunk(&potion, Quality::Poor, 0).effects[0].end;
    let rare = drunk(&potion, Quality::Rare, 0).effects[0].end;
    assert_eq!(poor, Some(duration_ticks(Quality::Poor)));
    assert!(rare > poor);
}

#[test]
fn a_new_dose_replaces_only_its_own_stat() {
    let strength = Source::Potion(Stat::Strength);
    assert!(strength.replaced_by(Source::Potion(Stat::Strength)));
    assert!(!strength.replaced_by(Source::Potion(Stat::Agility)));
    assert!(!strength.replaced_by(Source::Ointment));
}

#[test]
fn the_same_stats_and_signs_wait_whatever_the_strength() {
    let weak = Combination::of(&contents(&[(Stat::Strength, 4), (Stat::Mana, 2)]));
    let strong = Combination::of(&contents(&[(Stat::Strength, 9), (Stat::Mana, 5)]));
    let harmful = Combination::of(&contents(&[(Stat::Strength, 4), (Stat::Mana, -2)]));
    assert_eq!(weak, strong);
    assert_ne!(weak, harmful);
    let mut drinks = Cooldowns::default();
    drinks.drank(weak, 10);
    assert!(!drinks.ready(strong, 11));
    assert!(drinks.ready(harmful, 11));
    assert_eq!(drinks.ready_at(strong, 11), Some(10 + DRINK_COOLDOWN_TICKS));
    assert!(drinks.ready(strong, 10 + DRINK_COOLDOWN_TICKS));
    // Drinking another forgets cooldowns that are over.
    drinks.drank(harmful, 10 + DRINK_COOLDOWN_TICKS);
    assert_eq!(drinks, {
        let mut only = Cooldowns::default();
        only.drank(harmful, 10 + DRINK_COOLDOWN_TICKS);
        only
    });
}
