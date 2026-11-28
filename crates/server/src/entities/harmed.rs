//! A mob harmed by a player or another mob, by a blow or a spell: it loses the Health,
//! remembers who did it, does what its standing with them now says, and dies
//! if that was the last of it, eaten if a hunter killed it (`hunt`). Its band
//! and allies who see it join in (`defend`); a social one calls out, and its
//! band who hear come over (`curious`).

use shared::behaviour::Member;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::CharacterId;
use shared::vitals::Points;

use super::Doing;
use super::choice::{after_harm, friends_seen, others_of};
use super::curious::call_out;
use super::defend::rally;
use super::gear::{bury, record_of};
use crate::ServerWorld;
use crate::combat::{Harm, health_left};
use crate::storage::SiteId;

/// What a player (or another mob) did to a mob.
pub(crate) struct MobHit {
    pub harm: Points,
    /// Health left, per mille: none when it died.
    pub left: u16,
    /// What kind of creature it is.
    pub kind: CreatureKind,
}

/// Who harmed a mob, a player or another mob: who, where they stand, and what
/// they are.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Harmer {
    pub id: Id,
    pub position: (i64, i64),
    pub kind: CreatureKind,
    /// A mob's race and band (mob behaviour plan PR 4); `None` for a player.
    pub member: Option<Member>,
}

/// A mob on `place` (whose own level is `here_index`) takes `harm` from a
/// player or another mob: it is harmed, remembers it (`Memory::remember_harm`, saved), does
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
    // It turns to whoever harmed it, and knows they are there.
    one.face(shared::spot::Spot::middle_of(by.position));
    one.noticed(by.id, tick);
    if harm != Points::ZERO {
        // Hurt, a social one calls out: its band who hear come (`curious`).
        call_out(&mut population.noises, one, &behaviours);
        world.harmed.push(Harm {
            id: mob,
            left,
            told: by.id,
        });
    }
    // A player's entity id is their character's; a mob is remembered by its
    // own id the same way (`foes.rs`).
    let character = CharacterId(by.id);
    let start = one.start_towards(&by, &behaviours);
    one.memory.remember_harm(character, harm, start);
    let feeling = one.feeling_about(&by, &behaviours);
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
    // Killed by a hunter, it is eaten: nothing is left of it (`hunt`).
    let eaten = dead && super::hunt::eaten(&behaviours, &by, kind);
    let lay = one.position;
    // Its band and allies who see it come to its help.
    let rallied = rally(population, mob, &by, (&others, &behaviours), place.0, tick);
    if eaten {
        let due = &mut population.due;
        super::hunt::feast(&mut population.mobs, (mob, lay), by.id, tick, due);
    }
    record_of(world, kept_with, keep).memory = memory;
    for one in rallied {
        record_of(world, one.kept_with, one.keep).memory = one.memory;
    }
    if dead {
        bury(world, place, here_index, mob, !eaten);
    }
    Some(MobHit { harm, left, kind })
}
