//! A herb's popup shows only what the character knows.

use shared::herb::{Herbs, new_item};
use shared::id::Id;
use shared::lang::{Lang, Language};

use super::*;

fn roseroot() -> Item {
    let herbs = Herbs::built_in();
    let def = herbs.get("roseroot").expect("roseroot");
    new_item(def, 3, Id::for_test(1), None)
}

#[test]
fn an_unknown_herb_shows_nothing_but_its_part() {
    let lang = Lang::new(Language::English);
    let words = herb_words(
        &roseroot(),
        &Herbs::built_in(),
        &HerbKnowledge::default(),
        &lang,
    )
    .expect("a herb");
    assert_eq!(words.name, "Unknown herb");
    assert_eq!(words.part.as_deref(), Some("Root"));
    assert_eq!(words.effects, vec!["?"; 4]);
}

#[test]
fn known_effects_are_worded_and_the_rest_stay_unknown() {
    let lang = Lang::new(Language::English);
    let mut known = HerbKnowledge::default();
    known.learn_name("roseroot");
    known.learn_effect("roseroot", 0);
    known.learn_effect("roseroot", 3);
    let words = herb_words(&roseroot(), &Herbs::built_in(), &known, &lang).expect("a herb");
    assert_eq!(words.name, "Roseroot");
    // Roseroot: Toughness +2, Will +2, Intellect +1, Empathy -1.
    assert_eq!(
        words.effects,
        ["Raises Toughness", "?", "?", "Slightly lowers Empathy"]
    );
}

#[test]
fn health_and_mana_and_strengths_are_worded() {
    let lang = Lang::new(Language::English);
    let effect = |stat, change| effect_words(&Effect { stat, change }, &lang);
    assert_eq!(effect(Stat::Health, 3), "Greatly raises Health");
    assert_eq!(effect(Stat::Mana, -2), "Lowers Mana");
    assert_eq!(effect(Stat::Aura, 1), "Slightly raises Aura");
}

#[test]
fn not_a_herb_says_nothing() {
    let lang = Lang::new(Language::English);
    let stone = Item::new(Id::for_test(2), shared::item::ItemKind::Stone);
    assert!(herb_words(&stone, &Herbs::built_in(), &HerbKnowledge::default(), &lang).is_none());
}
