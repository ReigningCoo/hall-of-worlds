//! Grind's mixing panel (brewing plan PR 6): using Grind opens a dialog
//! listing the herbs carried (tick one of each kind: only one of each counts),
//! the oils carried, how many doses (each takes one of every herb ticked), and
//! what it would make (`Inventory::check_grind`, the server's own rule), with
//! the mixes made before (`Sheet::recipes`) to pick again. Grind asks the
//! server (`ClientMessage::Grind`), which winds up and makes it.

mod answer;
mod markup;
mod show;
mod words;

use bevy::prelude::*;
use shared::brewing::{HerbStack, OilStack, Recipe};
use shared::id::Id;

pub(super) use answer::answer_mixing;
#[cfg(test)]
pub(super) use markup::GRIND;
pub(super) use markup::spawn_mixing;
pub(super) use show::show_mixing;

/// Whether the panel is up, what it lists (in its rows' order) and what is
/// chosen.
#[derive(Resource, Default)]
pub(super) struct Mixing {
    pub(super) open: bool,
    /// The herb stacks listed, a row each.
    herbs: Vec<HerbStack>,
    /// The herb stacks ticked, one per herb at most.
    chosen: Vec<Id>,
    /// The oil bottles listed, an option each.
    oils: Vec<OilStack>,
    /// The oil chosen, by its place in `oils`.
    oil: usize,
    /// Doses asked for, 1 or more.
    doses: u32,
    /// The mixes made before listed, a row each.
    recipes: Vec<Recipe>,
    /// Set when what is chosen changes, so the rest is shown again.
    dirty: bool,
}
