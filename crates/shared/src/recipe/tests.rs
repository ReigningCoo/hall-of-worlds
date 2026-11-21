use super::*;
use crate::cooking::{DishName, Recipe, RecipeInput, RecipeStep, Tool, dish_recipe};
use crate::id::Id;
use crate::item::{ItemKind, StackLimits};
use crate::rng::Seed;

fn scroll(dish: &str, count: u32) -> crate::item::Item {
    scroll_id(dish, count, 1)
}

fn scroll_id(dish: &str, count: u32, id: u128) -> crate::item::Item {
    new_item(RecipeForm::cooking(dish), count, Id::for_test(id), None)
}

fn own(ml: u32) -> (Recipe, DishName) {
    let recipe = Recipe {
        rows: 1,
        sources: Vec::new(),
        steps: vec![RecipeStep {
            tool: Tool::Cauldron,
            column: 0,
            row: 0,
            height: 1,
            inputs: vec![RecipeInput::Water { ml }],
        }],
    };
    (recipe, DishName::Real("boiled_egg".into()))
}

#[test]
fn scrolls_are_one_cell_counted_without_quality_and_stack_by_dish() {
    let naan = scroll("naan", 2);
    assert_eq!(naan.kind, ItemKind::Recipe);
    assert_eq!(naan.quality, None);
    assert!(naan.is_counted());
    assert_eq!(naan.shape().cells().count(), 1);
    assert_eq!(naan.own_weight(), RECIPE_GRAMS);
    assert_eq!(
        naan.recipe().map(|r| r.name()),
        Some("Recipe for Naan".into())
    );
    let limits = StackLimits::default();
    assert!(limits.room(&naan, &scroll_id("naan", 1, 2)) > 0);
    assert_eq!(limits.room(&naan, &scroll_id("pita", 1, 3)), 0);
}

#[test]
fn give_item_rolls_a_dish_with_a_recipe() {
    for n in 0..20 {
        let form = dev_recipe(None, Seed::new(n)).expect("a dish");
        assert!(dish_recipe(&form.dish).is_some());
    }
    assert_eq!(
        dev_recipe(Some("naan"), Seed::new(1)),
        Some(RecipeForm::cooking("naan"))
    );
}

#[test]
fn reading_a_scroll_learns_it_once() {
    let mut book = RecipeBook::default();
    assert_eq!(book.read(&scroll("naan", 1)), Ok(()));
    assert!(book.knows(&RecipeForm::cooking("naan")));
    assert_eq!(book.read(&scroll("naan", 1)), Err(Refused::Known));
    assert_eq!(book.read(&scroll("no_such_dish", 1)), Err(Refused::Unknown));
    let egg = crate::item::Item::new(Id::for_test(4), ItemKind::Egg);
    assert_eq!(book.read(&egg), Err(Refused::NotAScroll));
    assert_eq!(book.len(), 1);
}

#[test]
fn removing_a_learnt_recipe_gives_its_scroll_back() {
    let mut book = RecipeBook::default();
    book.read(&scroll("naan", 1)).expect("read");
    book.learn(&RecipeForm::cooking("pita"), false);
    let naan = book
        .iter()
        .find(|e| matches!(e, Entry::Learnt { dish, .. } if dish == "naan"));
    let naan = naan.expect("naan").clone();
    let removed = book.remove(&naan).expect("in it");
    assert_eq!(removed.scroll, Some(RecipeForm::cooking("naan")));
    let pita = book.iter().next().expect("pita").clone();
    assert_eq!(book.remove(&pita).expect("in it").scroll, None);
    assert!(book.is_empty());
    assert_eq!(book.remove(&pita), None);
}

#[test]
fn own_recipes_move_to_the_front_and_the_oldest_go() {
    let mut book = RecipeBook::default();
    book.learn(&RecipeForm::cooking("naan"), true);
    for ml in 0..MOST_OWN as u32 + 5 {
        let (recipe, name) = own(ml);
        book.remember_cooking(recipe, name);
    }
    let owns = book.iter().filter(|e| matches!(e, Entry::Own(_))).count();
    assert_eq!(owns, MOST_OWN);
    assert!(book.knows(&RecipeForm::cooking("naan")), "learnt ones stay");
    // The same process again moves to the front, not in twice.
    let (recipe, name) = own(50);
    book.remember_cooking(recipe.clone(), name.clone());
    assert_eq!(
        book.iter().next(),
        Some(&Entry::Own(Own::Cooking { recipe, name }))
    );
    assert_eq!(book.len(), MOST_OWN + 1);
}

#[test]
fn older_sheets_load_with_an_empty_book() {
    let sheet = crate::sheet::Sheet::default();
    assert!(sheet.book.is_empty());
    let text = ron::to_string(&sheet).expect("saved");
    let without = text.replace(",book:[]", "");
    assert_ne!(text, without, "the book is saved");
    let loaded: crate::sheet::Sheet = ron::from_str(&without).expect("loaded");
    assert!(loaded.book.is_empty());
}
