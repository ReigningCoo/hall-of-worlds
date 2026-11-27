//! Designed spells' rules (magic plan, `docs/context/magic-system.md`): a
//! spell's steps (wheres with filters, effects in order), checking a design
//! against the caster's skills, its Mana cost, Void's reversal, the
//! combination table, automatic casting, automatic names, what each where
//! step hits, and the spellbook. See
//! `docs/context/shared/spell.md`.

mod acts;
mod aim;
mod auto_cast;
mod beam;
mod book;
mod cast;
mod check;
mod combination;
mod condition;
mod cost;
mod design;
mod effect;
mod ending;
mod filters;
mod flight;
mod harm;
mod hit;
mod hits;
mod key;
mod knock;
mod lasting;
mod line;
mod look;
mod mana;
mod missile;
mod name;
mod part;
mod place;
mod point;
mod shown;
mod spare;
mod surround;
mod teleport;
mod tip;
mod tip_words;
mod touch;
mod toward;
mod under;
mod utility;

pub use acts::{
    Acts, ICE_SECONDS, Knock, STEAM_SECONDS, acts, acts_on_place, cloud_cells, frozen_cells,
};
pub use aim::where_reach;
pub use auto_cast::{Aim, AutoCast, auto_cast};
pub use beam::{BEAM_RANGE, BEAM_SPREAD, beam, beam_headings};
pub use book::{
    BookRefusal, BookSpell, LONGEST_NAME, MOST_SPELLS, Spellbook, heal_spell, healing_missile,
    tidy_name,
};
pub use cast::{CAST_COST, CAST_WIND_UP, STEP_MILLIS, heal_restores};
pub use check::{DesignError, check, is_unlocked, unlocked_wheres};
pub use combination::{COMBINATIONS, Combination, Mixture, Outcome, outcomes};
pub use condition::{
    CHILL_SECONDS, CHILL_SPEED_PERCENT, CHILLED, WET, WET_FIRE, WET_SECONDS, chill, condition, wet,
};
pub use cost::{
    EFFECT_BASE, FORM_SAVING, LEAST, effect_cost, effect_step_cost, spell_cost, step_cost,
    where_part_cost, where_step_cost,
};
pub use design::{EffectStep, MAX_STEPS, MAX_WHERES, Spell, Step, WherePart, WhereStep};
pub use effect::Effect;
pub use ending::Ends;
pub use filters::Filters;
pub use flight::{Path, launch, launch_part};
pub use harm::{
    STEAM_HARM, does_harm, effect_power, harm_before_resisting, harm_skills, resistance, resisted,
    spell_power, takes_hold, warding_skill, will,
};
pub use hit::{Creature, Hit, Origin, Side, hit_anyone};
pub use hits::{part_hits, step_hits, where_hits};
pub use key::SpellKey;
pub use knock::{KNOCK_UNITS, MOST_KNOCK, knock_units, knocked_to};
pub use lasting::{
    CURSED, ILLUSION, LASTING_SECONDS, LESS_PERCENT, MORE_PERCENT, MOST_LESS_PERCENT, PARALYSED,
    PARALYSIS_SECONDS, POISON_SECONDS, POISONED, STONESKIN, WARDED, lasting, poison,
    poison_per_second, seconds_of,
};
pub use look::{LOOK_PARTS, Look};
pub use mana::{MANA_GIVEN_PERCENT, mana_drained, mana_given};
pub use missile::{
    Flight, MISSILE_FASTER, MISSILE_RANGE, MISSILE_SPEED, flight_millis, missile, missile_speed,
};
pub use name::auto_name;
pub use part::{Part, parts};
pub use place::Where;
pub use point::{POINT_RANGE, point};
pub use shown::{Shown, shown, shown_part};
pub use spare::spared_harm;
pub use surround::{SURROUND_RING, SURROUND_SPOTS, surround, surround_reach};
pub use teleport::{TELEPORT_RANGE, may_teleport_to};
pub use tip::{Fact, OutcomeTip, Reach, SpellTip, StepTip, WhereTip, facts, reach, spell_tip};
pub use tip_words::{TipLine, tip_lines, worded};
pub use touch::{TOUCH_HALF_ARC, TOUCH_WIDER, touch, touch_half_arc};
pub use toward::Toward;
pub use utility::{
    ANCHORED, BLESSED, CALMED, DARKENED, FRIGHTENED, HASTED, LEVITATING, LIGHT_CELLS, LIT,
    MOST_HASTE_PERCENT, MOST_LIGHT_CELLS, SHORT_SECONDS, SLOWED, TELEPORTING, WEAKENED, is_utility,
    utility_bonuses, utility_seconds,
};

#[cfg(test)]
mod book_tests;
#[cfg(test)]
mod cast_tests;
#[cfg(test)]
mod elements_tests;
#[cfg(test)]
mod flight_tests;
#[cfg(test)]
mod hits_tests;
#[cfg(test)]
mod lasting_tests;
#[cfg(test)]
mod look_tests;
#[cfg(test)]
mod merged_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod utility_tests;
