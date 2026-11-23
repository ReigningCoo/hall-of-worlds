//! A spell's missiles in flight (magic plan PR 6): launched by a missile
//! step from each place it starts (`spell::launch`), then flown a stretch
//! every tick (`Path::meets`), landing on the first creature they may hit
//! that is in their way at that moment, or at the end of their way. Every
//! player on the level is told of each as it is launched and as it lands
//! (`MissileNews`), so their clients can follow it. The step is done once
//! the last has landed: what they landed on are the next step's targets.

use shared::dungeon::Level;
use shared::facing::Heading;
use shared::id::Id;
use shared::protocol::{MissileId, MissileNews, ServerMessage};
use shared::spell::{Path, WhereStep, launch};

use super::super::Client;
use super::creatures::Present;
use super::step::{aimed_of, origins};
use super::{Aimed, Cast};
use crate::TICKS_PER_SECOND;

/// One missile flying.
#[derive(Clone, Debug)]
pub(crate) struct Flying {
    id: MissileId,
    path: Path,
    /// The creature it was launched from, which it never hits.
    from: Option<Id>,
    /// The action tick it was launched on.
    launched: u64,
    /// Units flown so far.
    along: u64,
}

/// The ways `step`'s missiles will fly, each with the creature it starts
/// from. None (a first step's Friend or Enemy finding nobody) is a fizzle for
/// the caller.
pub(super) fn missile_paths(
    cast: &Cast,
    (step, present, level): (&WhereStep, &Present, &Level),
    heading: Heading,
) -> Vec<(Path, Option<Id>)> {
    (origins(cast, present, heading).into_iter())
        .filter_map(|(origin, toward)| {
            let path = launch(level, step, &origin, toward, &present.creatures)?;
            Some((path, origin.creature.map(|index| present.ids[index])))
        })
        .collect()
}

/// Launches these missiles for client `i`'s spell, telling everyone on the
/// level.
pub(super) fn launch_missiles(
    clients: &mut [Client],
    i: usize,
    cast: &mut Cast,
    (paths, present): (Vec<(Path, Option<Id>)>, &Present),
    tick: u64,
) {
    let caster = clients[i].entity_id();
    for (path, from) in paths {
        let id = MissileId {
            caster,
            number: clients[i].casting.missiles,
        };
        clients[i].casting.missiles = clients[i].casting.missiles.wrapping_add(1);
        tell(clients, present, MissileNews::Flying { id, path });
        cast.flying.push(Flying {
            id,
            path,
            from,
            launched: tick,
            along: 0,
        });
    }
    cast.landed.clear();
}

/// Flies each missile on to where it is at `tick`, landing those that meet
/// someone or reach the end of their way. What they all landed on, once the
/// last has landed.
pub(super) fn fly(
    clients: &mut [Client],
    cast: &mut Cast,
    present: &Present,
    tick: u64,
) -> Option<Vec<Aimed>> {
    let creatures = &present.creatures;
    let mut flying = std::mem::take(&mut cast.flying);
    flying.retain_mut(|missile| {
        let millis = tick.saturating_sub(missile.launched) * 1000 / TICKS_PER_SECOND as u64;
        let along = missile.path.along_after(millis);
        let except = missile.from.and_then(|id| present.index_of(id));
        let met = missile
            .path
            .meets((missile.along, along), creatures, except);
        let hit = match met {
            Some((_, index)) => missile.path.hit(creatures, index),
            None if along >= missile.path.length => missile.path.end(),
            None => {
                missile.along = along;
                return true;
            }
        };
        let id = missile.id;
        tell(clients, present, MissileNews::Landed { id, at: hit.at });
        cast.landed.push(aimed_of(&hit, present));
        false
    });
    cast.flying = flying;
    cast.flying
        .is_empty()
        .then(|| std::mem::take(&mut cast.landed))
}

/// Tells every player on the level.
fn tell(clients: &mut [Client], present: &Present, news: MissileNews) {
    for player in present.players.iter().flatten() {
        clients[*player].link.send(&ServerMessage::Missile(news));
    }
}
