use super::*;
use crate::climate::Biome;
use crate::cooking::{Ingredients, Source};
use crate::id::Id;
use crate::item::{ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::livestock::{Livestock, Product};
use crate::potion::{BottleForm, Liquid, LiquidKind};
use crate::rng::Seed;
use crate::vessel::{VesselForm, VesselKind};

fn animal(key: &str) -> &'static crate::livestock::LivestockDef {
    Livestock::built_in().get(key).expect("a built-in animal")
}

#[test]
fn the_givers_are_the_tables() {
    let keys = |product| {
        givers(product)
            .map(|def| def.key.as_str())
            .collect::<Vec<_>>()
    };
    assert_eq!(keys(Product::Milk).len(), 11);
    assert_eq!(keys(Product::Egg).len(), 7);
    assert_eq!(keys(Product::Honey), ["honeybee"]);
    assert!(keys(Product::Egg).contains(&"chicken"));
    assert!(!keys(Product::Milk).contains(&"pig"));
}

#[test]
fn every_giver_is_an_ingredient_and_every_ingredient_a_giver() {
    let table = Ingredients::built_in();
    for (product, source) in [
        (Product::Milk, Source::Milk),
        (Product::Egg, Source::Egg),
        (Product::Honey, Source::Honey),
    ] {
        for def in givers(product) {
            assert!(
                table.get(source, &def.key).is_some(),
                "{source:?} {}",
                def.key
            );
        }
        for row in table.from_source(source) {
            assert!(animal(&row.key).gives(product), "{source:?} {}", row.key);
        }
    }
}

#[test]
fn named_by_their_animal() {
    assert_eq!(milk_name("cattle"), "Cow's Milk");
    assert_eq!(milk_name("yak"), "Yak Milk");
    assert_eq!(milk_name("nothing"), "Milk");
    assert_eq!(egg_name("chicken"), "Hen's Egg");
    assert_eq!(egg_name("muscovy_duck"), "Muscovy Duck Egg");
    assert_eq!(honey_name("honeybee"), "Honey");
    assert_eq!(Liquid::Milk("goat".to_owned()).name(), "Goat's Milk");
    assert_eq!(Liquid::Honey("honeybee".to_owned()).name(), "Honey");
}

#[test]
fn eggs_stack_by_animal_without_quality() {
    let hen = new_egg(animal("chicken"), 1, Id::stand_in(1), None).unwrap();
    let more = new_egg(animal("chicken"), 5, Id::stand_in(2), None).unwrap();
    let goose = new_egg(animal("goose"), 1, Id::stand_in(3), None).unwrap();
    assert_eq!(hen.kind, ItemKind::Egg);
    assert!(hen.quality.is_none());
    assert_eq!(hen.egg().unwrap().grams, 55);
    assert_eq!(goose.egg().unwrap().grams, 145);
    assert!(hen.looks_like(&more));
    assert!(!hen.looks_like(&goose));
    assert!(StackLimits::default().room(&hen, &more) > 0);
    assert!(new_egg(animal("cattle"), 1, Id::stand_in(4), None).is_none());
}

#[test]
fn give_item_chooses_alike_on_both_sides() {
    let seed = Seed::new(7);
    let rolled = |product| dev_animal(product, None, seed).map(|def| &def.key);
    assert_eq!(rolled(Product::Egg), rolled(Product::Egg));
    assert!(animal(rolled(Product::Milk).unwrap()).gives(Product::Milk));
    let goat = dev_animal(Product::Milk, Some("goat"), seed);
    assert_eq!(goat.map(|def| def.key.as_str()), Some("goat"));
    assert!(dev_animal(Product::Milk, Some("chicken"), seed).is_none());
}

#[test]
fn milk_and_honey_are_held_as_other_liquids() {
    let mut rng = Seed::new(3).rng();
    assert!(LiquidKind::all().contains(&LiquidKind::Milk));
    assert_eq!(rolled_milk(&mut rng).kind(), LiquidKind::Milk);
    assert_eq!(rolled_honey(&mut rng), Liquid::Honey("honeybee".to_owned()));
    let jug = VesselForm::new(VesselKind::Jug).full_of(Liquid::Milk("cattle".to_owned()));
    assert_eq!(jug.name(), "Jug of Cow's Milk");
    assert!(jug.drinkable());
    let honey = VesselForm::new(VesselKind::Jug).full_of(Liquid::Honey("honeybee".to_owned()));
    assert!(!honey.drinkable());
}

#[test]
fn cooking_finds_them_by_their_form() {
    let table = Ingredients::built_in();
    let egg = ItemForm::Egg(EggForm::of(animal("duck")).unwrap());
    let found = table.of_form(&egg).unwrap();
    assert_eq!((found.source, found.key.as_str()), (Source::Egg, "duck"));
    let milk = BottleForm::new(Some(Liquid::Milk("sheep".to_owned())), 0);
    let found = table.of_form(&ItemForm::Bottle(milk)).unwrap();
    assert_eq!((found.source, found.key.as_str()), (Source::Milk, "sheep"));
    let honey = VesselForm::new(VesselKind::Jug).full_of(Liquid::Honey("honeybee".to_owned()));
    let found = table.of_form(&ItemForm::Vessel(honey)).unwrap();
    assert_eq!(found.source, Source::Honey);
    let empty = ItemForm::Bottle(BottleForm::new(None, 0));
    assert!(table.of_form(&empty).is_none());
}

#[test]
fn a_place_sells_what_is_kept_there() {
    for n in 0..20 {
        let seed = Seed::new(n);
        let def = animal_for(Product::Milk, Biome::Tundra, seed).unwrap();
        assert!(def.kept.contains(&Biome::Tundra), "{}", def.key);
        let egg = animal_for(Product::Egg, Biome::Grassland, seed).unwrap();
        assert!(egg.gives(Product::Egg));
    }
    assert!(animal_for(Product::Wool, Biome::Grassland, Seed::new(1)).is_none());
}
