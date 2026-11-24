//! Mobs' pace under spells: chilled by a spell's ice (magic plan PR 11), a
//! third slower (`spell::CHILL_SPEED_PERCENT`) until the chill ends, and
//! hastened or slowed by Haste (PR 13, `MobSpells::pace`). Its speed is its
//! own with both, worked out again whenever either changes (`Mob::repace`),
//! its own back once they are over (`Mob::thaw`, every tick). Not saved: a
//! mob unloaded is warm again.

use shared::id::Id;
use shared::spell::CHILL_SPEED_PERCENT;

use super::Mob;
use crate::ServerWorld;
use crate::storage::SiteId;

impl Mob {
    /// Its own speed back once its chill has ended at `tick`.
    pub(super) fn thaw(&mut self, tick: u64) {
        if self.chilled.is_some_and(|until| tick >= until) {
            self.chilled = None;
            self.repace(tick);
        }
    }

    /// Its speed from `tick` on: its own, chilled, by its spells' pace; the
    /// energy gained until then at the speed it had.
    pub(super) fn repace(&mut self, tick: u64) {
        self.energy.settle(tick, self.speed);
        let chill = if self.chilled.is_some() {
            CHILL_SPEED_PERCENT
        } else {
            100
        };
        let paced = u64::from(self.own_speed) * u64::from(chill) / 100 * self.spells.pace() / 1000;
        self.speed = paced.clamp(1, u64::from(u32::MAX)) as u32;
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
    one.chilled = Some(one.chilled.map_or(until, |was| was.max(until)));
    one.repace(tick);
    true
}
