//! The combination table: parts mixed in one step that make something else
//! (fire + water = steam, the first row; the owner fills in the rest in the
//! effects PRs). A mixture not in the table does each part.

use serde::{Deserialize, Serialize};

use super::design::EffectStep;
use super::effect::Effect;
use super::part::{Part, parts};

/// Something only a mixture makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mixture {
    /// Fire and water: scalding steam.
    Steam,
}

impl Mixture {
    /// Its key, as in language files.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Steam => "steam",
        }
    }
}

/// One row: these two parts together make `makes`.
#[derive(Clone, Copy, Debug)]
pub struct Combination {
    pub of: [Part; 2],
    pub makes: Mixture,
}

/// Every combination, tried in this order.
pub const COMBINATIONS: &[Combination] = &[Combination {
    of: [Part::plain(Effect::Fire), Part::plain(Effect::Water)],
    makes: Mixture::Steam,
}];

/// What a step does in the end: a part, or a mixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Outcome {
    Part(Part),
    Mixture(Mixture),
}

impl Outcome {
    /// Its key, as in language files.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Part(part) => part.key(),
            Self::Mixture(mixture) => mixture.key(),
        }
    }
}

/// What a step does, with how many times: each row of `COMBINATIONS` takes as
/// many of its pair as both have (Fire ×3 with Water ×1 is Steam ×1 and Fire
/// ×2), mixtures first, then what is left in the order written.
pub fn outcomes(step: &EffectStep) -> Vec<(Outcome, u32)> {
    let mut left = parts(step);
    let mut outcomes = Vec::new();
    for row in COMBINATIONS {
        let count = |part: Part, left: &[(Part, u32)]| {
            left.iter()
                .find(|(seen, _)| *seen == part)
                .map_or(0, |(_, n)| *n)
        };
        let both = count(row.of[0], &left).min(count(row.of[1], &left));
        if both == 0 {
            continue;
        }
        for (part, n) in &mut left {
            if row.of.contains(part) {
                *n -= both;
            }
        }
        outcomes.push((Outcome::Mixture(row.makes), both));
    }
    outcomes.extend(
        left.into_iter()
            .filter(|(_, n)| *n > 0)
            .map(|(part, n)| (Outcome::Part(part), n)),
    );
    outcomes
}
