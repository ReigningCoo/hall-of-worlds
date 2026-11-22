//! `client.read_recipe`: opens the reading panel on a real dish's recipe
//! (`dish`, its key) or on an entry of the character's recipe book (`book`,
//! counted from 0, latest first), as reading a scroll or the Recipes tab
//! would; with neither, only says what is on it. Answers with the recipe
//! shown and its written steps (`missing`: the kinds framed red). See
//! `docs/context/debug-api.md`.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{AddDebugMethod, BrpResult, Value, json, params, refused};
use shared::recipe::{Craft, Entry};

use super::super::ReadRecipe;
use super::super::kitchen::Kitchens;
use super::entry::Shown;
use super::have::Have;
use super::open::Reading;
use super::written::written;
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::inventory::Carried;

pub(super) fn add(app: &mut App) {
    app.add_debug_method("client.read_recipe", read_recipe);
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct ReadParams {
    dish: Option<String>,
    book: Option<usize>,
}

fn read_recipe(
    In(request): In<Option<Value>>,
    reading: Res<Reading>,
    (carried, sheet, kitchens, wording): (
        Res<Carried>,
        Res<CharacterSheet>,
        Res<Kitchens>,
        Res<Wording>,
    ),
    mut read: MessageWriter<ReadRecipe>,
) -> BrpResult {
    let asked: ReadParams = params(request)?;
    let entry = match (asked.dish, asked.book) {
        (Some(dish), _) => Some(Entry::Learnt {
            craft: Craft::Cooking,
            dish,
            scroll: false,
        }),
        (None, Some(at)) => {
            let entry = sheet.0.book.iter().nth(at);
            Some(
                entry
                    .cloned()
                    .ok_or_else(|| refused(format!("No entry {at}")))?,
            )
        }
        (None, None) => None,
    };
    let Some(entry) = entry.or_else(|| reading.entry.clone()) else {
        return Ok(json!({ "open": reading.is_open() }));
    };
    let shown = Shown::of(&entry).ok_or_else(|| refused("No recipe to show"))?;
    let have = Have::of(&shown.recipe, &carried.0, &kitchens.0);
    let steps: Vec<Value> = (written(&shown.recipe, &have, &wording.0).into_iter())
        .map(|line| json!({ "words": line.words, "missing": line.missing }))
        .collect();
    read.write(ReadRecipe(entry));
    Ok(json!({
        "dish": shown.dish,
        "cuisine": shown.cuisine,
        "beyond_skill": shown.recipe.beyond_skill(shared::cooking::CookingSkills::of(&sheet.0.skills)),
        "steps": steps,
    }))
}
