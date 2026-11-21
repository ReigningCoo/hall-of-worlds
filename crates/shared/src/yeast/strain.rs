//! Cultures as strains of yeast (cooking plan PR 6, owner, 2026-11-21): the
//! Yeast item also holds a yoghurt or koji culture, kept and sampled as yeast
//! is. A ferment forms wild without one, poorer and slower; a matching one
//! does best, any other helps (`food::culture`).

use serde::{Deserialize, Serialize};

/// Which culture a Yeast item holds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Strain {
    /// Bread and brewing yeast: what every yeast was before cultures.
    #[default]
    Bread,
    /// A yoghurt culture, taken from a yoghurt. Not fed: a new one is taken
    /// from each batch, as yoghurt is kept going.
    Yoghurt,
    /// Koji, the mould for soy sauce and miso; fed with flour as yeast is.
    Koji,
}

impl Strain {
    /// Every strain, in order.
    pub const ALL: [Self; 3] = [Self::Bread, Self::Yoghurt, Self::Koji];

    /// What a Yeast item holding it is called.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bread => "Yeast",
            Self::Yoghurt => "Yoghurt Culture",
            Self::Koji => "Koji",
        }
    }

    /// Whether it is fed with flour and water (`feed.rs`).
    pub const fn fed(self) -> bool {
        !matches!(self, Self::Yoghurt)
    }

    /// Whether it ferments drinks in a cask (`shared::ferment`).
    pub const fn brews(self) -> bool {
        matches!(self, Self::Bread)
    }
}
