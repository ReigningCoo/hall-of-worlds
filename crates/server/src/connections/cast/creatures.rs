//! Everyone a spell's step might hit, on the caster's level: the caster,
//! other players there (friends, until parties exist), and the mobs (enemies
//! when unfriendly to the caster, otherwise neutral).

use shared::id::Id;
use shared::spell::{Creature, Side, resistance, will};

use super::super::Client;
use crate::ServerWorld;
use crate::entities::mobs_for_spell;
use crate::storage::SiteId;

/// Who is about, by place: `ids[i]` is `creatures[i]`'s; the caster first.
pub(super) struct Present {
    pub ids: Vec<Id>,
    pub creatures: Vec<Creature>,
    /// Which are players, by their place in `clients`.
    pub players: Vec<Option<usize>>,
    /// What each resists harmful magic with (`spell::resistance`).
    pub resistances: Vec<u64>,
    /// What each resists paralysis and illusions with (`spell::will`).
    pub wills: Vec<u64>,
}

impl Present {
    /// The place of the creature with this id, if it is here.
    pub fn index_of(&self, id: Id) -> Option<usize> {
        self.ids.iter().position(|one| *one == id)
    }
}

/// Everyone on `place` as client `caster` sees them, if the caster is still
/// there.
pub(super) fn present(
    clients: &[Client],
    caster: usize,
    world: &ServerWorld,
    place: (SiteId, u32),
) -> Option<Present> {
    let me = &clients[caster];
    let here = |client: &Client| {
        !client.gone
            && client.world == me.world
            && client.visit.as_ref().map(|visit| visit.population()) == Some(place)
    };
    if !here(me) {
        return None;
    }
    let mut present = Present {
        ids: vec![me.entity_id()],
        creatures: vec![Creature {
            body: me.body(world)?,
            side: Side::Caster,
        }],
        players: vec![Some(caster)],
        resistances: vec![me.resistance()],
        wills: vec![me.will()],
    };
    for (index, other) in clients.iter().enumerate() {
        let Some(body) = other.body(world).filter(|_| index != caster && here(other)) else {
            continue;
        };
        present.ids.push(other.entity_id());
        present.creatures.push(Creature {
            body,
            side: Side::Friend,
        });
        present.players.push(Some(index));
        present.resistances.push(other.resistance());
        present.wills.push(other.will());
    }
    let kind = me.inventory.creature;
    for mob in mobs_for_spell(world, place, kind, me.entity_id()) {
        present.ids.push(mob.id);
        let side = if mob.enemy {
            Side::Enemy
        } else {
            Side::Neutral
        };
        present.creatures.push(Creature {
            body: mob.body,
            side,
        });
        present.players.push(None);
        present.resistances.push(mob.resistance);
        present.wills.push(mob.will);
    }
    Some(present)
}

impl Client {
    /// What the player resists harmful magic with: their Aura, with every
    /// bonus, and their Warding, more under a Warding spell.
    fn resistance(&self) -> u64 {
        resistance(&self.bonused.attributes, Some(&self.sheet.skills)) * self.bonused.resisting
            / 1000
    }

    /// What the player resists paralysis and illusions with: their Will and
    /// their Warding, more under a Warding spell.
    fn will(&self) -> u64 {
        will(&self.bonused.attributes, Some(&self.sheet.skills)) * self.bonused.resisting / 1000
    }
}

#[cfg(test)]
impl Client {
    /// What the player resists harmful magic with now, for tests.
    pub(crate) fn resistance_for_test(&self) -> u64 {
        self.resistance()
    }
}
