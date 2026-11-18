//! `PotionKey`: a potion's contents in a fixed size, so an action bar slot can
//! hold a particular potion (brewing decision 22) and still be copied freely.

use serde::{Deserialize, Serialize};

use super::{Contents, Dose};
use crate::herb::Stat;

/// A potion's doses as one step per stat, in `Stat::ALL`'s order (0 for a
/// stat it doesn't move). Saved and sent as the `Contents` it stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "Contents", into = "Contents")]
pub struct PotionKey([i16; Stat::ALL.len()]);

impl PotionKey {
    /// The key of these contents.
    pub fn of(contents: &Contents) -> Self {
        let mut steps = [0; Stat::ALL.len()];
        for dose in contents.doses() {
            if let Some(place) = Stat::ALL.iter().position(|&stat| stat == dose.stat) {
                steps[place] = dose.step;
            }
        }
        Self(steps)
    }

    /// The contents it stands for.
    pub fn contents(self) -> Contents {
        let doses = Stat::ALL
            .iter()
            .zip(self.0)
            .map(|(&stat, step)| Dose { stat, step });
        Contents::new(doses)
    }

    /// Whether it holds no doses (not a potion).
    pub fn is_empty(self) -> bool {
        self.0.iter().all(|&step| step == 0)
    }
}

impl From<Contents> for PotionKey {
    fn from(contents: Contents) -> Self {
        Self::of(&contents)
    }
}

impl From<PotionKey> for Contents {
    fn from(key: PotionKey) -> Self {
        key.contents()
    }
}
