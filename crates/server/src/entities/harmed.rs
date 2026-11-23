//! A mob harmed by a player, by a blow or a spell: it loses the Health,
//! remembers who did it, does what its standing with them now says, and dies
//! if that was the last of it.

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;
use shared::vitals::Points;

use super::gear::{bury, record_of};
use super::timid::is_timid;
use super::{ATTACK_GAP, Doing, FRIENDLY_GAP, Mob, SETTLING_STEPS};
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
/// what its standing with them now says (`after_hit`), and dies if that was
/// the last of its Health. `None` if it is no longer there.
pub(crate) fn harm_mob(
    world: &mut ServerWorld,
    (place, here_index): ((SiteId, u32), u32),
    mob: Id,
    harm: Points,
    by: Harmer,
    tick: u64,
) -> Option<MobHit> {
    let population = world.populations.0.get_mut(&place)?;
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
    if let Some(doing) = after_hit(one, &by, feeling) {
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
    record_of(world, kept_with, keep).memory = memory;
    if dead {
        bury(world, place, here_index, mob, true);
    }
    Some(MobHit { harm, left, kind })
}

/// What a mob does once hit, by how it now stands with whoever hit it: a timid
/// one keeps away, an unfriendly one goes for them, and one only cooled to
/// neutral stops coming over but doesn't fight back. `None` to carry on.
fn after_hit(mob: &Mob, by: &Harmer, feeling: Feeling) -> Option<Doing> {
    if is_timid(mob) {
        let fleeing = matches!(mob.doing, Doing::Fleeing { from, .. } if from == by.id);
        return (!fleeing).then_some(Doing::Fleeing {
            from: by.id,
            struck: false,
        });
    }
    match (feeling, mob.doing) {
        (Feeling::Unfriendly, Doing::Going { after, gap, .. })
            if after == by.id && gap == ATTACK_GAP =>
        {
            None
        }
        (Feeling::Unfriendly, _) => Some(Doing::Going {
            after: by.id,
            last_seen: by.position,
            gap: ATTACK_GAP,
        }),
        (_, Doing::Going { after, gap, .. }) if after == by.id && gap == FRIENDLY_GAP => {
            Some(Doing::Settling {
                steps_left: SETTLING_STEPS,
            })
        }
        _ => None,
    }
}
