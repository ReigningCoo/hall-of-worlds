//! Fungi used like herbs in the Mortar (fungi plan PR 8): offered beside
//! herbs, ground by their four effects, known and taught in `FungusKnowledge`,
//! and never the same kind as a herb that shares a key.

use super::*;
use crate::brewing::{GroundHerb, Ingredient};
use crate::fungus::{Fungi, FungusDef, FungusKnowledge, cut};
use crate::herb::{HerbKnowledge, Herbs};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        let given = inventory.give(item, StackLimits::default(), stand_ins);
        given.unwrap();
    }
    inventory
}

/// The first built-in fungus with herb effects, and the first without.
fn fungi() -> (&'static FungusDef, &'static FungusDef) {
    let table = Fungi::built_in();
    let like_a_herb = table.iter().find(|def| !def.herb.is_empty()).unwrap();
    let food_only = table.iter().find(|def| def.herb.is_empty()).unwrap();
    (like_a_herb, food_only)
}

fn fungus(def: &FungusDef, count: u32, id: u128) -> Item {
    crate::fungus::new_item(def, count, Id::for_test(id), None)
}

#[test]
fn fungi_with_herb_effects_are_offered_beside_herbs() {
    let herbs = Herbs::built_in();
    let herb = herbs.iter().next().unwrap();
    let (like_a_herb, food_only) = fungi();
    let carried = carrying(vec![
        crate::herb::new_item(herb, 2, Id::for_test(1), None),
        fungus(like_a_herb, 3, 2),
        fungus(food_only, 1, 3),
    ]);
    let stacks = carried.herb_stacks();
    let kinds: Vec<_> = stacks
        .iter()
        .map(|s| (s.ingredient, s.herb.as_str()))
        .collect();
    assert_eq!(
        kinds,
        vec![
            (Ingredient::Herb, herb.key.as_str()),
            (Ingredient::Fungus, like_a_herb.key.as_str())
        ]
    );
    assert_eq!(stacks[1].quality, Quality::Common);
    assert_eq!(stacks[1].count, 3);
}

#[test]
fn a_fungus_serving_is_not_offered() {
    let table = Fungi::built_in();
    let piece = table
        .iter()
        .find(|def| !def.herb.is_empty() && def.servings > 1);
    let Some(piece) = piece else {
        return;
    };
    let mut ids = (10..).map(Id::for_test);
    let (serving, _) = cut(&fungus(piece, 1, 1), &mut || ids.next().unwrap()).unwrap();
    assert!(carrying(vec![serving]).herb_stacks().is_empty());
}

#[test]
fn a_fungus_is_ground_and_teaches_its_own_effects() {
    let herbs = Herbs::built_in();
    let (like_a_herb, _) = fungi();
    let carried = carrying(vec![
        fungus(like_a_herb, 2, 1),
        Item::new(Id::for_test(2), ItemKind::MortarAndPestle),
    ]);
    let order = AlchemyOrder {
        source: None,
        steps: vec![OrderStep::Mortar {
            herbs: vec![carried.herb_stacks()[0].stack],
            solvent: None,
        }],
        count: 1,
    };
    let skills = AlchemySkills::default();
    let checked = carried.check_alchemy(&order, &herbs, skills).unwrap();
    assert_eq!(checked.herbs, vec![like_a_herb.key.clone()]);
    let planned = &checked.planned;
    assert_eq!(planned.contributions.len(), like_a_herb.herb.len());
    assert!(
        planned
            .contributions
            .iter()
            .all(|c| c.ingredient == Ingredient::Fungus)
    );
    let mut herb_known = HerbKnowledge::default();
    let mut fungi_known = FungusKnowledge::default();
    let known = Known {
        herbs: &herb_known,
        fungi: &fungi_known,
    };
    assert_ne!(planned.product, Product::Nothing);
    assert!(preview(planned, known).contains(&Seen::Unknown));
    let lesson = lesson(planned, known).unwrap();
    assert_eq!(lesson.ingredient, Ingredient::Fungus);
    assert!(lesson.learn(&mut herb_known, &mut fungi_known));
    assert!(fungi_known.knows_effect(&like_a_herb.key, lesson.effect));
    assert!(herb_known.is_empty());
}

#[test]
fn a_herb_and_a_fungus_sharing_a_key_are_two_kinds() {
    let herbs = Herbs::built_in();
    let (like_a_herb, _) = fungi();
    let mut herb = herbs.iter().next().unwrap().clone();
    herb.key = like_a_herb.key.clone();
    let mix = vec![
        GroundHerb::herb(&herb, Quality::Common),
        GroundHerb::fungus(like_a_herb),
    ];
    let process = Process {
        source: None,
        steps: vec![Step::Mortar {
            herbs: mix,
            solvent: Solvent::Dry,
        }],
        at_bench: false,
    };
    let planned = plan(&process, AlchemySkills::default()).unwrap();
    assert_eq!(
        planned.contributions.len(),
        herb.effects.len() + like_a_herb.herb.len()
    );
}
