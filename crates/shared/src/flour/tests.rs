use super::*;
use crate::id::Id;
use crate::item::{ItemKind, StackLimits};
use crate::rng::Seed;
use crate::vegetable::Vegetables;

fn grain(key: &str) -> &'static crate::vegetable::VegetableDef {
    Vegetables::built_in().get(key).expect("a built-in grain")
}

#[test]
fn every_grain_mills_into_its_own_flour() {
    let keys: Vec<_> = grains().map(|def| def.key.as_str()).collect();
    assert_eq!(keys.len(), 17);
    for def in grains() {
        let flour = FlourForm::of(def).expect("grain mills");
        assert_eq!(flour.grain, def.key);
        assert!(flour.grams > 0 && flour.grams <= def.grams, "{}", def.key);
    }
}

#[test]
fn a_sheaf_mills_by_its_yield() {
    // 30 g of rye at 95%, of oats at 65%.
    assert_eq!(FlourForm::of(grain("rye")).unwrap().grams, 29);
    assert_eq!(FlourForm::of(grain("oats")).unwrap().grams, 20);
}

#[test]
fn only_grain_mills() {
    assert!(FlourForm::of(grain("carrot")).is_none());
    assert!(new_item(grain("carrot"), 1, Id::stand_in(1), None).is_none());
}

#[test]
fn named_as_a_sack() {
    let flour = FlourForm::of(grain("rye")).unwrap();
    assert_eq!(flour.name(), "Sack of Rye Flour");
    let unknown = FlourForm {
        grain: "nothing".to_owned(),
        grams: 1,
    };
    assert_eq!(unknown.name(), "Sack of Flour");
}

#[test]
fn sacks_stack_by_grain_without_quality() {
    let rye = new_item(grain("rye"), 1, Id::stand_in(1), None).unwrap();
    let more_rye = new_item(grain("rye"), 2, Id::stand_in(2), None).unwrap();
    let wheat = new_item(grain("wheat"), 1, Id::stand_in(3), None).unwrap();
    assert_eq!(rye.kind, ItemKind::Flour);
    assert!(rye.quality.is_none());
    assert!(rye.flour().is_some());
    assert!(rye.looks_like(&more_rye));
    assert!(!rye.looks_like(&wheat));
    assert!(StackLimits::default().room(&rye, &more_rye) > 0);
}

#[test]
fn give_item_chooses_alike_on_both_sides() {
    let seed = Seed::new(7);
    assert_eq!(dev_flour(None, seed), dev_flour(None, seed));
    let rye = dev_flour(Some("rye"), seed).map(|def| def.key.as_str());
    assert_eq!(rye, Some("rye"));
    assert!(dev_flour(Some("carrot"), seed).is_none());
}
