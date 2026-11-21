use super::*;
use crate::item::Quality;

#[test]
fn every_real_dish_has_a_recipe_that_makes_it() {
    let (ingredients, dishes) = (Ingredients::built_in(), Dishes::built_in());
    let mut failed = Vec::new();
    for def in dishes.iter() {
        let made = dish_recipe(&def.key).and_then(|r| makes(r, ingredients, dishes));
        if made != Some(DishName::Real(def.key.clone())) {
            failed.push(def.key.clone());
        }
    }
    assert!(failed.is_empty(), "{} don't: {failed:?}", failed.len());
}

#[test]
fn a_recipe_cooks_with_common_foods_by_the_item() {
    let recipe = dish_recipe("naan").expect("naan");
    let process = recipe.process(Ingredients::built_in(), Some(Quality::Common), Heat::Hearth);
    let process = process.expect("a process");
    assert_eq!(process.steps.len(), recipe.steps.len());
    let Some(Portion::Food { key, grams, .. }) = process.steps[0].portions.first() else {
        panic!("a food first");
    };
    assert_eq!(key, "wheat");
    // A sack of wheat flour at a time.
    assert_eq!(
        grams % unit_grams(Source::Vegetable, "wheat").expect("flour"),
        0
    );
}

#[test]
fn foods_are_counted_by_the_item_and_liquids_by_the_ml() {
    assert!(is_liquid_food(Source::Milk, "cattle"));
    assert!(is_liquid_food(Source::Pantry, "soy_sauce"));
    assert!(!is_liquid_food(Source::Pantry, "salt"));
    assert_eq!(unit_grams(Source::Milk, "cattle"), None);
    assert!(unit_grams(Source::Egg, "chicken").is_some_and(|g| g > 20));
    let ingredients = Ingredients::built_in();
    let any_egg = usual_food(&Wanted::Any(Source::Egg), ingredients);
    assert_eq!(any_egg, Some((Source::Egg, "chicken".to_owned())));
}

#[test]
fn a_long_recipe_is_beyond_a_new_cook() {
    let joined = (Dishes::built_in().iter())
        .filter(|def| def.joins > 0)
        .filter_map(|def| dish_recipe(&def.key))
        .max_by_key(|recipe| recipe.halves())
        .expect("a joined dish");
    let new = CookingSkills::default();
    assert!(joined.steps_beyond(new) > 0);
    assert!(joined.beyond_skill(new));
    let master = CookingSkills {
        cooking: crate::alchemy::CERTAIN,
        sub: [0; Tool::ALL.len()],
    };
    assert_eq!(joined.steps_beyond(master), 0);
    assert!(!joined.beyond_skill(master));
}

#[test]
fn the_same_process_written_two_ways_is_one_recipe() {
    let recipe = dish_recipe("jiaozi").expect("jiaozi").clone();
    let mut turned = recipe.clone();
    turned.steps.reverse();
    turned.sources = vec![None, None];
    assert_eq!(turned.normalised(), recipe.normalised());
}

#[test]
fn an_order_is_written_down_as_its_recipe() {
    use crate::id::Id;
    use crate::item::{Inventory, StackLimits};
    let hen = crate::livestock::Livestock::built_in()
        .get("chicken")
        .expect("hen");
    let eggs = crate::farmyard::new_egg(hen, 3, Id::for_test(1), None).expect("eggs");
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    (inventory.give(eggs, StackLimits::default(), stand_ins)).expect("carried");
    let order = CookOrder {
        bench: None,
        rows: 1,
        sources: Vec::new(),
        steps: vec![OrderStep {
            tool: Tool::Pan,
            column: 0,
            row: 0,
            height: 1,
            inputs: vec![
                OrderInput::Food {
                    stack: Id::for_test(1),
                    count: 2,
                },
                OrderInput::WaterFaced { ml: 100 },
            ],
        }],
        culture: None,
        count: 1,
    };
    let recipe = inventory.cooking_recipe(&order, &[], Ingredients::built_in());
    let recipe = recipe.expect("written down");
    let eggs = RecipeInput::Food {
        foods: vec![Wanted::One(Source::Egg, "chicken".into())],
        count: 2,
    };
    assert_eq!(
        recipe.steps[0].inputs,
        vec![eggs, RecipeInput::Water { ml: 100 }]
    );
    let missing = CookOrder {
        sources: vec![Some(Id::for_test(9))],
        ..order
    };
    assert_eq!(
        inventory.cooking_recipe(&missing, &[], Ingredients::built_in()),
        None
    );
}
