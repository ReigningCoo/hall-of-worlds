//! A mob's blow landing on another mob (mob behaviour plan PR 4): by the same
//! combat rules as on a player (`Fighter::harm`), then harmed as by a player
//! (`harm_mob`): it remembers the attacker by its id, turns on it or flees,
//! its band and allies who see it join in, and it may die.

use shared::combat::Fighter;

use super::fight::MobBlow;
use super::harmed::{Harmer, harm_mob};
use crate::ServerWorld;
use crate::combat::luck;
use crate::storage::SiteId;

/// Lands `blow`, struck on `place` this tick, if it is a blow landing and both
/// mobs are still there.
pub(super) fn strike_mob(world: &mut ServerWorld, place: (SiteId, u32), blow: MobBlow, tick: u64) {
    let MobBlow::Landed {
        mob,
        target,
        kind,
        attributes,
        wielded,
        guard,
        swing,
    } = blow
    else {
        return;
    };
    let Some(population) = world.populations.0.get(&place) else {
        return;
    };
    let find = |id| population.mobs.iter().find(|one| one.id == id);
    let (Some(attacker), Some(defender)) = (find(mob), find(target)) else {
        return;
    };
    let fighter = Fighter::mob(kind, &attributes, wielded, guard);
    let harm = fighter.harm(&swing, &defender.fighter(), &mut luck(tick, mob, target));
    let by = Harmer {
        id: mob,
        position: attacker.position,
        kind,
        member: Some(attacker.member()),
    };
    // The wilderness's live areas are all on its one level.
    let here_index = match place.0 {
        SiteId::Wilderness => 0,
        _ => place.1,
    };
    harm_mob(world, (place, here_index), target, harm, by, tick);
}
