//! Give Item's food: a few simple processes worked out by the rules
//! (`cooking::plan`), one chosen from the seed, until the server can cook
//! (plan PR 9).

use super::FoodForm;
use crate::cooking::{
    CookingSkills, Dishes, Heat, Ingredients, Portion, Process, Source, Step, Tool, plan,
};
use crate::item::{ItemKind, Quality};
use crate::rng::Seed;

fn food(source: Source, key: &str, grams: u32) -> Portion {
    Portion::Food {
        source,
        key: key.into(),
        grams,
        quality: None,
    }
}

/// One row of steps, each tool with what is put in at it.
fn row(steps: Vec<(Tool, Vec<Portion>)>) -> Process {
    Process {
        rows: 1,
        sources: Vec::new(),
        steps: (steps.into_iter().enumerate())
            .map(|(at, (tool, portions))| Step::at(tool, at as u8, 0, portions))
            .collect(),
        heat: Heat::Hearth,
    }
}

/// Dishes: a fried egg, scrambled eggs, a beef stew.
fn dishes() -> Vec<Process> {
    let egg = || food(Source::Egg, "chicken", 55);
    let water = Portion::Water { ml: 600 };
    vec![
        row(vec![(Tool::Pan, vec![egg()])]),
        row(vec![(Tool::Bowl, vec![egg(), egg()]), (Tool::Pan, vec![])]),
        row(vec![
            (
                Tool::Knife,
                vec![
                    food(Source::Meat, "cattle", 400),
                    food(Source::Vegetable, "carrot", 150),
                    food(Source::Vegetable, "onion", 100),
                ],
            ),
            (Tool::Cauldron, vec![water]),
        ]),
    ]
}

/// Intermediates: wheat dough, fresh noodles, warm milk, butter, a roux.
fn intermediates() -> Vec<Process> {
    let flour = || food(Source::Vegetable, "wheat", 300);
    let water = || Portion::Water { ml: 150 };
    let milk = || food(Source::Milk, "cattle", 500);
    vec![
        row(vec![(Tool::Bowl, vec![flour(), water()])]),
        row(vec![
            (Tool::Bowl, vec![flour(), water()]),
            (Tool::Knife, vec![]),
        ]),
        row(vec![(Tool::Cauldron, vec![milk()])]),
        row(vec![(Tool::Bowl, vec![milk()])]),
        row(vec![(
            Tool::Pan,
            vec![
                food(Source::Vegetable, "wheat", 50),
                food(Source::Fruit, "olive", 50),
            ],
        )]),
    ]
}

/// A Dish or an intermediate (`kind`) at `quality`, chosen from `seed`;
/// `None` for any other kind.
pub fn dev_food(kind: ItemKind, quality: Quality, seed: Seed) -> Option<FoodForm> {
    let processes = match kind {
        ItemKind::Dish => dishes(),
        ItemKind::Intermediate => intermediates(),
        _ => return None,
    };
    let at = (seed.derive("food").value() % processes.len() as u64) as usize;
    let ingredients = Ingredients::built_in();
    let planned = plan(
        &processes[at],
        ingredients,
        Dishes::built_in(),
        CookingSkills::default(),
    )
    .ok()?;
    let output = planned.outputs.first()?;
    let item = super::food_item(
        output,
        quality,
        None,
        ingredients,
        crate::id::Id::NOBODY,
        None,
    );
    item.food().cloned()
}
