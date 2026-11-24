//! What spells leave on creatures for a while (magic plan PR 11), as timed
//! effects (`bonus::Source::Spell`): Water's wet (fire does half to the wet,
//! and dries them) and ice's chill (a third slower). Numbers are first
//! guesses.

use crate::bonus::{Effect as Timed, EndsOn, Source, Stat, Totals};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::sheet::Sheet;

use super::effect::Effect;
use super::part::Part;

/// Seconds a Water keeps someone wet, each.
pub const WET_SECONDS: u64 = 30;
/// Seconds ice's chill lasts, each.
pub const CHILL_SECONDS: u64 = 10;
/// How fast someone chilled goes, in percent: a third slower.
pub const CHILL_SPEED_PERCENT: u32 = 67;
/// Fire's harm to someone wet, per mille: half.
pub const WET_FIRE: u64 = 500;

/// Being wet.
pub const WET: Part = Part::plain(Effect::Water);
/// Being chilled by ice.
pub const CHILLED: Part = Part::reversed(Effect::Water);

/// `part` for `seconds` × `count` from `now` (an action tick).
pub fn condition(part: Part, seconds: u64, count: u32, now: u64) -> Timed {
    let ticks = seconds * u64::from(count) * ACTION_TICKS_PER_SECOND as u64;
    Timed {
        source: Source::Spell(part),
        bonuses: Vec::new(),
        start: now,
        end: Some(now + ticks),
        ends_on: EndsOn::NOTHING,
    }
}

/// `count` Waters' wet, from `now`.
pub fn wet(count: u32, now: u64) -> Timed {
    condition(WET, WET_SECONDS, count, now)
}

/// `count` ices' chill, from `now`.
pub fn chill(count: u32, now: u64) -> Timed {
    condition(CHILLED, CHILL_SECONDS, count, now)
}

impl Sheet {
    /// Whether a spell's `part` lies on it now (at `now`).
    pub fn under_spell(&self, part: Part, now: u64) -> bool {
        (self.effects.0.iter()).any(|e| e.source == Source::Spell(part) && e.lasts(now))
    }

    /// Ends a spell's `part` on it (fire drying the wet). Whether one was.
    pub fn end_spell(&mut self, part: Part) -> bool {
        let before = self.effects.0.len();
        (self.effects.0).retain(|effect| effect.source != Source::Spell(part));
        self.effects.0.len() != before
    }

    /// How fast spells let it go, in percent: chilled a third slower, then
    /// Haste's more and Slow's less (`Stat::Speed`, PR 13), never below 1.
    pub fn spell_speed_percent(&self) -> u32 {
        let chilled = (self.effects.0.iter()).any(|e| e.source == Source::Spell(CHILLED));
        let chill = if chilled { CHILL_SPEED_PERCENT } else { 100 };
        let spells = self
            .effects
            .0
            .iter()
            .filter(|e| matches!(e.source, Source::Spell(_)));
        let totals = Totals::of(spells.flat_map(|e| &e.bonuses));
        let paced = u64::from(chill) * totals.per_mille(Stat::Speed) / 1000;
        paced.clamp(1, u64::from(u32::MAX)) as u32
    }
}
