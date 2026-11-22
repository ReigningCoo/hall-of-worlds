//! Shows the recipe on the panel, again whenever it, what is carried, the
//! skills or the kitchen change: the title, where the dish is from or "Your
//! recipe", the grid, the note when it is beyond the reader's Cooking skill
//! (how many steps, and the chance), the written steps with what is missing
//! framed red, and what it makes.

use bevy::prelude::*;
use shared::cooking::CookingSkills;
use shared::lang::{Args, Lang};

use super::super::kitchen::Kitchens;
use super::super::spaces::SpaceIcons;
use super::draw::{RecipeGrid, draw_recipe};
use super::entry::Shown;
use super::have::Have;
use super::open::Reading;
use super::written::{render, written};
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::dom::Dom;
use crate::inventory::Carried;

/// Where the form's ids are.
const FORM: &str = "recipe";

/// What showing reads.
type Reads<'w> = (
    Res<'w, Reading>,
    Res<'w, Carried>,
    Res<'w, CharacterSheet>,
    Res<'w, Kitchens>,
    Res<'w, SpaceIcons>,
    Res<'w, Wording>,
);

pub(super) fn show_reading(
    (reading, carried, sheet, kitchens, icons, wording): Reads,
    canvases: Query<Entity, With<RecipeGrid>>,
    added: Query<(), Added<RecipeGrid>>,
    mut dom: Dom,
    mut commands: Commands,
) {
    let changed = reading.is_changed() || carried.is_changed() || sheet.is_changed();
    if !(changed || kitchens.is_changed() || !added.is_empty()) {
        return;
    }
    let Some(shown) = reading.entry.as_ref().and_then(Shown::of) else {
        return;
    };
    let Ok(canvas) = canvases.single() else {
        return;
    };
    let lang = &wording.0;
    let have = Have::of(&shown.recipe, &carried.0, &kitchens.0);
    draw_recipe(&mut commands, canvas, (&shown.recipe, &have), &icons);
    dom.set(&format!("{FORM}/panel"), "title", &shown.dish);
    show_whose(&mut dom, &shown, lang);
    let skills = CookingSkills::of(&sheet.0.skills);
    let beyond = shown.recipe.beyond_skill(skills).then(|| {
        let over = shown.recipe.steps_beyond(skills).max(1);
        let chance = shown.recipe.chance(skills) / 100;
        let args = Args::new()
            .count("over", i64::from(over))
            .count("chance", i64::from(chance));
        render(lang, "recipe-beyond", &args)
    });
    dom.set_flag(&format!("{FORM}/beyond"), "hidden", beyond.is_none());
    dom.set_text(&format!("{FORM}/beyond"), beyond.unwrap_or_default());
    dom.empty(&format!("{FORM}/steps"));
    for (at, line) in written(&shown.recipe, &have, lang).into_iter().enumerate() {
        let id = format!("{FORM}/step.{at}");
        dom.copy_template(&format!("{FORM}/step"), &format!("{FORM}/steps"), at);
        dom.set_text(&id, line.words);
        let missing: Vec<String> = line.missing.iter().map(usize::to_string).collect();
        dom.set(&id, "missing", &missing.join(" "));
    }
    let makes = Args::new().text("dish", shown.dish.clone());
    dom.set_text(
        &format!("{FORM}/makes"),
        render(lang, "recipe-makes", &makes),
    );
}

/// Where a real dish is from, or the quill and "Your recipe".
fn show_whose(dom: &mut Dom, shown: &Shown, lang: &Lang) {
    let own = shown.cuisine.is_none();
    dom.set_flag(&format!("{FORM}/own"), "hidden", !own);
    dom.set_flag(&format!("{FORM}/cuisine"), "hidden", own);
    let words = match &shown.cuisine {
        Some(cuisine) => {
            let args = Args::new().text("cuisine", cuisine.clone());
            dom.set_text(
                &format!("{FORM}/cuisine"),
                render(lang, "recipe-cuisine", &args),
            );
            return;
        }
        None => render(lang, "recipe-own", &Args::new()),
    };
    dom.set_text(&format!("{FORM}/own_words"), words);
}
