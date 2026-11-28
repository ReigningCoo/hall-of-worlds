//! A mob harmed by a player, by a blow or a spell: it loses the Health,
//! remembers who did it, does what its standing with them now says, and dies
//! if that was the last of it. Its band and allies who see it join in
//! (`defend`).

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::CharacterId;
use shared::vitals::Points;

use super::Doing;
use super::choice::{after_harm, friends_seen, others_of};
use super::defend::rally;
use super::gear::{bury, record_of};
use crate::ServerWorld;
use crate::combat::{Harm, health_left};
use crate::storage::SiteId;

/// What a player did to a mob.
pub(crate) struct MobHit {
    pub harm: Points,
    /// Health left, per mille: none when it died.
    pub left: u16,
    /// What kind of creature it is.
    pub kind: CreatureKind,
}

/// The player who harmed a mob: who, where they stand, and what they are.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Harmer {
    pub id: Id,
    pub position: (i64, i64),
    pub kind: CreatureKind,
}

/// A mob on `place` (whose own level is `here_index`) takes `harm` from a
/// player: it is harmed, remembers it (`Memory::remember_harm`, saved), does
/// what its standing with them now says (`choice::after_harm`), and dies if that was
/// the last of its Health. `None` if it is no longer there.
pub(crate) fn harm_mob(
    world: &mut ServerWorld,
    (place, here_index): ((SiteId, u32), u32),
    mob: Id,
    harm: Points,
    by: Harmer,
    tick: u64,
) -> Option<MobHit> {
    let behaviours = world.populations.1.clone();
    let population = world.populations.0.get_mut(&place)?;
    let others = others_of(&population.mobs);
    let one = population.mobs.iter_mut().find(|one| one.id == mob)?;
    let limits = one.limits();
    one.vitals.hurt(harm, limits, tick, 0);
    let left = health_left(&one.vitals, limits, tick);
    let kind = one.kind;
    one.harmed_by.harmed(by.id, tick);
    // It turns to whoever harmed it.
    one.face(shared::spot::Spot::middle_of(by.position));
    if harm != Points::ZERO {
        world.harmed.push(Harm {
            id: mob,
            left,
            told: by.id,
        });
    }
    // A player's entity id is their character's.
    let character = CharacterId(by.id);
    let start = one.kind.feeling_about(by.kind);
    one.memory.remember_harm(character, harm, start);
    let feeling = one.feeling(by.kind, Some(&character));
    let at_rest = one.doing == Doing::Idle;
    let friends = friends_seen(one, &population.level, &others, &behaviours);
    let def = behaviours.of(kind);
    let (doing, scores) = after_harm(one, &by, feeling, (def, friends), tick);
    one.scores = scores;
    if let Some(doing) = doing {
        one.doing = doing;
        one.target = doing.target();
        if at_rest && doing != Doing::Idle {
            let due = one.next_due(tick);
            population.due.insert((due, mob));
        }
    }
    let keep = (one.id, one.home_level, one.home);
    let (memory, kept_with) = (one.memory.clone(), one.kept_with(place.0));
    let dead = one.health(tick) == Points::ZERO;
    // Its band and allies who see it come to its help.
    let rallied = rally(population, mob, &by, (&others, &behaviours), place.0, tick);
    record_of(world, kept_with, keep).memory = memory;
    for one in rallied {
        record_of(world, one.kept_with, one.keep).memory = one.memory;
    }
    if dead {
        bury(world, place, here_index, mob, true);
    }
    Some(MobHit { harm, left, kind })
}
