//! Whether what a recipe asks for is carried (`Inventory::has_recipe_input`).

use super::*;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::livestock::Livestock;
use crate::potion::{BottleForm, Liquid, Water};
use crate::pour::BOTTLE_ML;

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        (inventory.give(item, StackLimits::default(), stand_ins)).unwrap();
    }
    inventory
}

fn eggs(count: u32) -> Item {
    let hen = Livestock::built_in().get("chicken").unwrap();
    crate::farmyard::new_egg(hen, count, Id::for_test(1), None).unwrap()
}

fn water_bottles(count: u32) -> Item {
    let mut bottle = Item::new(Id::for_test(3), ItemKind::Bottle);
    let water = Some(Liquid::Water(Water::Plain));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(water, 3)));
    bottle.count = count;
    bottle
}

#[test]
fn enough_of_any_food_asked_for_is_carried() {
    let ingredients = Ingredients::built_in();
    let carried = carrying(vec![eggs(3)]);
    let any_egg = |count| RecipeInput::Food {
        foods: vec![Wanted::Any(Source::Egg)],
        count,
    };
    assert!(carried.has_recipe_input(&any_egg(3), ingredients));
    assert!(!carried.has_recipe_input(&any_egg(4), ingredients));
    let carrot = RecipeInput::Food {
        foods: vec![Wanted::One(Source::Vegetable, "carrot".to_owned())],
        count: 1,
    };
    assert!(!carried.has_recipe_input(&carrot, ingredients));
}

#[test]
fn water_is_counted_by_the_millilitre() {
    let ingredients = Ingredients::built_in();
    let carried = carrying(vec![water_bottles(2)]);
    let water = |ml| RecipeInput::Water { ml };
    assert!(carried.has_recipe_input(&water(2 * BOTTLE_ML), ingredients));
    assert!(!carried.has_recipe_input(&water(2 * BOTTLE_ML + 1), ingredients));
    let milk = RecipeInput::Liquid {
        foods: vec![Wanted::Any(Source::Milk)],
        ml: 100,
    };
    assert!(!carried.has_recipe_input(&milk, ingredients));
}

#[test]
fn the_stacks_an_input_takes_are_found() {
    let ingredients = Ingredients::built_in();
    let carried = carrying(vec![eggs(2), water_bottles(1)]);
    let any_egg = RecipeInput::Food {
        foods: vec![Wanted::Any(Source::Egg)],
        count: 5,
    };
    let stacks = carried.recipe_stacks(&any_egg, ingredients);
    assert_eq!(stacks.len(), 1);
    assert_eq!(stacks[0].count(), 2);
    assert!(any_egg.takes(stacks[0].top(), ingredients));
    let water = RecipeInput::Water { ml: 100 };
    assert!(!water.takes(stacks[0].top(), ingredients));
    assert_eq!(carried.recipe_stacks(&water, ingredients).len(), 1);
    assert_eq!(any_egg.amount(), 5);
    assert!(water.is_liquid() && !any_egg.is_liquid());
}
