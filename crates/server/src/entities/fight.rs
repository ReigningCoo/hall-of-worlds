//! Mobs fighting: attacking an enemy in reach, and being hit by a player.

use shared::combat::{Body, Fighter, Guard, Swing, Wielded, in_reach};
use shared::creature::CreatureKind;
use shared::dungeon::Level;
use shared::id::Id;
use shared::relief::Relief;
use shared::sheet::Attributes;
use shared::vitals::{Points, Vital};

use super::close_in::close_in;
use super::harmed::{Harmer, MobHit, harm_mob};
use super::timid::strikes_at;
use super::{ATTACK_GAP, Doing, Mob, Spot, Taken};
use crate::ServerWorld;
use crate::combat::{Act, Pending, luck, ticks_for};
use crate::storage::SiteId;

/// What a mob did to a player this tick, for the player's client to be told
/// once every level has had its turn.
pub(crate) enum MobBlow {
    Started {
        mob: Id,
        target: Id,
        lands_in: u32,
        ready_in: u32,
    },
    Landed {
        mob: Id,
        target: Id,
        kind: CreatureKind,
        attributes: Attributes,
        wielded: Wielded,
        /// What spells on it do to its blow (a Curse).
        guard: Guard,
        swing: Swing,
    },
    Fizzled {
        mob: Id,
        target: Id,
    },
}

impl Mob {
    fn fighter(&self) -> Fighter<'_> {
        Fighter {
            kind: self.kind,
            attributes: &self.attributes,
            skills: None,
            wielded: self.wielded,
            guard: self.spells.guard(),
        }
    }

    /// How much Health it has at this tick.
    pub(crate) fn health(&self, tick: u64) -> Points {
        let limits = self.limits();
        self.vitals.at(Vital::Health, limits, tick)
    }

    /// When it is next due: when its blow lands, or when it can act again.
    pub(super) fn next_due(&self, tick: u64) -> u64 {
        match self.fight.pending {
            Some(pending) => pending.lands_at.max(tick + 1),
            None => self.energy.ready_at(tick + 1, self.speed),
        }
    }
}

/// A mob's turn in a fight, if it is in one: its blow landing, or a new one
/// begun at an enemy in reach. What it cost; `None` to go about its business.
/// One not yet in reach of an enemy in the next cell closes in on them
/// (`close_in`). `taken` is where everyone stands, by cell.
pub(super) fn mob_turn(
    mob: &mut Mob,
    (level, relief): (&Level, Option<&Relief>),
    spots: &[Spot],
    (tick, taken): (u64, &Taken),
    blows: &mut Vec<MobBlow>,
) -> Option<u32> {
    let bodies = |mob: &Mob, target: Id| {
        let spot = spots.iter().find(|spot| spot.id == target)?;
        let them = Body::on(relief, spot.at, spot.kind.def().size);
        Some((Body::on(relief, mob.spot, mob.size()), them))
    };
    let reaches = |mob: &Mob, target: Id, swing: &Swing| {
        bodies(mob, target)
            .is_some_and(|(me, them)| in_reach(level, &me, swing.weapon.reach(), &them))
    };
    if let Some(pending) = mob.fight.pending.take() {
        let Act::Strike(swing) = pending.act else {
            return Some(0);
        };
        mob.fight.attacks += 1;
        let target = pending.target;
        blows.push(if reaches(mob, target, &swing) {
            MobBlow::Landed {
                mob: mob.id,
                target,
                kind: mob.kind,
                attributes: mob.attributes,
                wielded: mob.wielded,
                guard: mob.spells.guard(),
                swing,
            }
        } else {
            MobBlow::Fizzled {
                mob: mob.id,
                target,
            }
        });
        return Some(0);
    }
    // One after an enemy, closing in to attack, or a timid one caught near one.
    let timid = strikes_at(mob, spots).map(|spot| spot.id);
    let after = match (timid, mob.doing) {
        (Some(from), _) => from,
        (None, Doing::Going { after, gap, .. }) if gap == ATTACK_GAP => after,
        _ => return None,
    };
    let swing = mob.fighter().swing(mob.fight.attacks);
    if !reaches(mob, after, &swing) {
        let (me, them) = bodies(mob, after)?;
        return close_in(mob, level, (&me, &them), swing.weapon.reach(), taken);
    }
    let lands_in = ticks_for(swing.wind_up, mob.speed);
    mob.fight.pending = Some(Pending {
        target: after,
        act: Act::Strike(swing),
        lands_at: tick + lands_in,
    });
    if timid.is_some() {
        // Once: it steps away before it may strike again.
        mob.doing = Doing::Fleeing {
            from: after,
            struck: true,
        };
    }
    // Whatever a player does to it now is self-defence.
    let player = spots.iter().find(|spot| spot.id == after);
    if let Some(character) = player.and_then(|spot| spot.character) {
        mob.memory.remember_attacking(character);
    }
    blows.push(MobBlow::Started {
        mob: mob.id,
        target: after,
        lands_in: lands_in as u32,
        ready_in: ticks_for(swing.cost, mob.speed) as u32,
    });
    Some(swing.cost)
}

/// Who struck a mob, and with what.
pub(crate) struct Striker<'a> {
    pub id: Id,
    pub position: (i64, i64),
    pub fighter: Fighter<'a>,
    pub swing: Swing,
    /// A kick, not a blow: whether it shoves the mob a step away (`Fighter::kick_harm`).
    pub kicked: Option<bool>,
}

/// A player's blow lands on a mob on `place` (whose own level is `here_index`):
/// it is harmed as `harm_mob` says. `None` if it is no longer there.
pub(crate) fn hit_mob(
    world: &mut ServerWorld,
    (place, here_index): ((SiteId, u32), u32),
    mob: Id,
    by: Striker,
    tick: u64,
) -> Option<MobHit> {
    let population = world.populations.0.get_mut(&place)?;
    let one = population.mobs.iter_mut().find(|one| one.id == mob)?;
    let mut rng = luck(tick, by.id, mob);
    let harm = match by.kicked {
        Some(shoved) => by.fighter.kick_harm(&one.fighter(), shoved, &mut rng),
        None => by.fighter.harm(&by.swing, &one.fighter(), &mut rng),
    };
    let harmer = Harmer {
        id: by.id,
        position: by.position,
        kind: by.fighter.kind,
    };
    harm_mob(world, (place, here_index), mob, harm, harmer, tick)
}
