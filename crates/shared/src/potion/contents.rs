//! What a bottle holds: `Contents`, a list of `Dose`s kept in one order so
//! equal contents compare equal.

use serde::{Deserialize, Serialize};

use crate::herb::Stat;

/// One effect in a potion: a stat and how far it moves it, up (helpful) or
/// down (harmful), in whole steps. Never 0 inside `Contents`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dose {
    pub stat: Stat,
    pub step: i16,
}

/// What a bottle holds: nothing (an empty bottle), or doses of different
/// stats in `Stat`'s order, none of them 0. Made only through `new` (and
/// read back the same way), so the same effects are always the same contents.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "Vec<Dose>", into = "Vec<Dose>")]
pub struct Contents(Vec<Dose>);

impl Contents {
    /// An empty bottle's.
    pub const EMPTY: Self = Self(Vec::new());

    /// These doses in the normal order: those of one stat added together,
    /// any that come to 0 left out.
    pub fn new(doses: impl IntoIterator<Item = Dose>) -> Self {
        let mut kept: Vec<Dose> = Vec::new();
        for dose in doses {
            match kept.iter_mut().find(|it| it.stat == dose.stat) {
                Some(it) => it.step = it.step.saturating_add(dose.step),
                None => kept.push(dose),
            }
        }
        kept.retain(|dose| dose.step != 0);
        kept.sort_by_key(|dose| dose.stat);
        Self(kept)
    }

    /// Whether there is nothing in it.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Its doses, in `Stat`'s order.
    pub fn doses(&self) -> &[Dose] {
        &self.0
    }
}

impl From<Vec<Dose>> for Contents {
    fn from(doses: Vec<Dose>) -> Self {
        Self::new(doses)
    }
}

impl From<Contents> for Vec<Dose> {
    fn from(contents: Contents) -> Self {
        contents.0
    }
}
