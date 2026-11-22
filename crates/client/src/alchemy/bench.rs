//! `Bench`: the process being built on the panel (its source, its steps and
//! what is in each step's drop spaces), how many runs, and whether it is
//! running.

use bevy::prelude::*;
use shared::alchemy::{Apparatus, Planned};
use shared::id::Id;
use shared::item::Item;

/// One step: an apparatus and the carried stacks put into its drop spaces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Station {
    pub(super) apparatus: Apparatus,
    /// A Mortar's herb stacks, one of each kind.
    pub(super) herbs: Vec<Id>,
    /// A Mortar's bottle of oil or pure alcohol, or none (dry: a Paste).
    pub(super) oil: Option<Id>,
    /// An Alembic's bottle of a drink or pure alcohol fed in, or none.
    pub(super) alcohol: Option<Id>,
    /// A Filter's Powder, or none.
    pub(super) powder: Option<Id>,
    /// A Retort's solid, or none.
    pub(super) solid: Option<Id>,
    /// A Retort's liquids, a 100 ml bottle each, in order (one stack may be
    /// listed again).
    pub(super) liquids: Vec<Id>,
}

impl Station {
    pub(super) fn new(apparatus: Apparatus) -> Self {
        Self {
            apparatus,
            herbs: Vec::new(),
            oil: None,
            alcohol: None,
            powder: None,
            solid: None,
            liquids: Vec::new(),
        }
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
    /// A carried Paste, Ointment or Powder to start from instead of a Mortar.
    pub(super) source: Option<Id>,
    pub(super) steps: Vec<Station>,
    /// How many times to run it, 1 or more.
    pub(super) count: u32,
    pub(super) running: Option<Running>,
    /// What the last run made, shown at the end of the last step until the
    /// process changes.
    pub(super) made: Option<Item>,
}

impl Default for Bench {
    fn default() -> Self {
        Self {
            document: None,
            source: None,
            steps: Vec::new(),
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

    /// Puts a new step of this apparatus at `at` (at most the end).
    pub(super) fn insert(&mut self, at: usize, apparatus: Apparatus) {
        let at = at.min(self.steps.len());
        self.steps.insert(at, Station::new(apparatus));
        self.made = None;
    }

    /// Moves the step at `from` to stand at `to` among the others (as counted
    /// before it was taken out).
    pub(super) fn shift(&mut self, from: usize, to: usize) {
        if from >= self.steps.len() {
            return;
        }
        let station = self.steps.remove(from);
        let to = if to > from { to - 1 } else { to };
        self.steps.insert(to.min(self.steps.len()), station);
        self.made = None;
    }

    /// Takes the step at `at` out, with what was put in it; the source goes
    /// with the last step, as it is shown among the first step's spaces.
    pub(super) fn remove(&mut self, at: usize) {
        if at < self.steps.len() {
            self.steps.remove(at);
            self.made = None;
        }
        if self.steps.is_empty() {
            self.source = None;
        }
    }

    /// Every stack named anywhere on it.
    pub(super) fn stacks(&self) -> impl Iterator<Item = Id> + '_ {
        let steps = self.steps.iter().flat_map(|station| {
            let herbs = station.herbs.iter().copied();
            herbs
                .chain(station.oil)
                .chain(station.alcohol)
                .chain(station.powder)
                .chain(station.solid)
                .chain(station.liquids.iter().copied())
        });
        self.source.into_iter().chain(steps)
    }

    /// Forgets stacks no longer carried (used up, dropped).
    pub(super) fn keep_carried(&mut self, carried: impl Fn(Id) -> bool) {
        if self.source.is_some_and(|id| !carried(id)) {
            self.source = None;
        }
        for station in &mut self.steps {
            station.herbs.retain(|id| carried(*id));
            station.liquids.retain(|id| carried(*id));
            if station.oil.is_some_and(|id| !carried(id)) {
                station.oil = None;
            }
            if station.alcohol.is_some_and(|id| !carried(id)) {
                station.alcohol = None;
            }
            if station.powder.is_some_and(|id| !carried(id)) {
                station.powder = None;
            }
            if station.solid.is_some_and(|id| !carried(id)) {
                station.solid = None;
            }
        }
    }
}
