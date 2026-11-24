//! Spells lying on mobs (magic plan PR 12), kept as players keep them
//! (`bonus::Effect`, `Source::Spell`) but not saved: a mob unloaded is free
//! of them. What they do to blows (`combat::Guard`) and to resisting magic
//! is worked out again whenever they change; a spell's poison harms it once
//! a second, as from whoever cast it, so a poisoned mob can die of it.
//! Paralysis is the energy it would have gained meanwhile, spent at once.
//! PR 13: Haste and Slow change its pace (`chill.rs`), Blessing and Weakness
//! its attributes in a fight, a Calm keeps it from being unfriendly to anyone
//! (`feeling.rs`), and Fear sends it fleeing whoever cast it.

use shared::bonus::{Effect, Source, Stat, Totals};
use shared::combat::Guard;
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::id::Id;
use shared::sheet::Attributes;
use shared::spell::{
    ANCHORED, BLESSED, CALMED, DARKENED, Ends, FRIGHTENED, LEVITATING, LIT, PARALYSED, POISONED,
    TELEPORTING, WEAKENED,
};
use shared::vitals::{Points, Vital};

use super::harmed::{Harmer, harm_mob};
use super::{Doing, Mob};
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
    /// Its pace per mille (Haste, Slow).
    pace: u64,
    /// Its attributes with a Blessing's or Weakness's, if one lies on it.
    attributes: Option<Attributes>,
}

impl Default for MobSpells {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
            guard: Guard::NONE,
            resisting: 1000,
            poisoner: None,
            next_poison: 0,
            pace: 1000,
            attributes: None,
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

    /// Its pace per mille (Haste, Slow): 1000 with neither.
    pub(crate) fn pace(&self) -> u64 {
        self.pace
    }

    /// Its attributes with a Blessing's or Weakness's, if one lies on it.
    pub(crate) fn attributes(&self) -> Option<&Attributes> {
        self.attributes.as_ref()
    }

    /// Whether a spell's `part` lies on it.
    pub(crate) fn has(&self, part: shared::spell::Part) -> bool {
        (self.effects.iter()).any(|e| e.source == Source::Spell(part))
    }

    fn refresh(&mut self, tick: u64, base: &Attributes) {
        self.effects.retain(|effect| effect.lasts(tick));
        let totals = Totals::of(self.effects.iter().flat_map(|e| &e.bonuses));
        self.guard = Guard::of(&totals);
        self.resisting = totals.per_mille(Stat::Resistance);
        self.pace = totals.per_mille(Stat::Speed);
        let blessed = self.has(BLESSED) || self.has(WEAKENED);
        self.attributes = blessed.then(|| totals.attributes(base));
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

/// What lying on a mob does nothing to: light, floating and teleporting are
/// for players alone so far.
const PLAYERS_ONLY: [shared::spell::Part; 5] = [LIT, DARKENED, LEVITATING, TELEPORTING, ANCHORED];

/// A spell's `effect`, cast by `by` on their level `index`, lies on mob `mob`
/// on `place` from `tick`, in place of one from the same source; a poison
/// counts as theirs, harming from a second on. Paralysis spends the energy it
/// lasts; a Calm stops it fighting, Fear sends it fleeing them. Whether it
/// took (there, and something that changes a mob).
pub(crate) fn lay_on_mob(
    world: &mut ServerWorld,
    place: (SiteId, u32),
    mob: Id,
    effect: Effect,
    (by, index): (Harmer, u32),
    tick: u64,
) -> bool {
    if let Source::Spell(part) = effect.source
        && PLAYERS_ONLY.contains(&part)
    {
        return false;
    }
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
        one.spells.poisoner = Some((by, index));
        one.spells.next_poison = tick + SECOND;
    }
    if effect.source == Source::Spell(CALMED) && one.doing.target().is_some() {
        one.doing = Doing::Idle;
    }
    if effect.source == Source::Spell(FRIGHTENED) {
        one.doing = Doing::Fleeing {
            from: by.id,
            struck: false,
        };
    }
    let spells = &mut one.spells;
    spells.effects.retain(|old| old.source != effect.source);
    spells.effects.push(effect);
    one.refresh_spells(tick);
    true
}

impl Mob {
    /// Works out what its spells do again at `tick`, its pace too.
    fn refresh_spells(&mut self, tick: u64) {
        self.spells.refresh(tick, &self.attributes);
        self.repace(tick);
    }
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
        one.chilled = None;
        ended = true;
    }
    one.refresh_spells(tick);
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
            mob.refresh_spells(tick);
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
