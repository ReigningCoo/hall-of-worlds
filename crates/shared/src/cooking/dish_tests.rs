use super::*;
use crate::bonus::{self, Amount};
use crate::fungus::Fungi;
use crate::item::Quality;
use crate::vitals::Vital;

fn food(source: Source, key: &str, grams: u32) -> Portion {
    Portion::Food {
        source,
        key: key.into(),
        grams,
        quality: None,
    }
}

fn water(ml: u32) -> Portion {
    Portion::Water { ml }
}

fn egg() -> Portion {
    food(Source::Egg, "chicken", 60)
}

/// One row of steps, each tool with what goes in it, worked out at a hearth.
fn made(steps: Vec<(Tool, Vec<Portion>)>) -> Output {
    let steps = (steps.into_iter().enumerate())
        .map(|(column, (tool, portions))| Step::at(tool, column as u8, 0, portions))
        .collect();
    let process = Process {
        rows: 1,
        sources: Vec::new(),
        steps,
        heat: Heat::Hearth,
    };
    let planned = plan(
        &process,
        Ingredients::built_in(),
        Dishes::built_in(),
        skills(),
    );
    let mut outputs = planned.expect("made").outputs;
    assert_eq!(outputs.len(), 1);
    outputs.remove(0)
}

fn skills() -> CookingSkills {
    CookingSkills {
        cooking: 5000,
        sub: [5000; Tool::ALL.len()],
    }
}

fn name(output: &Output) -> String {
    output.dish.name(Dishes::built_in())
}

#[test]
fn the_dishes_are_all_there_and_name_real_foods() {
    let dishes = Dishes::built_in();
    assert!(dishes.len() > 200);
    let ingredients = Ingredients::built_in();
    for def in dishes.iter() {
        for wanted in def.specific.iter().chain(&def.with).flatten() {
            if let Wanted::One(source, key) = wanted {
                let found = ingredients.get(*source, key);
                assert!(found.is_some(), "{}: {source:?} {key}", def.key);
            }
        }
    }
}

#[test]
fn eggs_by_their_steps() {
    assert_eq!(name(&made(vec![(Tool::Pan, vec![egg()])])), "Fried Egg");
    let beaten = made(vec![
        (Tool::Bowl, vec![egg(), egg()]),
        (Tool::Pan, Vec::new()),
    ]);
    assert_eq!(name(&beaten), "Omelette");
    assert_eq!(beaten.dish.form, DishForm::EggDish);
    let with_milk = vec![egg(), egg(), food(Source::Milk, "cattle", 30)];
    let scrambled = made(vec![(Tool::Bowl, with_milk), (Tool::Pan, Vec::new())]);
    assert_eq!(name(&scrambled), "Scrambled Eggs");
}

#[test]
fn a_real_dish_wants_its_roles_and_may_be_all_its_food() {
    // A fried egg is all egg, so it loses nothing for it, but wants a fat.
    let fried = made(vec![(Tool::Pan, vec![egg()])]);
    assert_eq!(fried.balance, 1 - 1);
}

#[test]
fn generic_names_from_form_and_biggest_foods() {
    let stew = made(vec![
        (
            Tool::Knife,
            vec![
                food(Source::Meat, "sheep", 400),
                food(Source::Vegetable, "turnip", 300),
                food(Source::Vegetable, "parsnip", 100),
            ],
        ),
        (Tool::Cauldron, vec![water(300)]),
    ]);
    assert_eq!(name(&stew), "Mutton and Turnip Stew");
    assert_eq!(stew.dish.form, DishForm::Bowl);
    let soup = made(vec![
        (Tool::Knife, vec![food(Source::Vegetable, "turnip", 300)]),
        (Tool::Cauldron, vec![water(1000)]),
    ]);
    assert_eq!(name(&soup), "Turnip Soup");
}

#[test]
fn herbs_within_their_share_never_change_the_dish() {
    let plain = made(vec![(Tool::Pan, vec![egg()])]);
    let herbed = made(vec![(
        Tool::Pan,
        vec![egg(), food(Source::Herb, "black_pepper", 1)],
    )]);
    assert_eq!(plain.dish, herbed.dish);
}

#[test]
fn fungi_are_only_ever_mushrooms_in_a_name() {
    let fried = made(vec![(
        Tool::Pan,
        vec![food(Source::Fungus, "death_cap", 100)],
    )]);
    assert_eq!(name(&fried), "Fried Mushroom");
}

#[test]
fn water_carried_on_from_a_cauldron_is_drained() {
    let boiled = made(vec![
        (
            Tool::Cauldron,
            vec![food(Source::Vegetable, "potato", 300), water(1000)],
        ),
        (Tool::Sieve, Vec::new()),
    ]);
    let shape = Shape::of(
        &boiled.history,
        &boiled.portions,
        0,
        Ingredients::built_in(),
    );
    assert_eq!(shape.wetness, Wetness::Dry);
    assert_eq!(name(&boiled), "Boiled Potato");
}

#[test]
fn diet_labels_from_the_flags() {
    let ingredients = Ingredients::built_in();
    let bread = [food(Source::Vegetable, "wheat", 500), water(300)];
    let diet = Diet::of(&bread, ingredients);
    assert!(diet.vegan && diet.vegetarian && !diet.gluten_free && diet.lactose_free);
    let custard = [egg(), food(Source::Milk, "cattle", 200)];
    let diet = Diet::of(&custard, ingredients);
    assert!(!diet.vegan && diet.vegetarian && diet.gluten_free && !diet.lactose_free);
    let stew = [
        food(Source::Meat, "pig", 300),
        food(Source::Fruit, "walnut", 30),
    ];
    let diet = Diet::of(&stew, ingredients);
    assert!(!diet.vegetarian && !diet.nut_free);
    assert_eq!(diet.labels(), vec!["Gluten free", "Lactose free"]);
}

#[test]
fn poisonous_fungi_carry_their_poison() {
    let fungi = Fungi::built_in();
    let dish = [
        food(Source::Fungus, "death_cap", 50),
        food(Source::Fungus, "porcini", 50),
    ];
    let found: Vec<&str> = (poisons(&dish, true, fungi).iter())
        .map(|d| d.key.as_str())
        .collect();
    assert_eq!(found, vec!["death_cap"]);
    // One only harmful raw is safe cooked.
    let raw_only = (fungi.iter())
        .find(|def| {
            def.poison
                .as_ref()
                .is_some_and(|p| p.raw_only && p.when_eaten())
        })
        .expect("a fungus only harmful raw");
    let dish = [food(Source::Fungus, &raw_only.key, 50)];
    assert_eq!(poisons(&dish, false, fungi).len(), 1);
    assert!(poisons(&dish, true, fungi).is_empty());
}

fn restores(well_fed: &WellFed) -> (u32, u32) {
    (well_fed.health, well_fed.mana)
}

#[test]
fn well_fed_leans_by_what_is_in_it() {
    let ingredients = Ingredients::built_in();
    let beef = WellFed::of(&[food(Source::Meat, "cattle", 300)], true, ingredients);
    assert_eq!(restores(&beef), (WELL_FED_RESTORE, 0));
    let bread = WellFed::of(&[food(Source::Vegetable, "wheat", 300)], true, ingredients);
    assert!(bread.mana > bread.health);
    assert_eq!(bread.health + bread.mana, WELL_FED_RESTORE);
    assert_eq!(
        restores(&WellFed::of(&[water(500)], true, ingredients)),
        (0, 0)
    );
}

#[test]
fn well_fed_lasts_by_quality_with_one_at_a_time() {
    let minute = 60 * crate::energy::ACTION_TICKS_PER_SECOND as u64;
    assert_eq!(well_fed_ticks(Quality::Poor), 20 * minute);
    assert_eq!(well_fed_ticks(Quality::Common), 30 * minute);
    assert_eq!(well_fed_ticks(Quality::Legendary), 70 * minute);
    let beef = WellFed::of(
        &[food(Source::Meat, "cattle", 300)],
        true,
        Ingredients::built_in(),
    );
    let effect = beef.effect(Quality::Common, 100);
    assert_eq!(effect.source, bonus::Source::WellFed);
    assert_eq!(effect.end, Some(100 + 30 * minute));
    assert!(bonus::Source::WellFed.replaced_by(bonus::Source::WellFed));
    assert!(!bonus::Source::WellFed.replaced_by(bonus::Source::Drink));
    let restore = effect
        .bonuses
        .iter()
        .find(|b| b.stat == bonus::Stat::Restore(Vital::Health));
    assert_eq!(
        restore.map(|b| b.amount),
        Some(Amount::Flat(WELL_FED_RESTORE as i32))
    );
}

#[test]
fn well_fed_carries_cooked_effects_and_herbs() {
    let ingredients = Ingredients::built_in();
    let vegetables = crate::vegetable::Vegetables::built_in();
    let def = (vegetables.iter())
        .find(|def| {
            def.effects
                .iter()
                .any(|e| e.when == crate::vegetable::When::Cooked)
        })
        .expect("a vegetable with a cooked effect");
    let portions = [food(Source::Vegetable, &def.key, 200)];
    let cooked = WellFed::of(&portions, true, ingredients);
    let raw = WellFed::of(&portions, false, ingredients);
    assert_ne!(cooked.bonuses, raw.bonuses);
    let herb = crate::herb::Herbs::built_in_ref().get("basil").unwrap();
    let basil = WellFed::of(&[food(Source::Herb, "basil", 5)], true, ingredients);
    let first = herb.effects[0];
    assert_eq!(
        basil.bonuses,
        crate::fruit::bonus_of(first.stat, first.change)
            .into_iter()
            .collect::<Vec<_>>()
    );
}

#[test]
fn real_dishes_when_close() {
    let f = food;
    let stew = made(vec![
        (
            Tool::Knife,
            vec![
                f(Source::Meat, "cattle", 500),
                f(Source::Vegetable, "carrot", 200),
                f(Source::Vegetable, "onion", 100),
            ],
        ),
        (Tool::Cauldron, vec![water(1000)]),
    ]);
    assert_eq!(name(&stew), "Beef Stew");
    let pesto = vec![
        f(Source::Herb, "basil", 50),
        f(Source::Fruit, "olive", 50),
        f(Source::Herb, "garlic", 10),
        f(Source::Fruit, "stone_pine_nut", 20),
    ];
    assert_eq!(name(&made(vec![(Tool::Mortar, pesto)])), "Pesto");
    let rice = vec![f(Source::Vegetable, "rice", 300), water(600)];
    let fried = vec![egg(), f(Source::Vegetable, "onion", 50)];
    let egg_fried_rice = made(vec![(Tool::Cauldron, rice), (Tool::Pan, fried)]);
    assert_eq!(name(&egg_fried_rice), "Egg Fried Rice");
    // Grain in a Cauldron is porridge, however wet.
    let oats = vec![
        f(Source::Vegetable, "oats", 200),
        f(Source::Milk, "cattle", 400),
    ];
    assert_eq!(
        name(&made(vec![(Tool::Cauldron, oats)])),
        "Milk and Oats Porridge"
    );
}
