//! Keeping the mixing panel up to date: its rows from what is carried and the
//! recipes, and what the choice would make, through `Dom`.

use bevy::prelude::*;
use shared::brewing::{BrewSkills, GrindOrder, most_doses};
use shared::herb::HerbKnowledge;
use shared::item::Inventory;

use super::Mixing;
use super::markup::{NO_HERBS, PICK};
use super::words::{herb_row, makes, recipe_row};
use crate::character::CharacterSheet;
use crate::common::HerbTable;
use crate::dom::Dom;
use crate::inventory::Carried;

/// Up while choosing; its lists follow what is carried and the recipes, and
/// what it would make follows the choice.
pub(in crate::abilities) fn show_mixing(
    mut mixing: ResMut<Mixing>,
    (carried, sheet, table): (Res<Carried>, Res<CharacterSheet>, Res<HerbTable>),
    mut dom: Dom,
    mut was_open: Local<bool>,
) {
    let opened = mixing.open && !*was_open;
    if mixing.open != *was_open {
        *was_open = mixing.open;
        dom.set_flag("grind/mixing", "hidden", !mixing.open);
    }
    if !mixing.open {
        return;
    }
    let changed = carried.is_changed() || sheet.is_changed() || table.is_changed();
    if opened || changed {
        let known = &sheet.0.herbs;
        list_herbs(&mut mixing, &carried.0, (&*table, known), opened, &mut dom);
        list_oils(&mut mixing, &carried.0, opened, &mut dom);
        list_recipes(
            &mut mixing,
            (&carried.0, &sheet.0.recipes),
            (&*table, known),
            opened,
            &mut dom,
        );
    }
    if opened || mixing.dirty {
        mixing.dirty = false;
        show_choice(&mut mixing, &carried.0, &sheet, &table, &mut dom);
    }
}

type Names<'a> = (&'a HerbTable, &'a HerbKnowledge);

/// A checkbox for each herb stack carried; ticks kept on stacks still there.
fn list_herbs(mixing: &mut Mixing, carried: &Inventory, names: Names, all: bool, dom: &mut Dom) {
    let herbs = carried.herb_stacks();
    if herbs == mixing.herbs && !all {
        return;
    }
    dom.empty("grind/herbs");
    for (at, stack) in herbs.iter().enumerate() {
        dom.copy_template("grind/herb_row", "grind/herbs", at);
        dom.set_text(
            &format!("grind/herb.{at}"),
            herb_row(stack, &names.0.0, names.1),
        );
    }
    mixing
        .chosen
        .retain(|id| herbs.iter().any(|stack| stack.stack == *id));
    dom.set_text(
        "grind/words",
        if herbs.is_empty() { NO_HERBS } else { PICK },
    );
    mixing.herbs = herbs;
    mixing.dirty = true;
}

/// The oils carried, the one chosen kept if it is still there.
fn list_oils(mixing: &mut Mixing, carried: &Inventory, all: bool, dom: &mut Dom) {
    let oils = carried.oil_stacks();
    if oils == mixing.oils && !all {
        return;
    }
    let was = mixing.oils.get(mixing.oil).map(|oil| oil.stack);
    mixing.oil = (oils.iter())
        .position(|oil| Some(oil.stack) == was)
        .unwrap_or(0);
    let options: Vec<(String, String)> = match oils.is_empty() {
        true => vec![("0".into(), "No oil carried".into())],
        false => (oils.iter().enumerate())
            .map(|(at, oil)| {
                (
                    at.to_string(),
                    format!("{} ({})", oil.oil.name(), oil.count),
                )
            })
            .collect(),
    };
    dom.set_options("grind/oil", options);
    dom.set("grind/oil", "value", &mixing.oil.to_string());
    mixing.oils = oils;
    mixing.dirty = true;
}

/// A row for each mix made before, its Use button off unless all of it is
/// carried.
fn list_recipes(
    mixing: &mut Mixing,
    (carried, recipes): (&Inventory, &shared::brewing::Recipes),
    names: Names,
    all: bool,
    dom: &mut Dom,
) {
    let recipes: Vec<_> = recipes.iter().cloned().collect();
    if recipes == mixing.recipes && !all && !mixing.dirty {
        return;
    }
    dom.empty("grind/recipes");
    let herbs = carried.herb_stacks();
    let oils = carried.oil_stacks();
    for (at, recipe) in recipes.iter().enumerate() {
        dom.copy_template("grind/recipe_row", "grind/recipes", at);
        dom.set_text(
            &format!("grind/recipe.{at}"),
            recipe_row(recipe, &names.0.0, names.1),
        );
        let has_herbs =
            (recipe.herbs.iter()).all(|key| herbs.iter().any(|stack| &stack.herb == key));
        let has_oil = oils.iter().any(|oil| oil.oil == recipe.oil);
        dom.set_flag(
            &format!("grind/again.{at}"),
            "disabled",
            !(has_herbs && has_oil),
        );
    }
    dom.set_flag("grind/before_title", "hidden", recipes.is_empty());
    dom.set_flag("grind/recipe_scroll", "hidden", recipes.is_empty());
    mixing.recipes = recipes;
}

/// The ticks, the doses there is enough for, and what it would make.
fn show_choice(
    mixing: &mut Mixing,
    carried: &Inventory,
    sheet: &CharacterSheet,
    table: &HerbTable,
    dom: &mut Dom,
) {
    for (at, stack) in mixing.herbs.iter().enumerate() {
        let ticked = mixing.chosen.contains(&stack.stack);
        dom.set_flag(&format!("grind/herb.{at}"), "checked", ticked);
    }
    let skills = BrewSkills::of(&sheet.0.skills);
    let fewest = (mixing.herbs.iter())
        .filter(|stack| mixing.chosen.contains(&stack.stack))
        .map(|stack| stack.count)
        .min();
    let most = most_doses(skills).min(fewest.unwrap_or(1)).max(1);
    mixing.doses = mixing.doses.clamp(1, most);
    let options = (1..=most).map(|n| ((n - 1).to_string(), n.to_string()));
    dom.set_options("grind/doses", options);
    dom.set("grind/doses", "value", &(mixing.doses - 1).to_string());
    dom.set("grind/oil", "value", &mixing.oil.to_string());
    let order = mixing.order();
    let made = carried.check_grind(&order, &table.0, skills);
    let known = &sheet.0.herbs;
    let foreseen = (mixing.herbs.iter())
        .filter(|stack| mixing.chosen.contains(&stack.stack))
        .all(|stack| {
            table
                .0
                .get(&stack.herb)
                .is_some_and(|def| (0..def.effects.len()).all(|n| known.knows_effect(&def.key, n)))
        });
    let made = made.map(|(ointment, _)| ointment);
    dom.set_text("grind/makes", makes(&made, foreseen, skills));
    dom.set_flag("grind/grind", "disabled", made.is_err());
}

impl Mixing {
    /// What is chosen, as asked of the server.
    pub(super) fn order(&self) -> GrindOrder {
        GrindOrder {
            herbs: self.chosen.clone(),
            oil: self
                .oils
                .get(self.oil)
                .map_or(shared::id::Id::NOBODY, |oil| oil.stack),
            doses: self.doses.max(1),
        }
    }
}
