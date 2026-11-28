//! Alertness (mob behaviour plan PR 7, `shared::alertness`): each mob's
//! `Wariness`. A noise heard makes it Alert, turning to it at rest once its
//! reaction is over (`noise.rs`); seeing a player or a mob it is against makes
//! it Aware; a blow on one caught off guard (Unaware, or Alert and still
//! reacting) is a critical (`fight.rs`, `mob_blows.rs`): someone heard creeping
//! up gets that moment first. Players are told when a mob is Alert to or Aware of them,
//! for the sneaking eye.

use shared::alertness::reaction_ticks;
use shared::dungeon::{Level, line_is_clear};
use shared::id::Id;

use super::{Mob, Spot, cells_apart};
use crate::ServerWorld;
use crate::connections::Client;

/// Whether `mob` sees `spot` now: within its kind's sight, in its cone (so
/// sneaking up behind works) and a clear line.
pub(super) fn in_view(mob: &Mob, level: &Level, spot: &Spot) -> bool {
    spot.id != mob.id
        && cells_apart(spot.position, mob.position) <= mob.kind.senses().sight_range()
        && mob.cone().sees(spot.at)
        && line_is_clear(level, mob.position, spot.position)
}

impl Mob {
    /// How long it takes to turn to a noise, by its Mind row.
    pub(super) fn reaction(&self) -> u64 {
        reaction_ticks(&self.attributes)
    }

    /// Whether it is caught off guard: a blow on it now is a critical.
    pub(super) fn off_guard(&self) -> bool {
        self.wariness.off_guard()
    }

    /// It looks round on `tick`: Aware of a player it sees (the first), else of
    /// a mob it is against, else Alert or calming down.
    pub(super) fn look_round(&mut self, level: &Level, creatures: &[Spot], tick: u64) {
        let seen = |spot: &&Spot| in_view(self, level, spot);
        let player = creatures
            .iter()
            .filter(|spot| spot.member.is_none())
            .find(seen);
        let foe = || {
            (creatures.iter())
                .filter(|spot| self.foes.contains(&spot.id))
                .find(seen)
        };
        let who = player.or_else(foe).map(|spot| spot.id);
        self.wariness.sees(who, tick);
    }

    /// It knows `who` is there (they harmed it): Aware of them.
    pub(super) fn noticed(&mut self, who: Id, tick: u64) {
        self.wariness.sees(Some(who), tick);
    }

    /// The player it is Alert to or Aware of, if any.
    fn wary_of(&self) -> Option<Id> {
        match self.wariness.is_unaware() {
            true => None,
            false => self.wariness.about(),
        }
    }
}

/// Tells each player on a level whether a mob there is Alert to or Aware of
/// them, when that changes.
pub(crate) fn tell_alerts(clients: &mut [Client], world: &ServerWorld) {
    for client in clients.iter_mut() {
        let id = client.entity_id();
        let alerted = (client.visit.as_ref())
            .and_then(|visit| world.populations.0.get(&visit.population()))
            .is_some_and(|population| population.mobs.iter().any(|mob| mob.wary_of() == Some(id)));
        client.tell_mob_alert(alerted);
    }
}
