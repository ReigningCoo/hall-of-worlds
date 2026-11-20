//! A fungus is named only as far as the character knows it.

use shared::fungus::{Fungi, FungusKnowledge, new_item};
use shared::id::Id;
use shared::lang::{Lang, Language};

use super::*;

fn name(key: &str, known: &FungusKnowledge) -> String {
    let lang = Lang::new(Language::English);
    fungus_name(Fungi::built_in().get(key).unwrap(), known, &lang)
}

#[test]
fn unknown_fungi_are_never_named() {
    let none = FungusKnowledge::default();
    assert_eq!(name("death_cap", &none), "Unknown mushroom");
    assert_eq!(name("manna_lichen", &none), "Unknown lichen");
    let lang = Lang::new(Language::English);
    assert_eq!(fungus_key_name("no_such", &none, &lang), "Fungus");
}

#[test]
fn a_known_lookalike_shows_as_looks_like() {
    let mut known = FungusKnowledge::default();
    known.learn_name("straw_mushroom");
    assert_eq!(name("death_cap", &known), "Looks like Straw mushroom");
    // The straw mushroom itself is known by name.
    assert_eq!(name("straw_mushroom", &known), "Straw mushroom");
    known.learn_name("death_cap");
    assert_eq!(name("death_cap", &known), "Death cap");
}

#[test]
fn items_are_named_by_what_is_known() {
    let lang = Lang::new(Language::English);
    let def = Fungi::built_in().get("morel").unwrap();
    let item = new_item(def, 2, Id::for_test(1), None);
    let mut known = FungusKnowledge::default();
    assert_eq!(item_name(&item, &known, &lang), "Unknown mushroom");
    known.learn_name("morel");
    assert_eq!(item_name(&item, &known, &lang), "Morel");
}
