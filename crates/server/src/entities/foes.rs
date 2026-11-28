//! Mobs fighting mobs (mob behaviour plan PR 4): which mobs on its level a mob
//! would fight, and how it feels about another mob or whoever harmed it. Rival
//! bands and enemies (`behaviour::Relation`) start out unfriendly; its band,
//! allies and the rest start out as nothing to it, so it neither goes for nor
//! keeps away from them. A mob that harms it is remembered by its own id, as
//! a character would be (`Memory::remember_harm`). Hunting (PR 5): the wild
//! prey it hunts are its prey (`hunt.rs`), and those that hunt it its foes.

use shared::behaviour::{Behaviours, Relation};
use shared::protocol::CharacterId;
use shared::sheet::Feeling;

use super::choice::Other;
use super::harmed::Harmer;
use super::{Mob, Spot};

/// The mobs on its level a mob is against, as of the start of this tick: rival
/// bands and enemies (`fights`), the wild prey it hunts (`prey`), and those
/// that hunt it, which it keeps away from (`hunters`). Not saved.
#[derive(Clone, Debug, Default)]
pub(super) struct Foes {
    pub fights: Vec<shared::id::Id>,
    pub prey: Vec<shared::id::Id>,
    pub hunters: Vec<shared::id::Id>,
}

impl Foes {
    /// Whether it is against this mob, and so starts out unfriendly to it.
    pub(super) fn contains(&self, id: &shared::id::Id) -> bool {
        self.fights.contains(id) || self.prey.contains(id) || self.hunters.contains(id)
    }

    /// Whether this mob hunts it.
    pub(super) fn is_hunter(&self, id: shared::id::Id) -> bool {
        self.hunters.contains(&id)
    }

    /// Whether this mob is prey it hunts.
    pub(super) fn is_prey(&self, id: shared::id::Id) -> bool {
        self.prey.contains(&id)
    }
}

/// How a mob starts out feeling about another, by how it takes to it.
pub(super) fn start_feeling(relation: Relation) -> Feeling {
    match relation {
        Relation::Rival | Relation::Enemy => Feeling::Unfriendly,
        Relation::Kin | Relation::Ally | Relation::Neutral => Feeling::None,
    }
}

/// Whether a mob fights other mobs and may be fought by them: never a kept
/// animal in its pen, someone escorted, or one developer tools hold still.
pub(super) fn in_mob_fights(mob: &Mob) -> bool {
    !mob.penned && mob.escort.is_none() && !mob.still
}

impl Mob {
    /// Works out which of `others` (its level's mobs as they stood at the
    /// start of the tick) it would fight.
    pub(super) fn find_foes(&mut self, others: &[Other], behaviours: &Behaviours) {
        self.foes.fights.clear();
        self.foes.prey.clear();
        self.foes.hunters.clear();
        if !in_mob_fights(self) {
            return;
        }
        let me = self.member();
        let hunts = &behaviours.of(self.kind).hunts;
        for other in (others.iter()).filter(|other| other.id != self.id && other.fights) {
            let them = other.member.kind;
            if hunts.contains(&them) {
                self.foes.prey.push(other.id);
            } else if behaviours.of(them).hunts.contains(&self.kind) {
                self.foes.hunters.push(other.id);
            } else if start_feeling(behaviours.relation(me, other.member)) != Feeling::None {
                self.foes.fights.push(other.id);
            }
        }
    }

    /// How it feels about someone it sees: a player by its kind and what it
    /// remembers of them (`feeling`), another mob by whether it is a foe and
    /// what it remembers of it.
    pub(super) fn feeling_towards(&self, spot: &Spot) -> Feeling {
        match spot.member {
            None => self.feeling(spot.kind, spot.character.as_ref()),
            Some(_) => {
                let start = match self.foes.contains(&spot.id) {
                    true => Feeling::Unfriendly,
                    false => Feeling::None,
                };
                self.feeling_from(Some(&CharacterId(spot.id)), start)
            }
        }
    }

    /// How it started out feeling about whoever harmed it, before that.
    pub(super) fn start_towards(&self, by: &Harmer, behaviours: &Behaviours) -> Feeling {
        match by.member {
            None => self.kind.feeling_about(by.kind),
            Some(them) => start_feeling(behaviours.relation(self.member(), them)),
        }
    }

    /// How it feels about whoever harmed it, with what it remembers of them.
    pub(super) fn feeling_about(&self, by: &Harmer, behaviours: &Behaviours) -> Feeling {
        let start = self.start_towards(by, behaviours);
        self.feeling_from(Some(&CharacterId(by.id)), start)
    }
}

/// Everyone a level's mobs may see this tick: its players (`players`), then
/// every mob that may fight or be fought (`in_mob_fights`), where it stands.
pub(super) fn with_mobs(players: &[Spot], mobs: &[Mob]) -> Vec<Spot> {
    let mobs = mobs
        .iter()
        .filter(|mob| in_mob_fights(mob))
        .map(|mob| Spot {
            id: mob.id,
            character: None,
            kind: mob.kind,
            position: mob.position,
            at: mob.spot,
            scent: mob.scent,
            member: Some(mob.member()),
        });
    players.iter().cloned().chain(mobs).collect()
}
