//! What each outcome does where it lands (magic plan PR 11), beyond Heal:
//! Fire burns, dries the wet and lights fireplaces; reversed, cold burns.
//! Water wets and puts fires out; reversed, ice chills and freezes water to
//! walk on. Steam (fire and water) scalds, puts fires out and leaves a cloud
//! that blocks sight (owner, 2026-11-23). Heal reversed harms. Numbers are
//! first guesses.

use crate::dungeon::{Cell, Level};

use super::combination::{Mixture, Outcome, outcomes};
use super::design::{Spell, Step};
use super::effect::Effect;
use super::harm::does_harm;

/// Seconds water stays frozen, each ice.
pub const ICE_SECONDS: u64 = 60;
/// Seconds a cloud of steam lasts, each.
pub const STEAM_SECONDS: u64 = 5;

/// What an outcome does to creatures and to the place it lands on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Acts {
    /// Health lost (`harm::harm_before_resisting`).
    pub harms: bool,
    /// Wets creatures (`condition::wet`).
    pub wets: bool,
    /// Dries the wet.
    pub dries: bool,
    /// Chills creatures (`condition::chill`), if it takes hold.
    pub chills: bool,
    /// Lights fireplaces with kindling in their basket.
    pub lights: bool,
    /// Puts out campfires and fireplaces' and hearths' fires.
    pub douses: bool,
    /// Freezes water round where it lands (`frozen_cells`).
    pub freezes: bool,
    /// Leaves a cloud of steam round where it lands (`cloud_cells`).
    pub clouds: bool,
}

impl Acts {
    /// Whether it does anything to the place itself, not only to creatures:
    /// a first step landing only on such a place doesn't fizzle.
    pub fn on_place(self) -> bool {
        self.lights || self.douses || self.freezes || self.clouds
    }
}

/// Whether the spell's first effect step does something to the place it
/// lands on: then a first step that hit nobody still lands on its spots
/// rather than fizzling (Touch → Fire at a fireplace, Missile → Water at a
/// campfire).
pub fn acts_on_place(spell: &Spell) -> bool {
    let first = spell.steps.iter().find_map(|step| match step {
        Step::Effects(effects) => Some(effects),
        Step::Where(_) => None,
    });
    first.is_some_and(|step| outcomes(step).into_iter().any(|(o, _)| acts(o).on_place()))
}

/// What `outcome` does.
pub fn acts(outcome: Outcome) -> Acts {
    let harms = does_harm(outcome);
    let none = Acts::default();
    let Outcome::Part(part) = outcome else {
        let Outcome::Mixture(Mixture::Steam) = outcome else {
            return none;
        };
        return Acts {
            harms,
            douses: true,
            clouds: true,
            ..none
        };
    };
    match (part.effect, part.reversed) {
        (Effect::Fire, false) => Acts {
            harms,
            dries: true,
            lights: true,
            ..none
        },
        (Effect::Water, false) => Acts {
            wets: true,
            douses: true,
            ..none
        },
        (Effect::Water, true) => Acts {
            chills: true,
            freezes: true,
            ..none
        },
        _ => Acts { harms, ..none },
    }
}

/// The cells `count` ices freeze round `(x, y)`: water and shallows within
/// `count` cells (a square), the middle too.
pub fn frozen_cells(level: &Level, (x, y): (i64, i64), count: u32) -> Vec<(i64, i64)> {
    round(level, (x, y), count, |cell| {
        matches!(cell, Cell::Water | Cell::Shallows)
    })
}

/// The cells a cloud of `count` steams covers round `(x, y)`: every cell
/// sight passes through within `count` cells.
pub fn cloud_cells(level: &Level, (x, y): (i64, i64), count: u32) -> Vec<(i64, i64)> {
    round(level, (x, y), count, Cell::is_open)
}

fn round(
    level: &Level,
    (x, y): (i64, i64),
    count: u32,
    wanted: impl Fn(Cell) -> bool,
) -> Vec<(i64, i64)> {
    let reach = i64::from(count.clamp(1, 5));
    let mut cells = Vec::new();
    for cy in y - reach..=y + reach {
        for cx in x - reach..=x + reach {
            if level.contains(cx, cy) && wanted(level.cell(cx, cy)) {
                cells.push((cx, cy));
            }
        }
    }
    cells
}
