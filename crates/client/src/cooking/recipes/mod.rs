//! The Cooking panel's Recipes tab (recipes plan PR 5,
//! `docs/context/recipes.md`): the tab bar (Cooking, Recipes), the recipe
//! book listed latest first (own recipes in ink with a quill), each with
//! Read (the reading panel), Make (the recipe onto the bench, `ghosts`, then
//! back to the Cooking tab) and Remove (asked first: a learnt one read from
//! a scroll gives the scroll back), and the beyond-skill note.

mod answer;
mod show;

use bevy::prelude::*;
use shared::recipe::Entry;

pub(super) use answer::answer_recipes;
pub(super) use show::show_recipes;

/// The tab bar's tabs.
pub(super) const TABS: [&str; 2] = ["Cooking", "Recipes"];

/// The Recipes tab: which tab is shown, the entries listed (a row's index is
/// its place here), and the recipe whose removal is being asked about.
#[derive(Resource, Default)]
pub(super) struct Recipes {
    /// The panel's document these were shown on: a new one starts on Cooking.
    pub(super) document: Option<Entity>,
    pub(super) tab: usize,
    pub(super) listed: Vec<Entry>,
    /// What the list was last drawn for: the Cooking skill and whether a run
    /// was under way.
    pub(super) drawn_for: Option<(u32, bool)>,
    pub(super) asking: Option<Entry>,
}
