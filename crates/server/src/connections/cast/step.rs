//! What one where step of a spell hits (`shared::spell::where_hits`), from
//! the caster for the first, else from each thing the last where hit; and
//! whom an effect step lands on.

use shared::dungeon::Level;
use shared::facing::Heading;
use shared::spell::{
    Hit, Look, Origin, Shown, Toward, WhereStep, part_hits, shown, shown_part, where_hits,
};

use super::creatures::Present;
use super::{Aimed, Cast};

/// What `step`'s where now due (`Cast::part`) hits, as what comes next
/// starts from (the caster facing `heading`), and how it looked from each
/// place it started, to draw. Only the first where of the first step goes
/// at the caster's target; a merged where goes on from each hit of the one
/// before.
pub(super) fn where_step(
    cast: &Cast,
    step: &WhereStep,
    present: &Present,
    level: &Level,
    heading: Heading,
) -> (Vec<Aimed>, Vec<Shown>) {
    let creatures = &present.creatures;
    let look = Look::from_step(&cast.spell, cast.next);
    let (mut aimed, mut looks) = (Vec::new(), Vec::new());
    let part = cast.part;
    for (origin, toward) in origins(cast, present, heading) {
        if part > 0 {
            let hits = part_hits(level, step, part, &origin, creatures);
            let at = (step, part, look);
            looks.extend(shown_part(level, at, &origin, creatures, &hits));
            aimed.extend(hits.iter().map(|hit| aimed_of(hit, present)));
            continue;
        }
        let hits = where_hits(level, step, &origin, toward, creatures);
        looks.extend(shown(
            level,
            (step, look),
            &origin,
            toward,
            creatures,
            &hits,
        ));
        aimed.extend(hits.iter().map(|hit| aimed_of(hit, present)));
    }
    (aimed, looks)
}

/// Where a where step starts, and what at: from the caster for the first
/// (at their target), else from each thing the last where hit, going on.
pub(super) fn origins(cast: &Cast, present: &Present, heading: Heading) -> Vec<(Origin, Toward)> {
    let creatures = &present.creatures;
    if cast.aimed.is_empty() {
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
    }
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

/// For each target (by its place in `present`), the way away from where the
/// last where came from, as it hit them; the caster's own `heading` for one
/// the spell is on without a where.
pub(super) fn away_headings(
    cast: &Cast,
    present: &Present,
    targets: &[usize],
    heading: Heading,
) -> Vec<Heading> {
    (targets.iter())
        .map(|&index| {
            let id = present.ids[index];
            (cast.aimed.iter())
                .find(|aimed| aimed.creature == Some(id))
                .map_or(heading, |aimed| aimed.heading)
        })
        .collect()
}

/// The cells an effect step lands on: every spot the last where hit,
/// creature or not, or the caster's own cell.
pub(super) fn landing_cells(cast: &Cast, present: &Present) -> Vec<(i64, i64)> {
    let mut cells: Vec<(i64, i64)> = if cast.aimed.is_empty() {
        vec![present.creatures[0].body.at.cell()]
    } else {
        (cast.aimed.iter())
            .map(
                |aimed| match aimed.creature.and_then(|id| present.index_of(id)) {
                    Some(index) => present.creatures[index].body.at.cell(),
                    None => aimed.at.cell(),
                },
            )
            .collect()
    };
    cells.sort_unstable();
    cells.dedup();
    cells
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
pub(super) fn aimed_of(hit: &Hit, present: &Present) -> Aimed {
    Aimed {
        creature: hit.creature.map(|index| present.ids[index]),
        at: hit.at,
        height: hit.height,
        heading: hit.heading,
    }
}
