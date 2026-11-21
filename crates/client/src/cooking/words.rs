//! The panel's words, and an output as an item to show: what one run makes
//! at its least quality, its name, diet labels and Well Fed, the chance, the
//! kitchen, and why a process can't be run.

use bevy::prelude::*;
use shared::alchemy::CERTAIN;
use shared::cooking::{
    Dishes, Heat, Ingredients, OrderRefused, Output, Refused, Tool, WellFed, well_fed_ticks,
};
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::food::{Rest, food_item, tint};
use shared::id::Id;
use shared::item::{Item, Quality};
use shared::lang::Lang;

use super::kitchen::{KitchenNow, Place};
use crate::common::bonus_line;

/// An output as an item to show at `quality`.
pub(super) fn output_item(output: &Output, quality: Quality) -> Item {
    let ingredients = Ingredients::built_in();
    food_item(output, quality, None, ingredients, Id::stand_in(0), None)
}

/// Whether an output is an intermediate (it rests, and starts a later row)
/// rather than a dish.
pub(super) fn is_intermediate(output: &Output) -> bool {
    let tools = &output.history.tools;
    Rest::of(
        &output.dish,
        &output.portions,
        tools,
        Ingredients::built_in(),
    )
    .is_some()
}

/// "Common Fried Egg, at least".
pub(super) fn output_name(output: &Output) -> String {
    let item = output_item(output, output.quality);
    let name = item
        .top()
        .food()
        .map_or_else(|| output.dish.name(Dishes::built_in()), |food| food.name());
    format!("{:?} {name}, at least", output.quality)
}

/// An output's lines: its diet labels, and what eating it gives (Well Fed
/// for so long, restoring and changing), or that it is an intermediate.
pub(super) fn output_lines(output: &Output, lang: &Lang) -> Vec<String> {
    let mut lines = Vec::new();
    let labels = output.diet.labels();
    if !labels.is_empty() {
        lines.push(labels.join(", "));
    }
    if is_intermediate(output) {
        lines.push("Rests, then starts a later row".to_owned());
        return lines;
    }
    let ingredients = Ingredients::built_in();
    let fed = WellFed::of(&output.portions, output.history.cooked(), ingredients);
    let minutes = well_fed_ticks(output.quality) / (60 * ACTION_TICKS_PER_SECOND as u64);
    lines.push(format!("Well Fed for {minutes} minutes:"));
    for (amount, vital) in [(fed.health, "Health"), (fed.mana, "Mana")] {
        if amount > 0 {
            let amount = f64::from(amount) / 1000.0;
            lines.push(format!("  Restores {amount:.2} {vital} a second"));
        }
    }
    for bonus in &fed.bonuses {
        lines.push(format!("  {}", bonus_line(bonus, lang)));
    }
    lines
}

/// An output's own colour: its foods' tint.
pub(super) fn output_colour(output: &Output) -> Color {
    let [r, g, b] = tint(&output.portions, &output.history, Ingredients::built_in());
    Color::srgb_u8(r, g, b)
}

/// "Chance 100%", "Chance 12.5%".
pub(super) fn chance_words(chance: u32) -> String {
    let whole = chance / (CERTAIN / 100);
    let tenths = chance % (CERTAIN / 100) / (CERTAIN / 1000);
    match tenths {
        0 => format!("Chance {whole}%"),
        tenths => format!("Chance {whole}.{tenths}%"),
    }
}

/// Where the player is cooking, under the grid.
pub(super) fn kitchen_words(kitchen: &KitchenNow) -> &'static str {
    match kitchen.place {
        Place::Hearth => "At a lit hearth: every tool, its Oven, and no cap on quality.",
        Place::ColdHearth => {
            "At a hearth that isn't lit: its tools, but no heat. Light it with kindling."
        }
        Place::Town => "In town: the town's hearth, with its Oven and tools.",
        Place::Campfire => "By a campfire: Pan and Cauldron, no Oven; at most Rare.",
        Place::Nowhere => "No heat here: Knife, Bowl, Sieve and Mortar only; at most Rare.",
    }
}

/// Why a process can't be run, in words.
pub(super) fn refused_words(why: &OrderRefused, heat: Heat) -> String {
    match why {
        OrderRefused::Count => "Choose how many times.".to_owned(),
        OrderRefused::Tool(Tool::Oven) => "There's no Oven here: find a lit hearth.".to_owned(),
        OrderRefused::Tool(tool) => format!("No {} to hand: carry one.", tool.name()),
        OrderRefused::Source => "A row's intermediate isn't here.".to_owned(),
        OrderRefused::Food => "A food isn't carried, or can't be cooked.".to_owned(),
        OrderRefused::Liquid => "A liquid isn't carried, or can't be used.".to_owned(),
        OrderRefused::NoWaterFaced => "No water is faced.".to_owned(),
        OrderRefused::Culture => "The culture isn't carried.".to_owned(),
        OrderRefused::TooFew => "Not enough is carried for one run.".to_owned(),
        OrderRefused::Rules(why) => rules_words(why, heat).to_owned(),
    }
}

/// Why the rules refuse a process.
fn rules_words(why: &Refused, heat: Heat) -> &'static str {
    match why {
        Refused::Rows => "Too many rows.",
        Refused::NoSteps => "Add a step.",
        Refused::OutOfGrid { .. } => "A step is off the grid.",
        Refused::Overlap { .. } => "Two steps are in one place.",
        Refused::Split { .. } => "Two steps take what one step makes: join them instead.",
        Refused::NothingIn { .. } => "A step has nothing in it.",
        Refused::JoinMissing { .. } => "A joining step needs something in both rows.",
        Refused::NoLiquid { .. } => "A Cauldron or Sieve needs water or milk.",
        Refused::NoHeat { .. } if heat == Heat::Cold => "A Pan, Cauldron or Oven needs heat.",
        Refused::NoHeat { .. } => "That step needs more heat.",
        Refused::UnknownIngredient { .. } => "A food can't be cooked.",
        Refused::UnknownInSource { .. } => "A row's intermediate can't be cooked.",
        Refused::UnusedSource { .. } => "A row's intermediate goes into no step.",
    }
}
