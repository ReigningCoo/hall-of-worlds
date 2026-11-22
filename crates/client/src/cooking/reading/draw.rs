//! The recipe's grid, read only, drawn as the Cooking panel's
//! (`cooking::grid`): each step's tool box (a join two rows tall) with its
//! foods, liquids and water under it (a row's first step's starting with the
//! row's intermediate to start from, if any), each the kind's usual item with how many (liquids in
//! 100 ml, as the Cooking panel counts them). What is not to hand is red.

use bevy::prelude::*;
use shared::cooking::{Recipe, RecipeInput, RecipeStep};
use shared::item_text::KindToken;

use super::super::boxes::{Picture, space_shown, spawn_box, tool_item};
use super::super::grid_nodes::{at, spaces_node};
use super::super::spaces::{ML_STEP, Place, SpaceIcons};
use super::have::{Have, foods_named, input_token};
use crate::dom::Solid;
use crate::widgets::{HoverTip, UiId};

/// The grid's canvas.
#[derive(Component)]
pub(super) struct RecipeGrid;

/// Draws the whole grid into `canvas`.
pub(super) fn draw_recipe(
    commands: &mut Commands,
    canvas: Entity,
    (recipe, have): (&Recipe, &Have),
    icons: &SpaceIcons,
) {
    let columns = (recipe.steps.iter())
        .map(|step| step.column + 1)
        .max()
        .unwrap_or(0);
    let rows = recipe.rows.max(1);
    let node = Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::auto(u16::from(columns)),
        grid_template_rows: RepeatedGridTrack::auto(u16::from(rows)),
        column_gap: px(8),
        row_gap: px(8),
        justify_content: JustifyContent::Start,
        justify_items: JustifyItems::Start,
        ..default()
    };
    commands.entity(canvas).despawn_related::<Children>();
    commands.entity(canvas).with_children(|within| {
        within.spawn(node).with_children(|grid| {
            for (s, step) in recipe.steps.iter().enumerate() {
                draw_step(grid, (recipe, s, step), have, icons);
            }
        });
    });
}

/// The recipe's first step on `row` (the least column covering it).
fn first_in_row(recipe: &Recipe, row: usize) -> Option<usize> {
    let covers =
        |step: &RecipeStep| (step.row..step.row + step.height.max(1)).contains(&(row as u8));
    (recipe.steps.iter().enumerate())
        .filter(|(_, step)| covers(step))
        .min_by_key(|(_, step)| step.column)
        .map(|(at, _)| at)
}

/// A row's intermediate to start from, among its first step's spaces.
fn draw_source(
    spaces: &mut ChildSpawnerCommands,
    (recipe, row): (&Recipe, usize),
    have: &Have,
    icons: &SpaceIcons,
) {
    let Some(Some(rest)) = recipe.sources.get(row) else {
        return;
    };
    let tip = format!("Your {}, to start the row from", rest.name().to_lowercase());
    let missing = !have.source(row);
    let icon = icons.of(Place::Source(row as u8));
    let shown = space_shown(None, icon, missing.then_some(false));
    let marks = (HoverTip(with_missing(tip, missing)), Solid);
    spaces.spawn((shown, marks, UiId::new("recipe", "source").at(row)));
}

/// A step's tool box and its spaces, in its place.
fn draw_step(
    grid: &mut ChildSpawnerCommands,
    (recipe, s, step): (&Recipe, usize, &RecipeStep),
    have: &Have,
    icons: &SpaceIcons,
) {
    let picture = match tool_item(step.tool) {
        Some(kind) => Picture::Item(kind),
        None => Picture::Icon(icons.tool(step.tool)),
    };
    let missing = !have.tool(s);
    let name = step.tool.name();
    let tip = match step.height > 1 {
        true => format!("{name}, joining both rows"),
        false => name.to_owned(),
    };
    let tip = with_missing(tip, missing);
    let height = f32::from(step.height.max(1));
    grid.spawn(at(step.column, step.row, step.height.max(1)))
        .with_children(|column| {
            let marks = UiId::new("recipe", "tool").at(s);
            let edge = have.tool_edge(s);
            spawn_box(column, (picture, edge, height), tip, marks, |_| {});
            column.spawn(spaces_node()).with_children(|spaces| {
                for row in 0..recipe.sources.len() {
                    if first_in_row(recipe, row) == Some(s) {
                        draw_source(spaces, (recipe, row), have, icons);
                    }
                }
                for (i, input) in step.inputs.iter().enumerate() {
                    draw_input(spaces, (s, i, input), have, icons);
                }
            });
        });
}

/// One input's space: the kind's usual item and how many.
fn draw_input(
    spaces: &mut ChildSpawnerCommands,
    (s, i, input): (usize, usize, &RecipeInput),
    have: &Have,
    icons: &SpaceIcons,
) {
    let item = input_token(input).as_ref().and_then(KindToken::stand_in);
    let (count, place, words) = match input {
        RecipeInput::Food { foods, count } => (
            *count,
            Place::Food(s, i),
            format!("{count} × {}", foods_named(foods)),
        ),
        RecipeInput::Liquid { foods, ml } => (
            ml.div_ceil(ML_STEP),
            Place::Liquid(s, i),
            format!("{ml} ml of {}", foods_named(foods)),
        ),
        RecipeInput::Water { ml } => (
            ml.div_ceil(ML_STEP),
            Place::Liquid(s, i),
            format!("{ml} ml of water"),
        ),
    };
    let missing = !have.input(s, i);
    let stack = item.as_ref().map(|item| (item, count.max(1)));
    let shown = space_shown(stack, icons.of(place), missing.then_some(false));
    let tip = HoverTip(with_missing(words, missing));
    let id = UiId::new("recipe", "input").at(s * 100 + i);
    spaces.spawn((shown, tip, Solid, id));
}

/// A tip, saying so when what it names is not to hand.
fn with_missing(tip: String, missing: bool) -> String {
    match missing {
        true => format!("{tip} (not to hand)"),
        false => tip,
    }
}
