//! A process as the player builds it: perhaps a stored source, then steps,
//! each an apparatus and what goes into it; and why one can be refused.

use super::Apparatus;
use crate::brewing::GroundHerb;
use crate::potion::{Contents, Liquid, Oil};

/// The most a Retort holds, in 100 ml units (500 ml).
pub const RETORT_UNITS: usize = 5;

/// What the process makes from, in order.
#[derive(Clone, Debug, Default)]
pub struct Process<'a> {
    /// A stored Paste, Ointment or Powder the first step works on, instead of
    /// starting with a Mortar.
    pub source: Option<Stored>,
    pub steps: Vec<Step<'a>>,
}

/// A step: an apparatus and what is put into it.
#[derive(Clone, Debug)]
pub enum Step<'a> {
    /// Herbs (one of each kind) and what they are ground with.
    Mortar {
        herbs: Vec<GroundHerb<'a>>,
        solvent: Solvent,
    },
    /// Liquids, one 100 ml unit each, applied in order to what came before:
    /// waters, oils, drinks and pure alcohol (the last two adding alcohol).
    /// The solid's place comes with Retort solids (PR 8).
    Retort {
        liquids: Vec<Liquid>,
    },
    /// Distils what came before (`alembic`), reduced by the alcohol fed in
    /// (a drink or pure alcohol, one 100 ml unit), if any.
    Alembic {
        fed: Option<Liquid>,
    },
    /// Calcines what came before into a Powder (`crucible`), at
    /// `POWDER_PERCENT` of its strength, its liquid driven off.
    Crucible,
    /// Filters what came before through a Powder's doses (`filter`): a stat
    /// in both adds together, one in only one is taken out.
    Filter {
        powder: Option<Contents>,
    },
    Pelican,
}

impl Step<'_> {
    /// The apparatus it uses.
    pub fn apparatus(&self) -> Apparatus {
        match self {
            Self::Mortar { .. } => Apparatus::Mortar,
            Self::Retort { .. } => Apparatus::Retort,
            Self::Alembic { .. } => Apparatus::Alembic,
            Self::Crucible => Apparatus::Crucible,
            Self::Filter { .. } => Apparatus::Filter,
            Self::Pelican => Apparatus::Pelican,
        }
    }
}

/// What herbs are ground with in the Mortar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Solvent {
    /// Nothing: a Paste.
    Dry,
    /// An oil: an Ointment, the oil's row boosted.
    Oil(Oil),
    /// Pure alcohol: a Tincture (no row boost, every dose +5%), a 100 ml
    /// liquid already.
    Alcohol,
}

/// A carried mixture used as a process's source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stored {
    pub form: Form,
    pub contents: Contents,
}

/// What a mixture is between steps or as a stored source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    Paste,
    Ointment,
    /// From the Crucible; never counts as old.
    Powder,
    /// Herbs ground in pure alcohol: a liquid, so a potion when last, never
    /// a stored source.
    Tincture,
}

impl Form {
    /// Whether a stored one counts as old (a quality step less): a Paste or an
    /// Ointment does, a Powder never.
    pub fn is_old(self) -> bool {
        !matches!(self, Self::Powder)
    }
}

/// Why a process can't be run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// No steps.
    Empty,
    /// A Mortar after something else (it starts a process, from herbs).
    MortarNotFirst,
    /// A Mortar with no herbs.
    NoHerbs,
    /// The same herb twice in one Mortar (only one of each counts).
    SameHerb,
    /// A Retort with no liquid.
    NoLiquid,
    /// A Retort holding more than `RETORT_UNITS`.
    TooMuchLiquid,
    /// A liquid a Retort doesn't take (juice, cordial, vinegar, potions).
    Liquid,
    /// An Alembic with no liquid before it to distil.
    NothingToDistil,
    /// What an Alembic is fed holds no alcohol.
    NotAlcohol,
    /// A Crucible with nothing before it to calcine.
    NothingToCalcine,
    /// A Filter with nothing before it to filter.
    NothingToFilter,
    /// A Filter with no Powder in it.
    NoPowder,
    /// A Pelican with nothing before it to circulate.
    NothingToCirculate,
    /// An apparatus or input whose rules come with a later PR.
    NotBuilt(Apparatus),
}
