//! Eating herbs raw, Give Item's herb, and learning or forgetting everything.

use super::*;
use crate::bonus::{self, Amount, Source};
use crate::item::Quality;
use crate::rng::Seed;
use crate::vitals::Vital;

fn herb(key: &str) -> HerbDef {
    Herbs::built_in().get(key).expect("a built-in herb").clone()
}

#[test]
fn a_raw_dose_is_the_first_effect_weak_and_short() {
    // Roseroot's first effect: Toughness +2.
    let def = herb("roseroot");
    let dose = raw_dose(&def, Quality::Common, 10_000, 100).expect("a dose");
    assert_eq!(dose.source, Source::RawHerb);
    assert_eq!((dose.start, dose.end), (100, Some(100 + RAW_TICKS)));
    let bonus = dose.bonuses[0];
    let toughness = bonus::Stat::Attribute(crate::sheet::Attribute::Toughness);
    assert_eq!(bonus.stat, toughness);
    assert_eq!(bonus.amount, Amount::Flat(2 * RAW_ATTRIBUTE));
}

#[test]
fn potency_scales_a_raw_dose() {
    let def = herb("roseroot");
    // No Herbalism: as weak as a poor one, whatever its quality (80%).
    let weak = raw_dose(&def, Quality::Artifact, 0, 0).expect("a dose");
    assert_eq!(
        weak.bonuses[0].amount,
        Amount::Flat(2 * RAW_ATTRIBUTE * 80 / 100)
    );
    let strong = raw_dose(&def, Quality::Artifact, 10_000, 0).expect("a dose");
    assert_eq!(
        strong.bonuses[0].amount,
        Amount::Flat(2 * RAW_ATTRIBUTE * 2)
    );
}

#[test]
fn health_and_mana_are_restored_or_drained() {
    let herbs = Herbs::built_in();
    for def in herbs.iter() {
        let dose = raw_dose(def, Quality::Common, 0, 0).expect("every herb has a dose");
        let first = def.effects[0];
        let expected = match (first.stat, first.raises()) {
            (Stat::Health, true) => Some(bonus::Stat::Restore(Vital::Health)),
            (Stat::Health, false) => Some(bonus::Stat::Drain(Vital::Health)),
            (Stat::Mana, true) => Some(bonus::Stat::Restore(Vital::Mana)),
            (Stat::Mana, false) => Some(bonus::Stat::Drain(Vital::Mana)),
            _ => None,
        };
        if let Some(expected) = expected {
            assert_eq!(dose.bonuses[0].stat, expected, "{}", def.key);
            assert!(matches!(dose.bonuses[0].amount, Amount::Flat(milli) if milli > 0));
        }
    }
}

#[test]
fn give_items_herb_is_the_one_named_or_rolled_from_the_seed() {
    let herbs = Herbs::built_in();
    assert_eq!(
        dev_herb(&herbs, Some("roseroot"), Seed::new(7)).map(|d| &d.key[..]),
        Some("roseroot")
    );
    assert!(dev_herb(&herbs, Some("no_such_herb"), Seed::new(7)).is_none());
    let rolled = dev_herb(&herbs, None, Seed::new(7)).expect("one is rolled");
    assert_eq!(
        dev_herb(&herbs, None, Seed::new(7)),
        Some(rolled),
        "the same every time"
    );
    let many: std::collections::HashSet<_> = (0..200)
        .filter_map(|seed| dev_herb(&herbs, None, Seed::new(seed)))
        .map(|d| &d.key)
        .collect();
    assert!(many.len() > 20, "seeds roll many herbs");
}

#[test]
fn learning_all_of_a_herb_and_forgetting_everything() {
    let def = herb("roseroot");
    let mut known = HerbKnowledge::default();
    assert!(known.learn_all("roseroot"));
    assert!(!known.learn_all("roseroot"), "nothing new the second time");
    assert!(known.knows_name("roseroot"));
    assert_eq!(known.known_effects(&def).count(), EFFECTS);
    known.forget_all();
    assert!(known.is_empty());
}
