use super::*;
use crate::climate::Biome;
use crate::herb::climate_tests::extremes;
use crate::id::Id;

fn place(biome: Biome, celsius: i32) -> Habitat {
    Habitat {
        biome,
        metres: 0,
        celsius,
    }
}

/// Every biome, at its coldest and hottest.
fn every_place() -> Vec<Habitat> {
    let mut places = Vec::new();
    for biome in Biome::ALL {
        let (coldest, hottest) = extremes(biome);
        places.push(place(biome, coldest));
        places.push(place(biome, hottest));
    }
    places
}

#[test]
fn every_place_has_fruit_and_vegetables_both_ways() {
    for habitat in every_place() {
        for source in [Source::Grown, Source::Foraged] {
            for n in 0..20 {
                let seed = Seed::new(n);
                let fruit = fruit_for(Fruits::built_in(), &habitat, source, seed);
                assert!(fruit.is_some(), "no fruit at {habitat:?}, {source:?}");
                let vegetable = vegetable_for(Vegetables::built_in(), &habitat, source, seed);
                assert!(vegetable.is_some(), "no vegetable at {habitat:?}");
            }
        }
    }
}

#[test]
fn foraged_fruit_grows_wild_there_when_any_does() {
    let fruits = Fruits::built_in();
    for habitat in every_place() {
        if fruit::wild_fruits(fruits, &habitat).is_empty() {
            continue;
        }
        for n in 0..50 {
            let def = fruit_for(fruits, &habitat, Source::Foraged, Seed::new(n)).unwrap();
            assert!(def.grows_wild(&habitat), "{} at {habitat:?}", def.key);
        }
    }
}

#[test]
fn grown_produce_suits_the_place() {
    let habitat = place(Biome::SeasonalForest, 10);
    for n in 0..100 {
        let seed = Seed::new(n);
        let fruit = fruit_for(Fruits::built_in(), &habitat, Source::Grown, seed).unwrap();
        assert!(fruit.biomes.contains(&habitat.biome) && fruit.suits_warmth(10));
        let crop = vegetable_for(Vegetables::built_in(), &habitat, Source::Grown, seed).unwrap();
        assert!(
            !crop.wild_only && crop.grows_wild(habitat.biome),
            "{}",
            crop.key
        );
    }
}

#[test]
fn not_only_apples() {
    let habitat = place(Biome::SeasonalForest, 10);
    let mut seen = std::collections::BTreeSet::new();
    for n in 0..200 {
        let fruit = fruit_for(Fruits::built_in(), &habitat, Source::Grown, Seed::new(n));
        seen.insert(fruit.unwrap().key.clone());
    }
    assert!(seen.len() > 5, "{seen:?}");
}

#[test]
fn deserts_and_rainforests_differ() {
    let pick = |habitat: Habitat| -> std::collections::BTreeSet<String> {
        (0..200)
            .map(|n| {
                let seed = Seed::new(n);
                let def = fruit_for(Fruits::built_in(), &habitat, Source::Grown, seed);
                def.unwrap().key.clone()
            })
            .collect()
    };
    let desert = pick(place(Biome::Desert, 25));
    let tundra = pick(place(Biome::Tundra, -5));
    assert!(desert.is_disjoint(&tundra), "{desert:?} and {tundra:?}");
}

#[test]
fn an_item_is_suited_by_its_own_id() {
    let habitat = place(Biome::TropicalRainforest, 26);
    let make = |kind, n| {
        let mut item = Item::new(Id::for_test(n), kind);
        suit_to_place(&mut item, &habitat, Source::Grown);
        item.form
    };
    for n in 1..30 {
        let fruit = make(ItemKind::Fruit, n);
        assert!(matches!(fruit, Some(ItemForm::Fruit(_))));
        assert_eq!(fruit, make(ItemKind::Fruit, n));
        assert!(matches!(
            make(ItemKind::Vegetable, n),
            Some(ItemForm::Vegetable(_))
        ));
        assert_eq!(make(ItemKind::Stone, n), None);
    }
}
