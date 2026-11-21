use super::*;
use crate::flour::FlourForm;
use crate::fruit::Fruits;
use crate::fungus::Fungi;
use crate::herb::{HerbForm, Herbs};
use crate::item_form::ItemForm;
use crate::livestock::{Livestock, Parts};
use crate::vegetable::Vegetables;

fn table() -> &'static Ingredients {
    Ingredients::built_in()
}

fn missing<'a>(source: Source, keys: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    keys.filter(|key| table().get(source, key).is_none())
        .collect()
}

#[test]
fn every_herb_fruit_vegetable_and_fungus_has_a_row() {
    let herbs = Herbs::built_in();
    let herb_keys = herbs.iter().map(|def| def.key.as_str());
    assert_eq!(missing(Source::Herb, herb_keys), Vec::<&str>::new());
    let fruits = Fruits::built_in().iter().map(|def| def.key.as_str());
    assert_eq!(missing(Source::Fruit, fruits), Vec::<&str>::new());
    let vegetables = Vegetables::built_in().iter().map(|def| def.key.as_str());
    assert_eq!(missing(Source::Vegetable, vegetables), Vec::<&str>::new());
    let fungi = Fungi::built_in().iter().map(|def| def.key.as_str());
    assert_eq!(missing(Source::Fungus, fungi), Vec::<&str>::new());
}

#[test]
fn every_row_names_something_in_its_table() {
    let herbs = Herbs::built_in();
    let parts = Parts::built_in();
    let part_keys: Vec<&str> = (parts.organs.iter().map(|o| o.key.as_str()))
        .chain(parts.fats.iter().map(|f| f.key.as_str()))
        .chain(parts.others.iter().map(|p| p.key.as_str()))
        .collect();
    for def in table().iter() {
        let key = def.key.as_str();
        let found = match def.source {
            Source::Herb => herbs.get(key).is_some(),
            Source::Fruit => Fruits::built_in().get(key).is_some(),
            Source::Vegetable => Vegetables::built_in().get(key).is_some(),
            Source::Fungus => Fungi::built_in().get(key).is_some(),
            Source::Meat | Source::Milk | Source::Egg | Source::Honey => {
                Livestock::built_in().get(key).is_some()
            }
            Source::Part => part_keys.contains(&key),
        };
        assert!(found, "{:?} {key}", def.source);
    }
}

#[test]
fn the_research_is_all_there() {
    assert_eq!(table().len(), 468);
    assert_eq!(table().from_source(Source::Meat).count(), 25);
    assert_eq!(table().from_source(Source::Part).count(), 45);
}

#[test]
fn diet_flags_follow_the_owners_calls() {
    let oats = table().get(Source::Vegetable, "oats").unwrap();
    assert!(!oats.gluten);
    let coconut = table().get(Source::Fruit, "coconut").unwrap();
    assert!(!coconut.nut_allergen);
    assert!(table().get(Source::Fruit, "acorn").unwrap().nut_allergen);
    let beef = table().get(Source::Meat, "cattle").unwrap();
    assert!(beef.meat && beef.animal_product && !beef.dairy);
    let milk = table().get(Source::Milk, "cattle").unwrap();
    assert!(milk.dairy && milk.animal_product && !milk.meat);
}

#[test]
fn items_are_found_by_their_form() {
    let herb = Herbs::built_in().iter().next().unwrap().key.clone();
    let form = ItemForm::Herb(HerbForm {
        herb: herb.clone(),
        look: 0,
        grams: 1,
    });
    assert_eq!(table().of_form(&form).unwrap().key, herb);
    let rye = Vegetables::built_in().get("rye").unwrap();
    let flour = ItemForm::Flour(FlourForm::of(rye).unwrap());
    let found = table().of_form(&flour).unwrap();
    assert_eq!(
        (found.source, found.key.as_str()),
        (Source::Vegetable, "rye")
    );
}

#[test]
fn a_bad_row_is_refused() {
    let row = r#"[(source: Herb, key: "x", usage: Common, roles: [Herb], main: Some(Spice),
        cooking: Either, texture: Leafy, strength: 2, max_share: 5)]"#;
    assert!(Ingredients::read(row).is_err());
    let twice = r#"[
        (source: Herb, key: "x", usage: Common, roles: [Herb], cooking: Either, texture: Leafy, strength: 2, max_share: 5),
        (source: Herb, key: "x", usage: Common, roles: [Herb], cooking: Either, texture: Leafy, strength: 2, max_share: 5),
    ]"#;
    assert!(Ingredients::read(twice).is_err());
}
