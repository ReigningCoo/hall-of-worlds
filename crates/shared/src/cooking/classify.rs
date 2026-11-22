//! Naming what a process makes (owner, 2026-11-21): the real name of the
//! closest dish in `Dishes` when it is close to the true recipe or style,
//! else a generic name from its form and biggest foods. Herbs and spices
//! within their max share never change what a dish is (the leeway).

use serde::{Deserialize, Serialize};

use super::mix::{Foods, RoleMix};
use super::names::food_name;
use super::{DishDef, DishForm, Dishes, Generic, Ingredients, Portion, Shape, Source, Wants};
use super::{Role, Usage, Wetness};

/// The most foods a generic name names.
pub const NAMED_FOODS: usize = 2;

/// What an output is called.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DishName {
    /// A real dish, by its key in `Dishes`.
    Real(String),
    /// A generic dish of these foods, biggest first.
    Generic {
        kind: Generic,
        foods: Vec<(Source, String)>,
    },
}

/// What an output is: its name and what it is served as.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dish {
    pub name: DishName,
    pub form: DishForm,
}

impl DishName {
    /// It in English.
    pub fn name(&self, dishes: &Dishes) -> String {
        match self {
            Self::Real(key) => dishes
                .get(key)
                .map_or_else(|| key.clone(), |d| d.name.clone()),
            Self::Generic { kind, foods } => {
                let names: Vec<String> = foods.iter().map(|(s, k)| food_name(*s, k)).collect();
                kind.name(&names)
            }
        }
    }
}

impl Dish {
    /// Its name in English.
    pub fn name(&self, dishes: &Dishes) -> String {
        self.name.name(dishes)
    }

    /// What it wants, for balance: a real dish's roles and foods, a generic
    /// one's roles.
    pub fn wants(&self, dishes: &Dishes) -> Wants {
        match &self.name {
            DishName::Real(key) => dishes.get(key).map_or_else(Wants::default, |def| Wants {
                roles: def.needs.clone(),
                exempt: def.specific.clone(),
            }),
            DishName::Generic { kind, .. } => Wants {
                roles: kind.wants().to_vec(),
                exempt: Vec::new(),
            },
        }
    }
}

/// What an output with this shape holding these portions is.
pub fn classify(
    portions: &[Portion],
    shape: &Shape,
    ingredients: &Ingredients,
    dishes: &Dishes,
) -> Dish {
    let foods = Foods::of(portions, ingredients);
    let full = RoleMix::of(&foods);
    let beyond = RoleMix::beyond_leeway(&foods);
    let closest = (dishes.iter().enumerate())
        .filter_map(|(at, def)| Some((closeness(def, shape, &foods, &full, &beyond)?, at, def)))
        .min_by_key(|(score, at, _)| (*score, *at));
    if let Some((_, _, def)) = closest {
        return Dish {
            name: DishName::Real(def.key.clone()),
            form: def.form,
        };
    }
    let kind = Generic::of(shape, &foods, &beyond);
    Dish {
        name: DishName::Generic {
            kind,
            foods: named_foods(&foods),
        },
        form: kind.form(),
    }
}

/// How close an output is to `def`, smaller closer (tools apart, wetness
/// apart, needs missing, less the usual foods there, less how well known),
/// or `None` if it isn't close: the same last tool and joining or not, at
/// most one tool step more or less, every one of its foods, its needs (but
/// for a fat, often only what it is cooked in, which costs balance), the
/// biggest role one of them, a liquid if it wants one, none of what it
/// forbids beyond the leeway, and wetness at most a band apart. A dish
/// naming no foods is a style of mixing them, so it takes two or more. And
/// nothing foreign to it: each food beyond the leeway is one it names, or
/// mostly a role it needs, a seasoning or a fat.
fn closeness(
    def: &DishDef,
    shape: &Shape,
    foods: &Foods,
    full: &RoleMix,
    beyond: &RoleMix,
) -> Option<(u32, u32, u32, i32, i32)> {
    let has = |group: &Vec<super::Wanted>| {
        (foods.0.iter()).any(|(food, _)| group.iter().any(|w| w.matches(food.source, &food.key)))
    };
    let distance = shape.distance(&def.tools);
    let apart = shape.wetness.apart(def.liquid);
    let close = def.last() == shape.last()
        && (def.joins > 0) == (shape.joins > 0)
        && distance <= 1
        && apart <= 1
        && def.specific.iter().all(has)
        && (!def.specific.is_empty() || mixed(foods))
        && !foreign(def, foods)
        && !def.forbids.iter().any(|&role| beyond.has(role))
        && (def.needs.is_empty() || beyond.biggest().is_some_and(|r| def.needs.contains(&r)));
    if !close {
        return None;
    }
    let missing: Vec<Role> = (def.needs.iter().copied())
        .filter(|&role| !full.has(role))
        .collect();
    let dry = def.wants_liquid && shape.wetness == Wetness::Dry;
    if dry || missing.iter().any(|&role| role != Role::Fat) {
        return None;
    }
    let missing = missing.len() as u32;
    let usual: i32 = def
        .with
        .iter()
        .map(|group| if has(group) { 1 } else { -1 })
        .sum();
    Some((distance, apart, missing, -usual, -i32::from(def.known)))
}

/// The biggest foods by weight, up to `NAMED_FOODS` with different names:
/// herbs and spices within their leeway and medicinal herbs left out, unless
/// nothing else is there.
fn named_foods(foods: &Foods) -> Vec<(Source, String)> {
    let mut sorted: Vec<(&super::IngredientDef, u32)> = foods.0.clone();
    sorted.sort_by_key(|(_, grams)| std::cmp::Reverse(*grams));
    let plain = |&(def, grams): &(&super::IngredientDef, u32)| {
        def.usage != Usage::MedicinalOnly && !foods.within_leeway(def, grams)
    };
    let pick: Vec<_> = match sorted.iter().any(plain) {
        true => sorted.into_iter().filter(plain).collect(),
        false => sorted,
    };
    let mut named: Vec<(Source, String)> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for (def, _) in pick {
        let name = food_name(def.source, &def.key);
        if names.len() < NAMED_FOODS && !names.contains(&name) {
            names.push(name);
            named.push((def.source, def.key.clone()));
        }
    }
    named
}

/// Whether more than one food is in it beyond the leeway (medicinal herbs
/// left out).
fn mixed(foods: &Foods) -> bool {
    let plain = (foods.0.iter()).filter(|(def, grams)| {
        def.usage != Usage::MedicinalOnly && !foods.within_leeway(def, *grams)
    });
    plain.count() > 1
}

/// Whether a food beyond the leeway is foreign to `def`: not one it names,
/// and mostly a role it doesn't need, nor a seasoning or a fat.
fn foreign(def: &DishDef, foods: &Foods) -> bool {
    (foods.0.iter()).any(|&(food, grams)| {
        let named = (def.specific.iter().chain(&def.with).flatten())
            .any(|w| w.matches(food.source, &food.key));
        let fits = food.main.is_none_or(|role| {
            def.needs.contains(&role) || role == Role::Fat || super::is_seasoning(role)
        });
        let beyond = food.usage != Usage::MedicinalOnly && !foods.within_leeway(food, grams);
        beyond && !named && !fits
    })
}
