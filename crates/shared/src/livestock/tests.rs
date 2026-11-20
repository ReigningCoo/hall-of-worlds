use super::*;
use crate::creature::CreatureKind;
use crate::item::Quality;

fn def(key: &str) -> &'static LivestockDef {
    Livestock::built_in().get(key).expect("a built-in animal")
}

#[test]
fn the_table_has_every_animal() {
    let animals = Livestock::built_in();
    assert_eq!(animals.len(), 26);
    let count = |class| animals.iter().filter(|def| def.class == class).count();
    assert_eq!(count(Class::Mammal), 16);
    assert_eq!(count(Class::Bird), 8);
    assert_eq!(count(Class::Insect), 2);
}

#[test]
fn kinds_find_their_animal() {
    let animals = Livestock::built_in();
    for (kind, key) in [
        (CreatureKind::Cattle, "cattle"),
        (CreatureKind::Coo, "highland_cattle"),
        (CreatureKind::Horse, "horse"),
        (CreatureKind::Chicken, "chicken"),
        (CreatureKind::Sheep, "sheep"),
        (CreatureKind::Goat, "goat"),
        (CreatureKind::Pig, "pig"),
        (CreatureKind::WaterBuffalo, "water_buffalo"),
        (CreatureKind::Yak, "yak"),
        (CreatureKind::Donkey, "donkey"),
        (CreatureKind::Mule, "mule"),
        (CreatureKind::Dromedary, "dromedary"),
        (CreatureKind::BactrianCamel, "bactrian_camel"),
        (CreatureKind::Llama, "llama"),
        (CreatureKind::Alpaca, "alpaca"),
        (CreatureKind::Reindeer, "reindeer"),
        (CreatureKind::Rabbit, "rabbit"),
        (CreatureKind::Duck, "duck"),
        (CreatureKind::MuscovyDuck, "muscovy_duck"),
        (CreatureKind::Goose, "goose"),
        (CreatureKind::Turkey, "turkey"),
        (CreatureKind::Guineafowl, "guineafowl"),
        (CreatureKind::Pigeon, "pigeon"),
        (CreatureKind::Quail, "quail"),
    ] {
        assert_eq!(animals.by_kind(kind).map(|def| def.key.as_str()), Some(key));
    }
    assert!(animals.by_kind(CreatureKind::Wolf).is_none());
}

#[test]
fn variants_have_their_own_rows() {
    let animals = Livestock::built_in();
    let variants = |key| {
        animals
            .variants(key)
            .map(|def| def.key.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(variants("cattle"), ["highland_cattle"]);
    assert_eq!(variants("duck"), ["muscovy_duck"]);
    assert_eq!(variants("dromedary"), ["bactrian_camel"]);
    assert_eq!(variants("llama"), ["alpaca"]);
}

#[test]
fn the_parts_name_animals_in_the_table() {
    let parts = Parts::built_in();
    assert_eq!(parts.organs.len(), 26);
    assert_eq!(parts.fats.len(), 9);
    assert_eq!(parts.others.len(), 10);
    let organs = |key| {
        parts
            .organs_of(def(key))
            .map(|o| o.key.as_str())
            .collect::<Vec<_>>()
    };
    assert!(organs("cattle").contains(&"tripe"));
    assert!(
        !organs("pig").contains(&"tripe"),
        "a pig doesn't chew the cud"
    );
    assert!(organs("pigeon").contains(&"gizzard"));
    assert!(!organs("pigeon").contains(&"bile"), "no gall bladder");
    assert!(!organs("horse").contains(&"bile"));
    assert!(
        organs("honeybee").is_empty(),
        "insects give only their brood"
    );
    let fats = |key| parts.fats_of(def(key)).count();
    assert_eq!(fats("pig"), 2);
    assert_eq!(fats("horse"), 0);
}

#[test]
fn reading_refuses_what_is_wrong() {
    let unknown = "(organs: [(key: \"x\", name: \"x\", from: [Animal(\"dodo\")], base: 5)])";
    let error = Parts::read(unknown, Livestock::built_in()).expect_err("no dodo");
    assert!(error.contains("dodo"), "{error}");
    let twice = format!("[{0}, {0}]", one_row("a"));
    assert!(
        Livestock::read(&twice)
            .expect_err("twice")
            .contains("twice")
    );
    let variant = format!(
        "[{}]",
        one_row("a").replace("pet:", "variant_of: Some(\"b\"), pet:")
    );
    assert!(Livestock::read(&variant).is_err(), "no b");
}

/// A small animal written in RON.
fn one_row(key: &str) -> String {
    format!(
        r#"(key: "{key}", name: "A", latin: "A a", class: Mammal, kept: [Grassland],
        feral: (rarity: Never), diet: Grazer, favourites: ["grass"], water: Drinks,
        stages: (young: "y", juvenile: (female: "j", male: "j"),
            adult: (female: "f", male: "m"), young_until: 1, juvenile_until: 2),
        meat: (name: "a", young: "b", cuts: (poor: ["c"])), pet: (often: Rare))"#
    )
}

#[test]
fn stages_follow_age() {
    let stages = &def("cattle").stages;
    assert_eq!(stages.young_until, 9 * GAME_HOURS_A_MONTH);
    assert_eq!(stages.at(0), Stage::Young);
    assert_eq!(stages.at(216), Stage::Juvenile);
    assert_eq!(stages.at(720), Stage::Adult);
    assert_eq!(stages.adult.castrate.as_deref(), Some("ox"));
}

#[test]
fn cut_names_are_chosen_by_hash() {
    let meat = &def("cattle").meat;
    assert_eq!(meat.cut_name(Quality::Epic, 0), Some("sirloin"));
    assert_eq!(meat.cut_name(Quality::Epic, 4), Some("ribeye"));
    assert_eq!(def("pigeon").meat.cut_name(Quality::Poor, 7), None);
    assert_eq!(meat.cut_name(Quality::Artifact, 0), None);
}

#[test]
fn shares_sum_to_a_hundred() {
    for animal in Livestock::built_in().iter() {
        for young in [false, true] {
            let shares = animal.meat.shares(young);
            let total: u32 = shares.iter().map(|&(_, share)| share).sum();
            assert_eq!(total, 100, "{} {young}", animal.key);
        }
    }
    assert_eq!(def("cattle").meat.shares(false)[5], (Quality::Legendary, 3));
}

#[test]
fn young_meat_starts_a_step_higher() {
    assert_eq!(def("cattle").meat.young_lowest(), Some(Quality::Common));
    assert_eq!(def("yak").meat.young_lowest(), Some(Quality::Uncommon));
    assert_eq!(def("honeybee").meat.young_lowest(), Some(Quality::Poor));
    let young = def("cattle").meat.shares(true);
    assert_eq!(
        young.first().map(|&(quality, _)| quality),
        Some(Quality::Common)
    );
}

#[test]
fn furry_hides_are_pelts() {
    for key in ["sheep", "yak", "llama", "alpaca", "reindeer", "rabbit"] {
        assert_eq!(
            def(key).hide.as_ref().map(|hide| hide.cure),
            Some(Cure::Pelt),
            "{key}"
        );
    }
    assert_eq!(
        def("cattle").hide.as_ref().map(|hide| hide.cure),
        Some(Cure::Leather)
    );
    assert!(def("chicken").hide.is_none());
}

#[test]
fn organ_chances_follow_butchery() {
    assert_eq!(organ_chance(70, false, 0), 7000);
    assert_eq!(organ_chance(70, false, 100), 9500);
    assert_eq!(organ_chance(30, false, 50), 3000 + 65 * 50);
    assert_eq!(organ_chance(100, false, 100), OUT_OF);
    assert_eq!(organ_chance(1, true, 0), 50);
    assert_eq!(organ_chance(1, true, 100), 150);
}

#[test]
fn eggs_have_weights_and_milk_is_at_game_rates() {
    let milk = &def("cattle").produce[0];
    assert_eq!(
        (milk.product, milk.amount, milk.per),
        (Product::Milk, 10_000, Per::Day)
    );
    let egg = &def("chicken").produce[0];
    assert_eq!((egg.product, egg.grams), (Product::Egg, 55));
    assert!(def("pig").produce.is_empty());
}
