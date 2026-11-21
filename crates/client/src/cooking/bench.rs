//! `Bench`: the process being built on the Cooking panel: each row's source
//! (an intermediate to start from), the tool steps placed in the grid (a tall
//! one joining both rows) with what is put into each, a culture, how many
//! runs, and whether it is running.

use bevy::prelude::*;
use shared::cooking::{MOST_COLUMNS, MOST_ROWS, Planned, Tool};
use shared::id::Id;
use shared::item::Item;

/// The rows the panel shows: as many as the rules allow.
pub(super) const ROWS: usize = MOST_ROWS as usize;

/// One step: its tool, its place in the grid, and what is put into it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Station {
    pub(super) tool: Tool,
    pub(super) column: u8,
    /// Its top row.
    pub(super) row: u8,
    /// 1, or 2 when it joins both rows.
    pub(super) height: u8,
    /// Carried stacks of raw food, each with how many of it go in.
    pub(super) foods: Vec<(Id, u32)>,
    /// Carried bottles, vessels or casks of water, milk or honey, each with
    /// how many millilitres go in (bottles in whole 100 ml).
    pub(super) liquids: Vec<(Id, u32)>,
    /// Millilitres of the water faced.
    pub(super) faced: u32,
}

impl Station {
    pub(super) fn new(tool: Tool, column: u8, row: u8) -> Self {
        Self {
            tool,
            column,
            row,
            height: 1,
            foods: Vec::new(),
            liquids: Vec::new(),
            faced: 0,
        }
    }

    /// Whether it covers the grid's place `(column, row)`.
    pub(super) fn covers(&self, column: u8, row: u8) -> bool {
        self.column == column && (self.row..self.row + self.height).contains(&row)
    }
}

/// A run under way: when it was asked for (by `Time`, to give up if the
/// server never starts it), whether the server has started a run, and what
/// one run makes, as worked out on Start.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Running {
    pub(super) asked: f64,
    pub(super) started: bool,
    pub(super) planned: Option<Planned>,
}

/// The process on the panel.
#[derive(Resource, Debug)]
pub(super) struct Bench {
    /// The panel's document while it is open.
    pub(super) document: Option<Entity>,
    /// Each row's intermediate to start from, carried or in the hearth's
    /// storage.
    pub(super) sources: [Option<Id>; ROWS],
    pub(super) steps: Vec<Station>,
    /// A carried culture (a Yeast item), used where an output ferments.
    pub(super) culture: Option<Id>,
    /// How many times to run it, 1 or more.
    pub(super) count: u32,
    pub(super) running: Option<Running>,
    /// What the last run made, shown in the product's frame until the
    /// process changes.
    pub(super) made: Option<Item>,
}

impl Default for Bench {
    fn default() -> Self {
        Self {
            document: None,
            sources: [None; ROWS],
            steps: Vec::new(),
            culture: None,
            count: 1,
            running: None,
            made: None,
        }
    }
}

impl Bench {
    pub(super) fn is_open(&self) -> bool {
        self.document.is_some()
    }

    pub(super) fn is_running(&self) -> bool {
        self.running.is_some()
    }

    /// The step covering `(column, row)`, if any.
    pub(super) fn step_at(&self, column: u8, row: u8) -> Option<usize> {
        self.steps.iter().position(|s| s.covers(column, row))
    }

    /// Whether a step `height` tall can stand at `(column, row)`: inside the
    /// grid and over no other step than `except`.
    pub(super) fn free(&self, column: u8, row: u8, height: u8, except: Option<usize>) -> bool {
        if column >= MOST_COLUMNS || height == 0 || usize::from(row + height) > ROWS {
            return false;
        }
        (row..row + height).all(|r| {
            let at = self.step_at(column, r);
            at.is_none() || at == except
        })
    }

    /// The first column after every step in `row`.
    pub(super) fn row_end(&self, row: u8) -> u8 {
        let covering = self.steps.iter().filter(|s| s.covers(s.column, row));
        covering.map(|s| s.column + 1).max().unwrap_or(0)
    }

    /// The columns to show: one past the last used, at least 3.
    pub(super) fn columns_shown(&self) -> u8 {
        let used = self.steps.iter().map(|s| s.column + 1).max().unwrap_or(0);
        (used + 1).clamp(3, MOST_COLUMNS)
    }

    /// Adds a step of `tool` at the end of the top row (a click or tap).
    pub(super) fn add(&mut self, tool: Tool) -> bool {
        let column = self.row_end(0);
        self.place(tool, column, 0)
    }

    /// Puts a new step of `tool` at `(column, row)` if it is free.
    pub(super) fn place(&mut self, tool: Tool, column: u8, row: u8) -> bool {
        if !self.free(column, row, 1, None) {
            return false;
        }
        self.steps.push(Station::new(tool, column, row));
        self.made = None;
        true
    }

    /// Moves step `at` (with what is in it) to stand at `(column, row)` if
    /// there is room for it there; a join keeps both rows.
    pub(super) fn move_to(&mut self, at: usize, column: u8, row: u8) -> bool {
        let Some(height) = self.steps.get(at).map(|s| s.height) else {
            return false;
        };
        let row = if height > 1 { 0 } else { row };
        if !self.free(column, row, height, Some(at)) {
            return false;
        }
        let step = &mut self.steps[at];
        (step.column, step.row) = (column, row);
        self.made = None;
        true
    }

    /// Takes step `at` out, with what was put in it.
    pub(super) fn remove(&mut self, at: usize) {
        if at < self.steps.len() {
            self.steps.remove(at);
            self.made = None;
        }
    }

    /// Makes step `at` join both rows if the place above or below it is
    /// free, or a join one row again (keeping its top row).
    pub(super) fn toggle_join(&mut self, at: usize) -> bool {
        let Some(step) = self.steps.get(at) else {
            return false;
        };
        let (column, row) = (step.column, step.row);
        let (new_row, height) = if step.height > 1 {
            (row, 1)
        } else if self.free(column, row, 2, Some(at)) {
            (row, 2)
        } else if row > 0 && self.free(column, row - 1, 2, Some(at)) {
            (row - 1, 2)
        } else {
            return false;
        };
        let step = &mut self.steps[at];
        (step.row, step.height) = (new_row, height);
        self.made = None;
        true
    }

    /// Every stack named anywhere on it, with what one run uses of it:
    /// `(id, items, millilitres)`.
    pub(super) fn uses(&self) -> Vec<(Id, u32, u32)> {
        let mut uses: Vec<(Id, u32, u32)> = Vec::new();
        let mut add = |id: Id, count: u32, ml: u32| match uses.iter_mut().find(|u| u.0 == id) {
            Some(found) => (found.1, found.2) = (found.1 + count, found.2 + ml),
            None => uses.push((id, count, ml)),
        };
        for id in self.sources.iter().flatten() {
            add(*id, 1, 0);
        }
        for step in &self.steps {
            for &(id, count) in &step.foods {
                add(id, count, 0);
            }
            for &(id, ml) in &step.liquids {
                add(id, 0, ml);
            }
        }
        uses
    }

    /// Forgets stacks no longer there (used up, dropped, taken off the
    /// hearth): `there` says which still are.
    pub(super) fn keep_there(&mut self, there: impl Fn(Id) -> bool) {
        for source in &mut self.sources {
            if source.is_some_and(|id| !there(id)) {
                *source = None;
            }
        }
        if self.culture.is_some_and(|id| !there(id)) {
            self.culture = None;
        }
        for step in &mut self.steps {
            step.foods.retain(|(id, _)| there(*id));
            step.liquids.retain(|(id, _)| there(*id));
        }
    }
}
