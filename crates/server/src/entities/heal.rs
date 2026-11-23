//! Mobs being healed by a player's spell.

use shared::combat::Body;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;
use shared::vitals::{Points, Vital};

use crate::ServerWorld;
use crate::combat::health_left;
use crate::storage::SiteId;

/// A mob a spell might hit: where its body is, and whether it is the
/// caster's enemy (unfriendly as they see it, `Mob::feeling_shown`).
pub(crate) struct SpellMob {
    pub id: Id,
    pub body: Body,
    pub enemy: bool,
}

/// Every mob on this site level (`Visit::population`) as a player of `kind`
/// with entity id `caster` casting a spell sees them.
pub(crate) fn mobs_for_spell(
    world: &ServerWorld,
    place: (SiteId, u32),
    kind: CreatureKind,
    caster: Id,
) -> Vec<SpellMob> {
    let Some(population) = world.populations.0.get(&place) else {
        return Vec::new();
    };
    let character = CharacterId(caster);
    let relief = population.relief();
    (population.mobs.iter())
        .map(|one| SpellMob {
            id: one.id,
            body: Body::on(relief, one.spot, one.size()),
            enemy: one.feeling_shown(kind, Some(&character)) == Feeling::Unfriendly,
        })
        .collect()
}

/// A mob on this site level is given this much Health, never past its most:
/// how much it has now, per mille, and whether it gained any. `None` if it is
/// no longer there. It doesn't turn on the healer, whoever it is fighting:
/// taming is to build on this.
pub(crate) fn heal_mob(
    world: &mut ServerWorld,
    place: (SiteId, u32),
    mob: Id,
    amount: Points,
    tick: u64,
) -> Option<(u16, bool)> {
    let population = world.populations.0.get_mut(&place)?;
    let one = population.mobs.iter_mut().find(|one| one.id == mob)?;
    let limits = one.limits();
    let before = one.vitals.at(Vital::Health, limits, tick);
    one.vitals.restore(Vital::Health, amount, limits, tick);
    let gained = one.vitals.at(Vital::Health, limits, tick) > before;
    Some((health_left(&one.vitals, limits, tick), gained))
}
