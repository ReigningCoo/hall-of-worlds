//! What the player does on the Recipes tab: choosing a tab; Read opens the
//! reading panel on the recipe; Make lays it on the bench, filled from what
//! is carried (`Bench::load`), and goes back to the Cooking tab, where the
//! player presses Start; Remove asks first, and Remove in the requester asks
//! the server (`RecipeAsk::Remove`), Keep or a press outside leaves it.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, RecipeAsk};

use super::super::ReadRecipe;
use super::super::bench::Bench;
use super::super::kitchen::Kitchens;
use super::super::reading::Shown;
use super::Recipes;
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};
use crate::inventory::Carried;

pub(in crate::cooking) fn answer_recipes(
    mut events: MessageReader<UiEvent>,
    mut recipes: ResMut<Recipes>,
    mut bench: ResMut<Bench>,
    (carried, kitchens): (Res<Carried>, Res<Kitchens>),
    mut read: MessageWriter<ReadRecipe>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("cooking", name) = event.id.parts() else {
            continue;
        };
        let entry = (event.id.index()).and_then(|at| recipes.listed.get(at).cloned());
        match (name, &event.value, entry) {
            ("tabs", UiValue::Choice { index, .. }, _) if *index != recipes.tab => {
                recipes.tab = *index;
            }
            ("read", UiValue::Pressed, Some(entry)) => {
                read.write(ReadRecipe(entry));
            }
            ("make", UiValue::Pressed, Some(entry)) if !bench.is_running() => {
                if let Some(shown) = Shown::of(&entry) {
                    bench.load(&shown.recipe, &carried.0, &kitchens.0);
                    recipes.tab = 0;
                }
            }
            ("remove", UiValue::Pressed, Some(entry)) => recipes.asking = Some(entry),
            ("really_remove", UiValue::Pressed, _) => {
                if let Some(entry) = recipes.asking.take() {
                    let ask = RecipeAsk::Remove(entry);
                    to_server.write(ToServer(ClientMessage::Recipe(ask)));
                }
            }
            ("keep", UiValue::Pressed, _) | ("asking", UiValue::Dismissed, _) => {
                recipes.asking = None;
            }
            _ => {}
        }
    }
}
