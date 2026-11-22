//! The Cooking panel (cooking plan PR 10, `docs/context/cooking.md`): opened
//! by the Cook ability, it builds a process from the kitchen tools, dragged
//! down into a grid of two rows (a step clicked joins both rows), each step
//! with drop spaces for foods, water, milk and honey, each row with a space
//! for an intermediate to start from; it shows what the process makes at its
//! least quality (`Inventory::check_cooking`, the server's own rule) at the
//! kitchen the player is at, with its name, chance, diet labels and Well
//! Fed; Start sends the order (`ClientMessage::Cook`) and colour fills along
//! the steps as each run winds up. Its Recipes tab lists the recipe book,
//! with Read, Make (the recipe laid on the bench, what is missing as red
//! ghosts) and Remove. See `docs/context/client/cooking.md`.

mod bench;
mod beyond;
mod boxes;
#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod drag;
mod ghost_space;
mod ghosts;
mod grid;
mod grid_nodes;
mod kitchen;
mod markup;
mod open;
mod order;
mod reading;
mod recipes;
mod run;
mod side;
mod spaces;
mod words;

use bevy::prelude::*;

use crate::common::PointerSystems;
use crate::dom::{DomInput, HoldObjects};

/// Opens the Cooking panel, or closes it if it is open (the Cook ability
/// used, by `abilities`).
#[derive(Message, Clone, Copy, Debug, Default)]
pub struct OpenCooking;

/// Opens the recipe reading panel on this recipe from the book (or shows it
/// there if the panel is open): the Recipes tab's Read (recipes plan PR 5).
/// Reading a scroll into the book opens it by itself.
#[derive(Message, Clone, Debug)]
pub struct ReadRecipe(pub shared::recipe::Entry);

pub struct CookingPlugin;

impl Plugin for CookingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<bench::Bench>()
            .init_resource::<drag::BenchDrag>()
            .init_resource::<drag::BenchAim>()
            .init_resource::<kitchen::Kitchens>()
            .init_resource::<spaces::SpaceIcons>()
            .init_resource::<recipes::Recipes>()
            .add_message::<OpenCooking>()
            .add_message::<ReadRecipe>()
            .add_systems(
                PreUpdate,
                (
                    drag::press_bench
                        .after(PointerSystems)
                        .after(DomInput::Find)
                        .before(DomInput::Presses)
                        .before(HoldObjects),
                    drag::drop_on_bench.after(HoldObjects),
                ),
            )
            .add_systems(
                Update,
                (
                    open::open_panel,
                    open::bind_panel,
                    open::notice_closed,
                    open::tell_open,
                    kitchen::keep_kitchen,
                    side::answer_panel,
                    recipes::answer_recipes,
                    run::follow_run,
                    grid::draw_grid,
                    side::show_side,
                    recipes::show_recipes,
                    run::show_fill,
                )
                    .chain(),
            );
        reading::add(app);
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
