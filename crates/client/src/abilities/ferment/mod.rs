//! Ferment's panel (drinks plan PR 6): using Ferment opens a dialog to choose
//! a cask (carried, or lying in reach) holding juice, cordial or water; for
//! water, the fruit or grain carried to mash into it (all of the stack); and
//! the yeast stack and how many of its usable yeasts to pitch (more starts a
//! big cask faster). It says what it would make (`ferment::plan`, the
//! server's own rule). Ferment asks the server (`ClientMessage::Ferment`),
//! which winds up and seals the cask fermenting.

mod answer;
mod markup;
mod show;

use bevy::prelude::*;
use shared::ferment::{FermentOrder, YeastStack};
use shared::item::Item;

pub(super) use answer::answer_panel;
#[cfg(test)]
pub(super) use markup::FERMENT;
pub(super) use markup::spawn_panel;
pub(super) use show::show_panel;

/// Whether the panel is up, what it offers in its dropdowns' order, and what
/// is chosen.
#[derive(Resource, Default)]
pub(super) struct Fermenting {
    pub(super) open: bool,
    /// Casks that could be set going, carried first, then lying in reach.
    casks: Vec<Item>,
    cask: usize,
    /// Fruit and grain stacks carried; the dropdown's first option is none.
    adds: Vec<Item>,
    /// The option chosen: 0 for nothing added, else `adds[add - 1]`.
    add: usize,
    yeasts: Vec<YeastStack>,
    yeast: usize,
    /// How many yeasts, 1 or more.
    count: u32,
    /// Set when what is chosen changes, so what it makes is shown again.
    dirty: bool,
}

impl Fermenting {
    /// The fruit or grain stack chosen, if any.
    fn added(&self) -> Option<&Item> {
        self.add.checked_sub(1).and_then(|at| self.adds.get(at))
    }

    /// The order for what is chosen, if a cask and a yeast are there.
    fn order(&self) -> Option<FermentOrder> {
        Some(FermentOrder {
            cask: self.casks.get(self.cask)?.id,
            add: self.added().map(|add| add.id),
            yeast: self.yeasts.get(self.yeast)?.stack,
            yeasts: self.count.max(1),
        })
    }
}
