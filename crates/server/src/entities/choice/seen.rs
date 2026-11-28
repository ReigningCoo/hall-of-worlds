//! What a mob makes of someone it sees, by today's rules (its kind's reaction,
//! a territorial mob's guarding, a timid one's distance), and the other mobs it
//! may count as friends.

use shared::behaviour::{Behaviours, Member, Relation};
use shared::dungeon::{Level, line_is_clear};
use shared::id::Id;
use shared::sheet::{Feeling, Reaction};

use super::super::hunt::HUNT_LEASH;
use super::super::territory::{LEASH, is_territorial};
use super::super::timid::{ENEMY_APART, too_near};
use super::super::{Mob, Spot, cells_apart};

/// What a mob would do about someone, before scoring.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum Wants {
    Nothing,
    Attack,
    Approach,
    Guard,
    /// Step away: they are nearer than it likes.
    KeepAway,
}

/// Someone a mob sees, as its scoring goes.
#[derive(Clone, Copy, Debug)]
pub(in super::super) struct Seen {
    pub id: Id,
    pub position: (i64, i64),
    pub wants: Wants,
    /// Against it: it would go for them, or dislikes them.
    pub enemy: bool,
    /// How many cells away (`cells_apart`).
    pub apart: i64,
    /// Standing by an open fire (`fire::near_fire`).
    pub near_fire: bool,
}

impl Seen {
    /// What `mob` makes of a player it can see.
    pub(in super::super) fn of(mob: &Mob, spot: &Spot, near_fire: bool) -> Self {
        let feeling = mob.feeling_towards(spot);
        let wants = wants(mob, spot, feeling);
        // Prey is no threat to be afraid of.
        let prey = mob.foes.is_prey(spot.id);
        Self {
            id: spot.id,
            position: spot.position,
            wants,
            enemy: !prey && (wants == Wants::Attack || feeling == Feeling::Unfriendly),
            apart: cells_apart(mob.position, spot.position),
            near_fire,
        }
    }
}

/// Today's rules: a hunter always goes after its prey, unless it is too far
/// from home (`HUNT_LEASH`), and prey keeps away from hunters near it; a territorial mob guards against strangers and
/// goes for those it is angry with only near its home; any other goes by its
/// kind's reaction, a timid one keeping away only from those too near.
pub(in super::super) fn wants(mob: &Mob, spot: &Spot, feeling: Feeling) -> Wants {
    if mob.foes.is_prey(spot.id) {
        return match cells_apart(spot.position, mob.home) <= HUNT_LEASH {
            true => Wants::Attack,
            false => Wants::Nothing,
        };
    }
    // Prey keeps away from those hunting it, whatever its temper.
    if mob.foes.is_hunter(spot.id) {
        return match cells_apart(mob.position, spot.position) < ENEMY_APART {
            true => Wants::KeepAway,
            false => Wants::Nothing,
        };
    }
    if is_territorial(mob) && feeling != Feeling::Friendly {
        return match feeling {
            Feeling::None | Feeling::Friendly => Wants::Nothing,
            Feeling::Neutral => Wants::Guard,
            Feeling::Unfriendly if cells_apart(spot.position, mob.home) > LEASH => Wants::Guard,
            Feeling::Unfriendly => Wants::Attack,
        };
    }
    match mob.kind.reaction(feeling) {
        Reaction::Ignore => Wants::Nothing,
        Reaction::Flee if too_near(mob, spot) => Wants::KeepAway,
        Reaction::Flee => Wants::Nothing,
        Reaction::Approach => Wants::Approach,
        Reaction::Attack => Wants::Attack,
    }
}

/// Another mob as it stood at the start of the tick.
#[derive(Clone, Copy, Debug)]
pub(in super::super) struct Other {
    pub id: Id,
    pub member: Member,
    pub position: (i64, i64),
    /// It may fight other mobs and be fought by them (`foes::in_mob_fights`).
    pub fights: bool,
}

/// Every mob on a level as it stands now.
pub(in super::super) fn others_of(mobs: &[Mob]) -> Vec<Other> {
    mobs.iter()
        .map(|mob| Other {
            id: mob.id,
            member: mob.member(),
            position: mob.position,
            fights: super::super::foes::in_mob_fights(mob),
        })
        .collect()
}

/// How many of its band and its allies `mob` can see (within its sight, the
/// way clear: friends behind it count too).
pub(in super::super) fn friends_seen(
    mob: &Mob,
    level: &Level,
    others: &[Other],
    behaviours: &Behaviours,
) -> i32 {
    let sees_within = mob.kind.senses().sight_range();
    let me = mob.member();
    others
        .iter()
        .filter(|other| other.id != mob.id)
        .filter(|other| {
            matches!(
                behaviours.relation(me, other.member),
                Relation::Kin | Relation::Ally
            )
        })
        .filter(|other| {
            cells_apart(other.position, mob.position) <= sees_within
                && line_is_clear(level, mob.position, other.position)
        })
        .count() as i32
}

impl Mob {
    /// Its race and band, for relations.
    pub(in super::super) fn member(&self) -> Member {
        Member {
            kind: self.kind,
            band: self.band,
        }
    }
}
