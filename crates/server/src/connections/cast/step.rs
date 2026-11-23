//! What one where step of a spell hits (`shared::spell::where_hits`), from
//! the caster for the first, else from each thing the last where hit; and
//! whom an effect step lands on.

use shared::dungeon::Level;
use shared::facing::Heading;
use shared::spell::{Hit, Origin, Toward, WhereStep, where_hits};

use super::creatures::Present;
use super::{Aimed, Cast};

/// What `step` hits, as the next steps' targets (the caster facing
/// `heading`). Only the first where goes at the caster's target.
pub(super) fn where_step(
    cast: &Cast,
    step: &WhereStep,
    present: &Present,
    level: &Level,
    heading: Heading,
) -> Vec<Aimed> {
    let creatures = &present.creatures;
    let origins: Vec<(Origin, Toward)> = if cast.aimed.is_empty() {
        let toward = (cast.next == 0)
            .then_some(cast.toward)
            .flatten()
            .and_then(|id| present.index_of(id))
            .map_or(Toward::Ahead, Toward::Creature);
        vec![(Origin::from_creature(creatures, 0, heading), toward)]
    } else {
        (cast.aimed.iter())
            .map(|aimed| (origin_of(aimed, present), Toward::Ahead))
            .collect()
    };
    origins
        .iter()
        .flat_map(|(origin, toward)| where_hits(level, step, origin, *toward, creatures))
        .map(|hit| aimed_of(&hit, present))
        .collect()
}

/// Whom an effect step lands on, by their place in `present`: the creatures
/// the last where hit (if still here), or the caster if there was none.
pub(super) fn effect_targets(cast: &Cast, present: &Present) -> Vec<usize> {
    if cast.aimed.is_empty() {
        return vec![0];
    }
    let mut targets: Vec<usize> = (cast.aimed.iter())
        .filter_map(|aimed| aimed.creature.and_then(|id| present.index_of(id)))
        .collect();
    // Hit by two missiles is still one creature healed once a step.
    targets.sort_unstable();
    targets.dedup();
    targets
}

/// Where a later step starts from something hit: its body where it is now,
/// if it is still here, else the spot.
fn origin_of(aimed: &Aimed, present: &Present) -> Origin {
    match aimed.creature.and_then(|id| present.index_of(id)) {
        Some(index) => Origin::from_creature(&present.creatures, index, aimed.heading),
        None => Origin {
            at: aimed.at,
            height: aimed.height,
            heading: aimed.heading,
            creature: None,
        },
    }
}

/// A hit kept by id.
fn aimed_of(hit: &Hit, present: &Present) -> Aimed {
    Aimed {
        creature: hit.creature.map(|index| present.ids[index]),
        at: hit.at,
        height: hit.height,
        heading: hit.heading,
    }
}
