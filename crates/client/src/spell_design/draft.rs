//! `Draft`: the spell being designed on the panel (its steps, and the
//! player's own name kept from a spell taken out of the book to change), and
//! every change the panel makes to it, one where or effect at a time
//! (`units.rs`). Kept when the panel closes.

use bevy::prelude::*;
use shared::spell::{BookSpell, Filters, MAX_STEPS, Spell, Step};

use super::piece::FilterKind;
use super::units::{Unit, click_joins, filters_of, step_of, units, with};

/// The design on the panel.
#[derive(Resource, Debug, Default)]
pub(super) struct Draft {
    /// The panel's document while it is open.
    pub(super) document: Option<Entity>,
    /// The steps, their wheres never holding filters: those are `filters`.
    pub(super) steps: Vec<Step>,
    /// The filters, always the first step's (owner, 2026-11-23): put on its
    /// where when it has one (`spell`).
    pub(super) filters: Filters,
    /// The player's own name of a spell taken out of the book, offered again
    /// on saving.
    pub(super) name: Option<String>,
    /// Whether the Save requester is up, and the name typed in it.
    pub(super) naming: bool,
    pub(super) typed: String,
}

impl Draft {
    pub(super) fn is_open(&self) -> bool {
        self.document.is_some()
    }

    /// The design as a spell: the filters on the first step's where.
    pub(super) fn spell(&self) -> Spell {
        let mut steps = self.steps.clone();
        if let Some(Step::Where(first)) = steps.first_mut() {
            first.filters = self.filters;
        }
        Spell::new(steps)
    }

    /// Whether filters are set with no where first for them to filter.
    pub(super) fn filters_stranded(&self) -> bool {
        self.filters.any() && !matches!(self.steps.first(), Some(Step::Where(_)))
    }

    /// Puts steps on the panel, the filters taken off their wheres.
    pub(super) fn set_steps(&mut self, mut steps: Vec<Step>) {
        self.filters = match steps.first() {
            Some(Step::Where(first)) => first.filters,
            _ => Filters::NONE,
        };
        for step in &mut steps {
            if let Step::Where(step) = step {
                step.filters = Filters::NONE;
            }
        }
        self.steps = steps;
    }

    /// Puts a spell from the book on the panel, in place of what was there.
    pub(super) fn load(&mut self, kept: BookSpell) {
        self.set_steps(kept.spell.steps);
        self.name = kept.name;
        self.naming = false;
    }

    /// Empties the panel.
    pub(super) fn clear(&mut self) {
        self.steps.clear();
        self.filters = Filters::NONE;
        self.name = None;
        self.naming = false;
    }

    /// A new step at `at` (at most the end) holding one where or effect.
    pub(super) fn add_step(&mut self, at: usize, unit: Unit) {
        let Some(step) = step_of(&[unit], Filters::NONE) else {
            return;
        };
        if self.steps.len() < MAX_STEPS {
            self.steps.insert(at.min(self.steps.len()), step);
        }
    }

    /// Whether the step at `at` takes one more of this.
    pub(super) fn takes(&self, at: usize, unit: Unit) -> bool {
        self.steps
            .get(at)
            .and_then(|step| with(step, unit))
            .is_some()
    }

    /// One more where or effect at the end of the step at `at`, if it takes
    /// it (any where in a where step, merged after the rest; any effect in an
    /// effect step).
    pub(super) fn add_to(&mut self, at: usize, unit: Unit) {
        if let Some(step) = self.steps.get(at).and_then(|step| with(step, unit)) {
            self.steps[at] = step;
        }
    }

    /// A where or effect clicked: added to the last step if it takes it (the
    /// same where again, an effect to effects), else in a new step at the end.
    pub(super) fn append(&mut self, unit: Unit) {
        let joins = |last: usize| click_joins(&self.steps[last], unit) && self.takes(last, unit);
        match self.steps.len().checked_sub(1) {
            Some(last) if joins(last) => self.add_to(last, unit),
            _ => self.add_step(self.steps.len(), unit),
        }
    }

    /// Puts a filter on the first step, which must be a where for it to
    /// filter anything; or takes it off.
    pub(super) fn set_filter(&mut self, filter: FilterKind, on: bool) {
        if on && !matches!(self.steps.first(), Some(Step::Where(_))) {
            return;
        }
        filter.set(&mut self.filters, on);
    }

    /// Whether this filter is on.
    pub(super) fn has_filter(&self, filter: FilterKind) -> bool {
        filter.of(self.filters)
    }

    /// Moves the step at `from` to stand at `to` among the others (as counted
    /// before it was taken out).
    pub(super) fn shift(&mut self, from: usize, to: usize) {
        if from >= self.steps.len() {
            return;
        }
        let step = self.steps.remove(from);
        let to = if to > from { to - 1 } else { to };
        self.steps.insert(to.min(self.steps.len()), step);
    }

    /// Takes the step at `at` out.
    pub(super) fn remove(&mut self, at: usize) {
        if at < self.steps.len() {
            self.steps.remove(at);
        }
        self.forget_filters_when_empty();
    }

    /// Filters go with the last step.
    fn forget_filters_when_empty(&mut self) {
        if self.steps.is_empty() {
            self.filters = Filters::NONE;
        }
    }

    /// Takes the `n`th where or effect out of a step, and the step once
    /// empty; returns it.
    pub(super) fn remove_unit(&mut self, at: usize, n: usize) -> Option<Unit> {
        let step = self.steps.get(at)?;
        let mut left = units(step);
        if n >= left.len() {
            return None;
        }
        let taken = left.remove(n);
        match step_of(&left, filters_of(step)) {
            Some(fewer) => self.steps[at] = fewer,
            None => {
                self.steps.remove(at);
                self.forget_filters_when_empty();
            }
        }
        Some(taken)
    }

    /// Moves one where or effect out of its step into another step that takes
    /// it (`Goes::Step`) or a new step at a place in the row (`Goes::Gap`).
    /// Places are counted before it was taken out; put back if it doesn't fit.
    pub(super) fn move_unit(&mut self, (step, n): (usize, usize), into: Goes) {
        let before = self.steps.len();
        let Some(unit) = self.remove_unit(step, n) else {
            return;
        };
        // Its step went with it: places after it are one fewer.
        let gone = self.steps.len() < before;
        let after = |place: usize| {
            if gone && place > step {
                place - 1
            } else {
                place
            }
        };
        match into {
            Goes::Step(to) if gone && to == step => self.add_step(step, unit),
            Goes::Step(to) if self.takes(after(to), unit) => self.add_to(after(to), unit),
            Goes::Step(_) if gone => self.add_step(step, unit),
            Goes::Step(_) => self.add_to(step, unit),
            Goes::Gap(to) => self.add_step(after(to), unit),
        }
    }
}

/// Where a moved where or effect goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Goes {
    /// Added to the end of this step.
    Step(usize),
    /// A new step at this place in the row.
    Gap(usize),
}
