//! Mobs chilled by a spell's ice (magic plan PR 11): slower
//! (`spell::CHILL_SPEED_PERCENT`) until the chill ends, their own speed back
//! after (`Mob::thaw`, every tick). Not saved: a mob unloaded is warm again.

use shared::id::Id;
use shared::spell::CHILL_SPEED_PERCENT;

use super::Mob;
use crate::ServerWorld;
use crate::storage::SiteId;

impl Mob {
    /// Its own speed back once its chill has ended at `tick`.
    pub(super) fn thaw(&mut self, tick: u64) {
        if let Some((until, own)) = self.chilled
            && tick >= until
        {
            self.energy.settle(tick, self.speed);
            self.speed = own;
            self.chilled = None;
        }
    }
}

/// A mob on this site level is chilled until `until` (an action tick), a
/// chill it has already lasting the longer. Whether it was there.
pub(crate) fn chill_mob(
    world: &mut ServerWorld,
    place: (SiteId, u32),
    mob: Id,
    until: u64,
    tick: u64,
) -> bool {
    let Some(population) = world.populations.0.get_mut(&place) else {
        return false;
    };
    let Some(one) = population.mobs.iter_mut().find(|one| one.id == mob) else {
        return false;
    };
    let own = one.chilled.map_or(one.speed, |(_, own)| own);
    let until = one.chilled.map_or(until, |(was, _)| was.max(until));
    one.energy.settle(tick, one.speed);
    one.speed = (own * CHILL_SPEED_PERCENT / 100).max(1);
    one.chilled = Some((until, own));
    true
}
