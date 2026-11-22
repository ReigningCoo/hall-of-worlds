//! Showing the Recipes tab: which page is shown, the requester while a
//! removal is asked about, and the list again whenever the book, the Cooking
//! skill or whether a run is under way changes: each row the dish (an own
//! recipe's in ink, with a quill), where it is from or "Your recipe", and
//! the beyond-skill note.

use bevy::prelude::*;
use shared::cooking::CookingSkills;
use shared::lang::{Args, Lang};
use shared::recipe::Entry;

use super::super::bench::Bench;
use super::super::beyond::beyond_words;
use super::super::reading::Shown;
use super::{Recipes, TABS};
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::dom::Dom;

/// Where the form's ids are.
const FORM: &str = "cooking";

pub(in crate::cooking) fn show_recipes(
    bench: Res<Bench>,
    mut recipes: ResMut<Recipes>,
    (sheet, wording): (Res<CharacterSheet>, Res<Wording>),
    mut dom: Dom,
) {
    let Some(document) = bench.document else {
        return;
    };
    if recipes.document != Some(document) {
        // A panel just opened: on its Cooking tab, the list drawn afresh.
        recipes.document = Some(document);
        recipes.tab = 0;
        recipes.asking = None;
        recipes.drawn_for = None;
        recipes.listed.clear();
    }
    if recipes.is_changed() {
        dom.set(&format!("{FORM}/tabs"), "value", TABS[recipes.tab]);
        dom.set_flag(&format!("{FORM}/cooking_page"), "hidden", recipes.tab != 0);
        dom.set_flag(&format!("{FORM}/recipes_page"), "hidden", recipes.tab != 1);
        show_asking(&mut dom, recipes.asking.as_ref());
    }
    let skills = CookingSkills::of(&sheet.0.skills);
    let book: Vec<Entry> = sheet.0.book.iter().cloned().collect();
    let now = Some((skills.cooking_percent(), bench.is_running()));
    if recipes.listed == book && recipes.drawn_for == now {
        return;
    }
    recipes.drawn_for = now;
    let list = format!("{FORM}/recipes");
    dom.empty(&list);
    dom.set_flag(&format!("{FORM}/no_recipes"), "hidden", !book.is_empty());
    dom.set_flag(&format!("{FORM}/recipes_scroll"), "hidden", book.is_empty());
    for (at, entry) in book.iter().enumerate() {
        dom.copy_template(&format!("{FORM}/recipe"), &list, at);
        show_row(
            &mut dom,
            (at, entry),
            (skills, bench.is_running()),
            &wording.0,
        );
    }
    recipes.listed = book;
}

/// One row of the list.
fn show_row(
    dom: &mut Dom,
    (at, entry): (usize, &Entry),
    (skills, running): (CookingSkills, bool),
    lang: &Lang,
) {
    let id = |name: &str| format!("{FORM}/{name}.{at}");
    let Some(shown) = Shown::of(entry) else {
        dom.set_text(&id("dish"), "A recipe of another craft");
        for button in ["read", "make"] {
            dom.set_flag(&id(button), "disabled", true);
        }
        return;
    };
    let own = shown.cuisine.is_none();
    dom.set_flag(&id("quill"), "hidden", !own);
    dom.set_flag(&id("dish"), "hidden", own);
    dom.set_flag(&id("own_dish"), "hidden", !own);
    dom.set_text(&id(if own { "own_dish" } else { "dish" }), &shown.dish);
    let whose = match &shown.cuisine {
        Some(cuisine) => {
            let args = Args::new().text("cuisine", cuisine.clone());
            render(lang, "recipe-cuisine", &args)
        }
        None => render(lang, "recipe-own", &Args::new()),
    };
    dom.set_text(&id("whose"), whose);
    let beyond = beyond_words(shown.recipe.halves(), skills, lang);
    dom.set_flag(&id("recipe_beyond"), "hidden", beyond.is_none());
    dom.set_text(&id("recipe_beyond"), beyond.unwrap_or_default());
    dom.set_flag(&id("make"), "disabled", running);
}

/// The requester asking before a recipe is removed, up while one is asked
/// about.
fn show_asking(dom: &mut Dom, asking: Option<&Entry>) {
    dom.set_flag(&format!("{FORM}/asking"), "hidden", asking.is_none());
    let Some(entry) = asking else {
        return;
    };
    let dish = Shown::of(entry).map_or_else(|| "this recipe".to_owned(), |shown| shown.dish);
    let question = match entry {
        Entry::Learnt { scroll: true, .. } => {
            format!("Remove the recipe for {dish} from your book? You get its scroll back.")
        }
        Entry::Learnt { .. } => format!("Remove the recipe for {dish} from your book?"),
        Entry::Own(_) => {
            format!("Remove your own recipe for {dish}? It comes back only if you cook it again.")
        }
    };
    dom.set_text(&format!("{FORM}/question"), question);
}

fn render(lang: &Lang, id: &str, args: &Args) -> String {
    lang.render(id, args).unwrap_or_else(|error| error.text)
}
