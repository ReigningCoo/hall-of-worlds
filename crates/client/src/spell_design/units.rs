//! A step as the panel shows it: no counts, each where or effect as often as
//! it was added (owner, 2026-11-23). `shared::spell` keeps them counted (a
//! where step's `count`, an effect step's runs of the same effect), which the
//! rules read the same way, so a step is turned into units and back.

use shared::spell::{Effect, EffectStep, Filters, Step, Where, WhereStep};

use super::piece::Piece;

/// The most of one where in a step, or of one effect in a row in a step.
pub(super) const MOST_REPEATS: usize = 9;

/// One where or one effect, as added.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Unit {
    Where(Where),
    Effect(Effect),
}

impl Unit {
    /// The piece it is drawn as.
    pub(super) const fn piece(self) -> Piece {
        match self {
            Self::Where(place) => Piece::Where(place),
            Self::Effect(effect) => Piece::Effect(effect),
        }
    }

    /// The unit a piece adds, if it isn't a filter.
    pub(super) const fn of(piece: Piece) -> Option<Self> {
        match piece {
            Piece::Where(place) => Some(Self::Where(place)),
            Piece::Effect(effect) => Some(Self::Effect(effect)),
            Piece::Filter(_) => None,
        }
    }
}

/// A step's units in order.
pub(super) fn units(step: &Step) -> Vec<Unit> {
    match step {
        Step::Where(step) => vec![Unit::Where(step.place); usize::from(step.count)],
        Step::Effects(step) => (step.effects.iter())
            .flat_map(|&(effect, count)| vec![Unit::Effect(effect); usize::from(count)])
            .collect(),
    }
}

/// The step of these units (with these filters, for a where step): none when
/// empty, mixing wheres and effects or two kinds of where, or too many of one.
pub(super) fn step_of(units: &[Unit], filters: Filters) -> Option<Step> {
    let first = *units.first()?;
    let step = match first {
        Unit::Where(place) => {
            if units.iter().any(|unit| *unit != first) || units.len() > MOST_REPEATS {
                return None;
            }
            Step::Where(WhereStep {
                place,
                count: units.len() as u8,
                filters,
            })
        }
        Unit::Effect(_) => {
            let mut effects: Vec<(Effect, u8)> = Vec::new();
            for unit in units {
                let Unit::Effect(effect) = *unit else {
                    return None;
                };
                match effects.last_mut() {
                    Some((last, count)) if *last == effect => *count += 1,
                    _ => effects.push((effect, 1)),
                }
            }
            if effects
                .iter()
                .any(|(_, count)| usize::from(*count) > MOST_REPEATS)
            {
                return None;
            }
            Step::Effects(EffectStep { effects })
        }
    };
    Some(step)
}

/// The step's filters (none for an effect step).
pub(super) fn filters_of(step: &Step) -> Filters {
    match step {
        Step::Where(step) => step.filters,
        Step::Effects(_) => Filters::NONE,
    }
}

/// The step with one more unit at its end, if it takes it: the same where
/// again, or any effect in an effect step.
pub(super) fn with(step: &Step, unit: Unit) -> Option<Step> {
    let mut more = units(step);
    more.push(unit);
    step_of(&more, filters_of(step))
}
