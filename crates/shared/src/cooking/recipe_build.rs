//! Every real dish's recipe, built from its pattern (`DishDef`): its foods
//! (what it must have, what it usually has, a common food for each role it
//! needs), how many of each (`recipe_tune`), its steps laid out in the grid
//! and water for its wetness (`recipe_layout`). Each draft is cooked with
//! Common foods and kept only if it comes out as the dish; a few drafts are
//! tried, from the fullest.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::recipe_layout::{Placed, lay_out};
use super::recipe_tune::{Chosen, common_food, tune};
use super::{
    CookingSkills, DishDef, DishName, Dishes, Heat, IngredientDef, Ingredients, Recipe, Role,
    Wanted, is_liquid_food, is_seasoning, plan, unit_grams, usual_food,
};
use crate::item::Quality;

/// The built-in dishes' recipes by dish key, built once.
static DISH_RECIPES: LazyLock<BTreeMap<String, Recipe>> = LazyLock::new(|| {
    let (ingredients, dishes) = (Ingredients::built_in(), Dishes::built_in());
    (dishes.iter())
        .filter_map(|def| Some((def.key.clone(), build_recipe(def, ingredients, dishes)?)))
        .collect()
});

/// The recipe of a built-in real dish, by its key.
pub fn dish_recipe(key: &str) -> Option<&'static Recipe> {
    DISH_RECIPES.get(key)
}

/// One way of drafting a recipe: with the dish's usual foods or not, and a
/// heavier hand with its needs over what it only names.
#[derive(Clone, Copy)]
pub(super) struct Draft {
    usual: bool,
    lean: bool,
}

/// The drafts tried, in order.
pub(super) const DRAFTS: [Draft; 4] = [
    Draft {
        usual: true,
        lean: false,
    },
    Draft {
        usual: true,
        lean: true,
    },
    Draft {
        usual: false,
        lean: false,
    },
    Draft {
        usual: false,
        lean: true,
    },
];

/// `def`'s recipe: the first draft that, cooked with Common foods at a
/// hearth, comes out as it; `None` if none does.
pub fn build_recipe(def: &DishDef, ingredients: &Ingredients, dishes: &Dishes) -> Option<Recipe> {
    DRAFTS.iter().find_map(|&draft| {
        let recipe = drafted(def, ingredients, draft)?;
        (makes(&recipe, ingredients, dishes) == Some(DishName::Real(def.key.clone())))
            .then_some(recipe)
    })
}

/// What a recipe makes, cooked with Common foods at a hearth by a cook of no
/// skill: its one output's name, or `None` if it can't be made or gives more
/// than one.
pub fn makes(recipe: &Recipe, ingredients: &Ingredients, dishes: &Dishes) -> Option<DishName> {
    let process = recipe.process(ingredients, Some(Quality::Common), Heat::Hearth)?;
    let planned = plan(&process, ingredients, dishes, CookingSkills::default()).ok()?;
    match planned.outputs.as_slice() {
        [output] => Some(output.dish.name.clone()),
        _ => None,
    }
}

/// One draft of `def`'s recipe.
pub(super) fn drafted(def: &DishDef, ingredients: &Ingredients, draft: Draft) -> Option<Recipe> {
    let mut chosen: Vec<Chosen> = Vec::new();
    for group in &def.specific {
        choose(&mut chosen, group, def, ingredients, true);
    }
    if draft.usual {
        for group in &def.with {
            let fits = first_def(group, ingredients)
                .is_some_and(|food| !food.roles.iter().any(|role| def.forbids.contains(role)));
            if fits {
                choose(&mut chosen, group, def, ingredients, false);
            }
        }
    }
    for &role in &def.needs {
        let covered = (chosen.iter()).any(|c| {
            c.def.main == Some(role) || (role == Role::Fat && c.def.roles.contains(&role))
        });
        if !covered {
            let food = common_food(role, def, &chosen, ingredients)?;
            chosen.push(Chosen::new(food, true));
        }
    }
    // A dish naming no foods is a way of mixing them: two at least.
    let bulk = |c: &&Chosen| !c.def.main.is_some_and(is_seasoning);
    if def.specific.is_empty() && chosen.iter().filter(bulk).count() < 2 {
        let role = (def.needs.iter().copied()).find(|r| !is_seasoning(*r))?;
        let food = common_food(role, def, &chosen, ingredients)?;
        chosen.push(Chosen::new(food, true));
    }
    for food in &mut chosen {
        let grams = base_grams(food, def, draft.lean);
        food.set_grams(grams);
    }
    let bulk: u32 = (chosen.iter())
        .filter(|c| !matches!(c.def.main, Some(Role::Herb | Role::Spice)))
        .map(Chosen::grams)
        .sum();
    for food in &mut chosen {
        if matches!(food.def.main, Some(Role::Herb | Role::Spice)) {
            let share = u32::from(food.def.max_share).clamp(1, 2);
            food.set_grams((bulk * share / 100).max(1));
        }
    }
    tune(def, &mut chosen, ingredients);
    let placed: Vec<Placed> = chosen.iter().map(Chosen::placed).collect();
    Some(lay_out(def, placed))
}

/// Adds a group's first food that exists (the usual one for "any", or one
/// of the dish's usual foods of that source), unless it is already chosen.
fn choose<'a>(
    chosen: &mut Vec<Chosen<'a>>,
    group: &[Wanted],
    def: &DishDef,
    ingredients: &'a Ingredients,
    specific: bool,
) {
    let Some(at) = (group.iter()).position(|w| known(w, ingredients).is_some()) else {
        return;
    };
    let mut first = group[at].clone();
    if let Wanted::Any(source) = first {
        let usual = (def.with.iter().flatten()).find(|w| {
            matches!(w, Wanted::One(s, _) if *s == source) && known(w, ingredients).is_some()
        });
        if let Some(usual) = usual {
            first = usual.clone();
        }
    }
    let food = known(&first, ingredients).expect("found");
    if chosen.iter().any(|c| std::ptr::eq(c.def, food)) {
        return;
    }
    let mut foods = vec![first];
    let rest: Vec<Wanted> = group
        .iter()
        .filter(|w| !foods.contains(w))
        .cloned()
        .collect();
    foods.extend(rest);
    chosen.push(Chosen::with_foods(foods, food, specific));
}

/// The culinary data of the food `wanted` means, if a recipe can use it.
pub(super) fn known<'a>(
    wanted: &Wanted,
    ingredients: &'a Ingredients,
) -> Option<&'a IngredientDef> {
    let (source, key) = usual_food(wanted, ingredients)?;
    let usable = is_liquid_food(source, &key) || unit_grams(source, &key).is_some();
    usable.then(|| ingredients.get(source, &key)).flatten()
}

fn first_def<'a>(group: &[Wanted], ingredients: &'a Ingredients) -> Option<&'a IngredientDef> {
    group.iter().find_map(|w| known(w, ingredients))
}

/// A food's grams to start from: by its main role, less for a food the dish only usually has (its main
/// role not one it needs), and with `lean` its needs heavier still.
fn base_grams(food: &Chosen, def: &DishDef, lean: bool) -> u32 {
    let Some(main) = food.def.main else {
        return 40;
    };
    let grams = match main {
        Role::Starch => 300,
        Role::Protein | Role::Vegetable => 200,
        Role::Nut => 100,
        Role::Sweet => 80,
        Role::Fat => 50,
        Role::Acid | Role::Aromatic => 40,
        Role::Herb | Role::Spice => 1,
    };
    let needed = def.needs.contains(&main) || food.specific;
    match (needed, lean) {
        (true, true) => grams * 3 / 2,
        (true, false) => grams,
        (false, _) if main == Role::Fat || is_seasoning(main) => grams,
        (false, true) => 20,
        (false, false) => 40,
    }
}
