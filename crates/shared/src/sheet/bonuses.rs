//! A sheet with its bonuses: what is worn and its effects added up
//! (`shared::bonus`), the attributes and most Health and Mana they make, and
//! changing effects while keeping Health's and Mana's share.

use super::{Attributes, Sheet};
use crate::bonus::{Bonus, Cancel, Effect, Source, Stat, Totals, eating};
use crate::creature::CreatureKind;
use crate::item::{Item, ItemKind};
use crate::item_set::set_bonuses;
use crate::requirement::effective_bonuses;
use crate::vitals::Limits;

/// What working out a sheet's limits needs besides the sheet itself.
#[derive(Clone, Copy, Debug)]
pub struct Situation<'a> {
    /// What it wears (`Inventory::worn`): mobs and the server's players.
    pub worn: &'a [Item],
    /// Its playtime in action ticks (0 for mobs).
    pub playtime: u64,
    /// Ticks to refill from nothing to full (`Limits::refill`).
    pub refill: u64,
}

impl Sheet {
    /// Every bonus from what is worn and the effects lasting at `now`, added up.
    /// What is worn gives its good bonuses by how well it works for this
    /// creature (`requirement::effective_bonuses`), judged by its attributes
    /// with effects only. Sets' bonuses for the pieces worn are added in full.
    pub fn totals(&self, worn: &[Item], now: u64) -> Totals {
        let own = self.attributes_without_worn(now);
        let mut from_items: Vec<Bonus> = worn
            .iter()
            .flat_map(|item| effective_bonuses(item, &own))
            .collect();
        // Sets' bonuses count in full: no piece asks anything of them.
        from_items.extend(set_bonuses(worn));
        Totals::of(from_items.iter().chain(self.effects.bonuses(now)))
    }

    /// Its attributes with the effects lasting at `now` but nothing worn: what
    /// items' requirements are checked against.
    pub fn attributes_without_worn(&self, now: u64) -> Attributes {
        Totals::of(self.effects.bonuses(now)).attributes(&self.attributes)
    }

    /// Its attributes with their bonuses: what rules should go by.
    pub fn attributes_with(&self, worn: &[Item], now: u64) -> Attributes {
        self.totals(worn, now).attributes(&self.attributes)
    }

    /// The most Health and Mana at `now` with every bonus, and how they come
    /// back. Call `forget_ended` first: an effect ended but not yet forgotten
    /// no longer counts, so what it restored since the last settling is lost.
    pub fn limits(&self, situation: Situation, now: u64) -> Limits {
        self.limits_until(situation, now, self.later.next_start(now))
    }

    /// The limits its player is told of: as `limits`, but rates said to hold
    /// past the start of an effect laid on for later that they don't know of,
    /// so a hidden poison's start can't be read from when restoring ends.
    pub fn told_limits(&self, situation: Situation, now: u64) -> Limits {
        self.limits_until(situation, now, self.later.known().next_start(now))
    }

    /// `limits` with `later` the next start counted.
    fn limits_until(&self, situation: Situation, now: u64, later: Option<u64>) -> Limits {
        let totals = self.totals(situation.worn, now);
        let attributes = totals.attributes(&self.attributes);
        // Rates hold until the next effect ends or one laid on for later starts.
        let until = (self.effects.next_end(now).into_iter()).chain(later).min();
        Limits {
            refill: situation.refill,
            ..self
                .vitals
                .limits_with(&attributes, &totals, situation.playtime, until)
        }
    }

    /// Something worn has changed since its limits were `before`: keeps
    /// Health's and Mana's share (`Vitals::rebase`).
    pub fn rebase(&mut self, before: Limits, situation: Situation, now: u64) {
        let after = self.limits(situation, now);
        self.vitals.rebase(before, after, now);
    }

    /// Changes its effects, keeping Health's and Mana's share. Those ended by
    /// `now` are forgotten first, at their own ends.
    pub fn change_effects<T>(
        &mut self,
        situation: Situation,
        now: u64,
        change: impl FnOnce(&mut crate::bonus::Effects) -> T,
    ) -> T {
        self.forget_ended(situation, now);
        let before = self.limits(situation, now);
        let answer = change(&mut self.effects);
        self.rebase(before, situation, now);
        answer
    }

    /// Lays an effect on, in place of any from the same source.
    pub fn add_effect(&mut self, effect: Effect, situation: Situation, now: u64) {
        self.change_effects(situation, now, |effects| effects.add(effect));
    }

    /// Eats one of these as a creature of kind `eater`, if it may
    /// (`bonus::eating`): healing if it suits it, poison if it poisons it.
    /// Whether it did.
    pub fn eat(
        &mut self,
        kind: ItemKind,
        eater: CreatureKind,
        situation: Situation,
        now: u64,
    ) -> bool {
        let Some(effects) = eating(kind, eater, now) else {
            return false;
        };
        self.change_effects(situation, now, |on| {
            for effect in effects {
                on.add(effect);
            }
        });
        true
    }

    /// Ends the effects this ends (before being hurt, call this first, so
    /// restoring counts up to the harm). Whether any were.
    pub fn cancel_effects(&mut self, cancel: Cancel, situation: Situation, now: u64) -> bool {
        self.forget_ended(situation, now);
        if !self.effects.cancelled_by(cancel) {
            return false;
        }
        self.change_effects(situation, now, |effects| effects.cancel(cancel))
    }

    /// Forgets the effects that have ended by `now`, each settled at its own
    /// end, so restoring stops exactly there, and starts those laid on for
    /// later (`bonus::Later`) at their own starts. Whether any had.
    pub fn forget_ended(&mut self, situation: Situation, now: u64) -> bool {
        let mut any = false;
        while let Some(tick) = (self.effects.first_ended(now).into_iter())
            .chain(self.later.first_due(now))
            .min()
        {
            // Just before, everything ending then still counts and nothing
            // starting then does yet.
            let before = self.limits(situation, tick.saturating_sub(1));
            self.effects.forget_ended(tick);
            for pending in self.later.take_due(tick) {
                self.start_pending(pending.effect);
            }
            self.vitals
                .rebase(before, self.limits(situation, tick), tick);
            any = true;
        }
        any
    }

    /// Older saves kept changes to the most on `Vitals`: each becomes an effect.
    pub(super) fn fold_old_changes(&mut self) {
        for change in std::mem::take(&mut self.vitals.changes) {
            self.effects.0.push(Effect {
                source: Source::Dev,
                bonuses: vec![Bonus::flat(
                    Stat::Most(change.vital),
                    change.amount.saturating_mul(1000),
                )],
                start: 0,
                end: change.until,
                ends_on: crate::bonus::EndsOn::NOTHING,
            });
        }
    }
}

#[cfg(test)]
mod food_tests;
#[cfg(test)]
mod tests;
