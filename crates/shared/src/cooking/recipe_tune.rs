//! How many of each food a dish's recipe takes (`recipe_build`): a common
//! food for a role it needs (`common_food`), and the counts nudged one at a
//! time (`tune`) until nothing it forbids shows, every role it needs counts
//! and its biggest role is one of them.

use super::mix::{Foods, RoleMix};
use super::recipe_layout::Placed;
use super::{
    DishDef, IngredientDef, Ingredients, Portion, Role, Source, Tool, Usage, Wanted, Wetness,
    is_liquid_food, is_seasoning, unit_grams,
};

/// The millilitres of a liquid food a recipe counts as one step of it.
const LIQUID_STEP: u32 = 50;

/// The most nudges `tune` makes.
const MOST_NUDGES: u32 = 60;

/// A food chosen for a recipe: any of `foods`, `def` the usual one's data,
/// `count` of `unit` grams (millilitres for a liquid food).
pub(super) struct Chosen<'a> {
    pub foods: Vec<Wanted>,
    pub def: &'a IngredientDef,
    /// One the dish must have.
    pub specific: bool,
    liquid: bool,
    unit: u32,
    count: u32,
}

impl<'a> Chosen<'a> {
    pub fn new(def: &'a IngredientDef, specific: bool) -> Self {
        Self::with_foods(
            vec![Wanted::One(def.source, def.key.clone())],
            def,
            specific,
        )
    }

    pub fn with_foods(foods: Vec<Wanted>, def: &'a IngredientDef, specific: bool) -> Self {
        let liquid = is_liquid_food(def.source, &def.key);
        let unit = match liquid {
            true => LIQUID_STEP,
            false => unit_grams(def.source, &def.key).unwrap_or(1),
        };
        Self {
            foods,
            def,
            specific,
            liquid,
            unit,
            count: 1,
        }
    }

    /// As near `grams` as whole ones go, at least one.
    pub fn set_grams(&mut self, grams: u32) {
        self.count = ((grams + self.unit / 2) / self.unit).max(1);
    }

    pub fn grams(&self) -> u32 {
        self.count * self.unit
    }

    /// As a recipe input waiting for its place.
    pub fn placed(&self) -> Placed {
        let starch = self.def.main == Some(Role::Starch);
        match self.liquid {
            true => Placed::liquid(self.foods.clone(), self.grams(), starch),
            false => Placed::food(self.foods.clone(), self.count, self.grams(), starch),
        }
    }
}

/// A common food not yet chosen whose main role is `role`, that brings
/// nothing the dish forbids (nor, if one can be found, anything it doesn't
/// need beyond fats and seasonings): seasonings by the smallest item, acids,
/// sweets from honey first, acids and fats from fruit first, the rest from the garden first; then the
/// fewest roles, then the table's order.
pub(super) fn common_food<'a>(
    role: Role,
    def: &DishDef,
    chosen: &[Chosen],
    ingredients: &'a Ingredients,
) -> Option<&'a IngredientDef> {
    let unit = |food: &IngredientDef| unit_grams(food.source, &food.key);
    let rank = |source: Source| {
        let order = match role {
            Role::Sweet => [
                Source::Honey,
                Source::Fruit,
                Source::Vegetable,
                Source::Herb,
            ],
            Role::Acid | Role::Fat => [
                Source::Fruit,
                Source::Vegetable,
                Source::Herb,
                Source::Honey,
            ],
            _ => [
                Source::Vegetable,
                Source::Fruit,
                Source::Herb,
                Source::Honey,
            ],
        };
        let others = [Source::Egg, Source::Milk, Source::Fungus];
        let others = others
            .iter()
            .chain(&[Source::Pantry, Source::Meat, Source::Part]);
        order
            .iter()
            .chain(others)
            .position(|s| *s == source)
            .unwrap_or(99)
    };
    let fits = |food: &&IngredientDef| {
        food.main == Some(role)
            && matches!(food.usage, Usage::Staple | Usage::Common)
            && food.max_share >= if is_seasoning(role) { 1 } else { 20 }
            && !food.roles.iter().any(|r| def.forbids.contains(r))
            && (is_liquid_food(food.source, &food.key) || unit(food).is_some())
            && !chosen.iter().any(|c| std::ptr::eq(c.def, *food))
    };
    let plain = |food: &&IngredientDef| {
        (food.roles.iter()).all(|r| def.needs.contains(r) || *r == Role::Fat || is_seasoning(*r))
    };
    let pick = |strict: bool| {
        (ingredients.iter().enumerate())
            .filter(|(_, food)| fits(food) && (!strict || plain(food)))
            .min_by_key(|(at, food)| {
                let small = match is_seasoning(role) {
                    true => unit(food).unwrap_or(u32::MAX),
                    false => 0,
                };
                (small, rank(food.source), food.roles.len(), *at)
            })
            .map(|(_, food)| food)
    };
    pick(true).or_else(|| pick(false))
}

/// Nudges the counts, one at a time, until nothing `def` forbids shows,
/// every role it needs counts (where a food of it is chosen) and its biggest role is
/// one it needs, and its milk leaves it no wetter than it should be; stops
/// early when no food can help.
pub(super) fn tune(def: &DishDef, chosen: &mut [Chosen], ingredients: &Ingredients) {
    for _ in 0..MOST_NUDGES {
        let portions: Vec<Portion> = (chosen.iter())
            .map(|c| Portion::Food {
                source: c.def.source,
                key: c.def.key.clone(),
                grams: c.grams(),
                quality: None,
            })
            .collect();
        let foods = Foods::of(&portions, ingredients);
        let (full, beyond) = (RoleMix::of(&foods), RoleMix::beyond_leeway(&foods));
        let of_role = |role: Role| chosen.iter().position(|c| c.def.main == Some(role));
        let missing = (def.needs.iter()).find_map(|&role| match full.has(role) {
            true => None,
            false => of_role(role),
        });
        let at = if def.forbids.iter().any(|&role| beyond.has(role)) {
            booster(def, chosen)
        } else if missing.is_some() {
            missing
        } else if !def.needs.is_empty()
            && !beyond
                .biggest()
                .is_some_and(|role| def.needs.contains(&role))
        {
            booster(def, chosen)
        } else if too_wet(def, chosen) {
            let milk = chosen
                .iter()
                .position(|c| c.def.source == Source::Milk && c.count > 1);
            if let Some(at) = milk {
                chosen[at].count -= 1;
                continue;
            }
            booster(def, chosen)
        } else {
            return;
        };
        match at {
            Some(at) => chosen[at].count += 1,
            None => return,
        }
    }
}

/// Whether the milk alone makes it wetter than it should be (water is added
/// after, for the rest). Milk through a Sieve loses its whey.
fn too_wet(def: &DishDef, chosen: &[Chosen]) -> bool {
    if def.tools.contains(&Tool::Sieve) {
        return false;
    }
    let milk: u32 = (chosen.iter())
        .filter(|c| c.def.source == Source::Milk)
        .map(Chosen::grams)
        .sum();
    let all: u32 = chosen.iter().map(Chosen::grams).sum();
    Wetness::of(milk, all) > def.liquid
}

/// The food to add one more of to make the dish more what it needs: the
/// largest whose main role it needs (not a seasoning) and that brings
/// nothing it forbids.
fn booster(def: &DishDef, chosen: &[Chosen]) -> Option<usize> {
    (chosen.iter().enumerate())
        .filter(|(_, c)| {
            c.def
                .main
                .is_some_and(|m| def.needs.contains(&m) && !is_seasoning(m))
                && !c.def.roles.iter().any(|r| def.forbids.contains(r))
        })
        .max_by_key(|(at, c)| (c.unit, std::cmp::Reverse(*at)))
        .map(|(at, _)| at)
}
