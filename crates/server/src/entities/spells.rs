//! Spells lying on mobs (magic plan PR 12), kept as players keep them
//! (`bonus::Effect`, `Source::Spell`) but not saved: a mob unloaded is free
//! of them. What they do to blows (`combat::Guard`) and to resisting magic
//! is worked out again whenever they change; a spell's poison harms it once
//! a second, as from whoever cast it, so a poisoned mob can die of it.
//! Paralysis is the energy it would have gained meanwhile, spent at once.

use shared::bonus::{Effect, Source, Stat, Totals};
use shared::combat::Guard;
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::id::Id;
use shared::spell::{Ends, PARALYSED, POISONED};
use shared::vitals::{Points, Vital};

use super::Mob;
use super::harmed::{Harmer, harm_mob};
use crate::ServerWorld;
use crate::storage::SiteId;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// What spells lie on a mob.
#[derive(Clone, Debug)]
pub(crate) struct MobSpells {
    effects: Vec<Effect>,
    guard: Guard,
    /// Resistance to harmful magic per mille (Warding).
    resisting: u64,
    /// Who poisoned it, and on which of their levels, for the harm to count
    /// as theirs; and when the poison next harms.
    poisoner: Option<(Harmer, u32)>,
    next_poison: u64,
}

impl Default for MobSpells {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
            guard: Guard::NONE,
            resisting: 1000,
            poisoner: None,
            next_poison: 0,
        }
    }
}

impl MobSpells {
    /// What they do to blows.
    pub(crate) fn guard(&self) -> Guard {
        self.guard
    }

    /// Resistance to harmful magic per mille.
    pub(crate) fn resisting(&self) -> u64 {
        self.resisting
    }

    fn refresh(&mut self, tick: u64) {
        self.effects.retain(|effect| effect.lasts(tick));
        let totals = Totals::of(self.effects.iter().flat_map(|e| &e.bonuses));
        self.guard = Guard::of(&totals);
        self.resisting = totals.per_mille(Stat::Resistance);
        if !self
            .effects
            .iter()
            .any(|e| e.source == Source::Spell(POISONED))
        {
            self.poisoner = None;
        }
    }

    /// The poison's harm a second, if it is due at `tick`, and who did it.
    fn poison_due(&mut self, tick: u64) -> Option<(Points, Harmer, u32)> {
        let (by, index) = self.poisoner?;
        if tick < self.next_poison {
            return None;
        }
        self.next_poison = tick + SECOND;
        let totals = Totals::of(self.effects.iter().flat_map(|e| &e.bonuses));
        let harm = Points(totals.draining(Vital::Health));
        Some((harm, by, index))
    }
}

/// A spell's `effect` lies on mob `mob` on `place` from `tick`, in place of
/// one from the same source; a poison counts as `poisoner`'s, harming from a
/// second on. Paralysis spends the energy it lasts. Whether it was there.
pub(crate) fn lay_on_mob(
    world: &mut ServerWorld,
    place: (SiteId, u32),
    mob: Id,
    effect: Effect,
    poisoner: Option<(Harmer, u32)>,
    tick: u64,
) -> bool {
    let Some(one) = find(world, place, mob) else {
        return false;
    };
    if effect.source == Source::Spell(PARALYSED) {
        let ticks = effect.end.unwrap_or(tick).saturating_sub(tick);
        let cost = u64::from(one.speed) * ticks;
        one.energy
            .spend(tick, one.speed, u32::try_from(cost).unwrap_or(u32::MAX));
    }
    if effect.source == Source::Spell(POISONED) {
        one.spells.poisoner = poisoner;
        one.spells.next_poison = tick + SECOND;
    }
    let spells = &mut one.spells;
    spells.effects.retain(|old| old.source != effect.source);
    spells.effects.push(effect);
    spells.refresh(tick);
    true
}

/// Ends the spells' effects `ends` ends on mob `mob` (dispelling ends a
/// chill too). Whether any were.
pub(crate) fn end_mob_spells(
    world: &mut ServerWorld,
    place: (SiteId, u32),
    mob: Id,
    ends: Ends,
    tick: u64,
) -> bool {
    let Some(one) = find(world, place, mob) else {
        return false;
    };
    let before = one.spells.effects.len();
    (one.spells.effects).retain(|effect| !ends.ends(effect.source));
    let mut ended = one.spells.effects.len() != before;
    if ends == Ends::Spells && one.chilled.is_some() {
        one.chilled = one.chilled.map(|(_, own)| (tick, own));
        one.thaw(tick);
        ended = true;
    }
    one.spells.refresh(tick);
    ended
}

/// Every tick: forgets mobs' spells that have ended, and harms those a
/// spell's poison is due to harm.
pub(crate) fn poison_mobs(world: &mut ServerWorld, tick: u64) {
    let mut due = Vec::new();
    for (&place, population) in &mut world.populations.0 {
        for mob in &mut population.mobs {
            if mob.spells.effects.is_empty() {
                continue;
            }
            mob.spells.refresh(tick);
            if let Some((harm, by, index)) = mob.spells.poison_due(tick) {
                due.push((place, index, mob.id, harm, by));
            }
        }
    }
    for (place, index, mob, harm, by) in due {
        harm_mob(world, (place, index), mob, harm, by, tick);
    }
}

fn find(world: &mut ServerWorld, place: (SiteId, u32), mob: Id) -> Option<&mut Mob> {
    let population = world.populations.0.get_mut(&place)?;
    population.mobs.iter_mut().find(|one| one.id == mob)
}
