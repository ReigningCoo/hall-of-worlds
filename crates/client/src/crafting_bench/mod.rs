//! Crafting benches (crafting benches plan PR 3, `shared::crafting_bench`):
//! the tools the server tells of each bench on the level, the bench the
//! player is at, and a bench's panel: clicking (or tapping) one walks up to
//! it, turns to it and opens a grid of its tools like a shop's stock, where
//! carried tools are dropped on and picked off. The models are
//! `dungeon_map`'s. See `docs/context/client/crafting_bench.md`.

use bevy::prelude::*;

use crate::common::PointerSystems;
use crate::dom::DomInput;
use crate::player::PlayerSystems;

// Split into files (`cargo xtask lint` keeps files small).
mod close;
#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod grid;
mod here;
mod markup;
mod open;
mod server;
mod state;

use state::{BenchHeading, BenchPanel};

/// To the right of where the Character panel opens, so tools can be dragged
/// between them, leaving the player in view.
const AT: Vec2 = Vec2::new(680.0, 120.0);
/// How many cells across a bench's grid is.
const COLUMNS: u8 = 6;

pub struct CraftingBenchPlugin;

impl Plugin for CraftingBenchPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BenchPanel>()
            .init_resource::<BenchHeading>()
            .add_systems(
                PreUpdate,
                close::close_on_escape
                    .after(PointerSystems)
                    .before(DomInput::Front),
            )
            // After the steps there have gone, so the server has the player at it.
            .add_systems(PostUpdate, open::open_on_arrival)
            .add_systems(
                Update,
                (
                    (open::change_of_mind, open::follow_clicked_bench)
                        .chain()
                        .in_set(PlayerSystems::Input),
                    here::keep_bench_here.after(PlayerSystems::Travel),
                    (
                        close::close_when_left,
                        grid::note_gifts,
                        server::read_server,
                        grid::bind_grid,
                        grid::draw_grid,
                        grid::show_gift_aim,
                    )
                        .chain()
                        .after(PlayerSystems::Travel),
                ),
            );
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
