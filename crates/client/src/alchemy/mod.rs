//! The Alchemy panel (Alchemy plan PR 4, `docs/context/alchemy.md`): opened
//! by the Alchemy ability, it builds a process from the apparatus whose
//! sub-skills the character knows, dragged down into a row of steps, each
//! with the drop spaces its apparatus takes; it shows what the process makes
//! at its least quality (`shared::alchemy::plan`, the server's own rule),
//! its chance, and the effects known so far; Start sends the order
//! (`ClientMessage::Alchemy`) and colour fills along the steps as each run
//! winds up. See `docs/context/client/alchemy.md`.

mod bench;
mod boxes;
#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod drag;
mod markup;
mod open;
mod order;
mod rows;
mod run;
mod side;
mod spaces;
mod words;

use bevy::prelude::*;

use crate::common::PointerSystems;
use crate::dom::{DomInput, HoldObjects};

/// Opens the Alchemy panel, or closes it if it is open (the Alchemy ability
/// used, by `abilities`).
#[derive(Message, Clone, Copy, Debug, Default)]
pub struct OpenAlchemy;

pub struct AlchemyPlugin;

impl Plugin for AlchemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<bench::Bench>()
            .init_resource::<drag::BenchDrag>()
            .init_resource::<drag::BenchAim>()
            .init_resource::<spaces::SpaceIcons>()
            .add_message::<OpenAlchemy>()
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
                    side::answer_panel,
                    run::follow_run,
                    rows::draw_rows,
                    side::show_side,
                    run::show_fill,
                )
                    .chain(),
            );
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
