//! The six apparatus a process is built from, and the Alchemy sub-skill each
//! one has (Alchemy plan, owner, 2026-11-19).

use serde::{Deserialize, Serialize};

/// Grinding: the Mortar & pestle's sub-skill.
pub const GRINDING: &str = "crafting/alchemy/grinding";
/// Infusing: the Retort's.
pub const INFUSING: &str = "crafting/alchemy/infusing";
/// Distilling: the Alembic's.
pub const DISTILLING: &str = "crafting/alchemy/distilling";
/// Calcining: the Crucible's.
pub const CALCINING: &str = "crafting/alchemy/calcining";
/// Filtering: the Filter's.
pub const FILTERING: &str = "crafting/alchemy/filtering";
/// Circulating: the Pelican's.
pub const CIRCULATING: &str = "crafting/alchemy/circulating";

/// One apparatus. The Mortar, the Retort and the Alembic have rules so far;
/// the others are refused as not built until their PRs (6 and 7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Apparatus {
    Mortar,
    Retort,
    Alembic,
    Crucible,
    Filter,
    Pelican,
    // New apparatus go at the end: messages carry one by its place in this list.
}

impl Apparatus {
    pub const ALL: [Apparatus; 6] = [
        Self::Mortar,
        Self::Retort,
        Self::Alembic,
        Self::Crucible,
        Self::Filter,
        Self::Pelican,
    ];

    /// "Mortar & Pestle", "Retort", ...
    pub fn name(self) -> &'static str {
        match self {
            Self::Mortar => "Mortar & Pestle",
            Self::Retort => "Retort",
            Self::Alembic => "Alembic",
            Self::Crucible => "Crucible",
            Self::Filter => "Filter",
            Self::Pelican => "Pelican",
        }
    }

    /// Its sub-skill's path in the skill tree, under Alchemy: known the first
    /// time the apparatus is picked up, practised by using it.
    pub fn sub_skill(self) -> &'static str {
        match self {
            Self::Mortar => GRINDING,
            Self::Retort => INFUSING,
            Self::Alembic => DISTILLING,
            Self::Crucible => CALCINING,
            Self::Filter => FILTERING,
            Self::Pelican => CIRCULATING,
        }
    }

    /// Its place in `ALL`.
    pub fn index(self) -> usize {
        self as usize
    }
}
