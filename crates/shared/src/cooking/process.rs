//! What a cooking process is made of: a grid of rows, tool steps placed in
//! it (a tall step joining rows), what goes into each step, intermediates
//! starting rows, and where it is worked.

use serde::{Deserialize, Serialize};

use super::{Source, Tool};
use crate::item::Quality;

/// The rows a process has for now (cooking plan: 2 to start, more later).
pub const MOST_ROWS: u8 = 2;
/// The columns a row may have steps in.
pub const MOST_COLUMNS: u8 = 12;

/// Something going into a step.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Portion {
    /// A raw food, by its culinary data's source and key
    /// (`Ingredients::get`), its weight, and its quality if it has one (none
    /// counts as Common).
    Food {
        source: Source,
        key: String,
        grams: u32,
        quality: Option<Quality>,
    },
    /// Water from any source (bottles, vessels, casks, water faced), by the
    /// millilitre. It has no role and no quality.
    Water { ml: u32 },
}

impl Portion {
    /// Whether it is a liquid a Cauldron or Sieve can work: water or milk.
    pub fn is_liquid(&self) -> bool {
        match self {
            Self::Water { .. } => true,
            Self::Food { source, .. } => *source == Source::Milk,
        }
    }
}

/// The steps an intermediate went through before, and the cooks' sub-skills
/// at them: they count towards the quality of what is made from it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    /// Its steps in halves: 2 a step, 3 a join (a step and a half).
    pub halves: u32,
    /// The ∛ of each tool's sub-skill used (`CookingSkills::sub_root`), each
    /// tool once, the lowest kept when two cooks used it.
    pub roots: Vec<(Tool, u32)>,
    /// Its tools in the order they were used (a source's first), for naming
    /// what it becomes (`classify`).
    #[serde(default)]
    pub tools: Vec<Tool>,
}

impl History {
    /// Adds `other`'s steps, keeping the lower root of a tool in both.
    pub fn merge(&mut self, other: &History) {
        self.halves += other.halves;
        self.tools.extend(other.tools.iter().copied());
        for &(tool, root) in &other.roots {
            self.add_root(tool, root);
        }
    }

    /// How many of its steps were joins: each counts a half more than a step.
    pub fn joins(&self) -> u32 {
        self.halves.saturating_sub(2 * self.tools.len() as u32)
    }

    /// Whether any of its steps used heat (`Tool::needs_heat`): cooked, not
    /// raw.
    pub fn cooked(&self) -> bool {
        self.tools.iter().any(|tool| tool.needs_heat())
    }

    /// Notes `tool` used at `root`, keeping the lower if already there.
    pub fn add_root(&mut self, tool: Tool, root: u32) {
        match self.roots.iter_mut().find(|(t, _)| *t == tool) {
            Some((_, kept)) => *kept = (*kept).min(root),
            None => self.roots.push((tool, root)),
        }
        self.roots.sort_unstable();
    }
}

/// An intermediate (dough, stock, butter, ...) in a row's source space: what
/// it holds and what was done to it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Carried {
    pub portions: Vec<Portion>,
    pub history: History,
}

/// One tool step in the grid: its column, the top row it covers and how many
/// rows (more than one makes it a join, taking every row's output), and what
/// is put into it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub tool: Tool,
    pub column: u8,
    pub row: u8,
    pub height: u8,
    pub portions: Vec<Portion>,
}

impl Step {
    /// A one-row step.
    pub fn at(tool: Tool, column: u8, row: u8, portions: Vec<Portion>) -> Self {
        Self {
            tool,
            column,
            row,
            height: 1,
            portions,
        }
    }

    /// Whether it joins rows.
    pub fn is_join(&self) -> bool {
        self.height > 1
    }

    /// Whether it covers `row`.
    pub fn covers(&self, row: u8) -> bool {
        (self.row..self.row.saturating_add(self.height)).contains(&row)
    }

    /// Its weight in halves of a step: 2, or 3 for a join.
    pub fn halves(&self) -> u32 {
        if self.is_join() { 3 } else { 2 }
    }
}

/// Where a process is worked, for heat and the quality cap.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Heat {
    /// No fire: only steps without heat.
    #[default]
    Cold,
    /// A campfire (or other makeshift heat): every step, capped at Rare.
    Fire,
    /// A proper hearth (towns' too): every step, no cap.
    Hearth,
}

/// A whole process.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Process {
    /// Rows in the grid, 1 to `MOST_ROWS`.
    pub rows: u8,
    /// Each row's source space, by row (shorter than `rows` is fine).
    pub sources: Vec<Option<Carried>>,
    pub steps: Vec<Step>,
    pub heat: Heat,
}

/// Why a process can't be made. `step` is an index into `Process::steps`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refused {
    /// No rows, or more than `MOST_ROWS`; or a source past the last row.
    Rows,
    NoSteps,
    /// Past the grid's rows or columns, or no height.
    OutOfGrid {
        step: usize,
    },
    /// Covers a place another step covers.
    Overlap {
        step: usize,
    },
    /// Two steps both take its output.
    Split {
        step: usize,
    },
    /// Nothing goes into it.
    NothingIn {
        step: usize,
    },
    /// A join with a row holding nothing before it.
    JoinMissing {
        step: usize,
    },
    /// A Cauldron or Sieve with no water or milk in it.
    NoLiquid {
        step: usize,
    },
    /// A heat step with no heat.
    NoHeat {
        step: usize,
    },
    /// A food with no culinary data.
    UnknownIngredient {
        step: usize,
    },
    /// A source holding a food with no culinary data.
    UnknownInSource {
        row: u8,
    },
    /// A source no step takes.
    UnusedSource {
        row: u8,
    },
}
