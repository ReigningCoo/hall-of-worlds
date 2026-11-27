//! What a spell is, for its tip (owner, 2026-11-27): its cost and each step
//! with numbers, for the caster as they are now (their skills) and before the
//! target resists, as the caster can't know how a target will take it. Facts
//! only, from the same functions casting uses; `tip_words` words them.

use crate::skill::Skills;
use crate::spot::CELL_SPAN;
use crate::vitals::Points;

use super::acts::{ICE_SECONDS, STEAM_SECONDS, acts};
use super::beam::{BEAM_RANGE, BEAM_SPREAD};
use super::cast::heal_restores;
use super::combination::{Outcome, outcomes};
use super::condition::{CHILL_SECONDS, CHILL_SPEED_PERCENT, WET_SECONDS};
use super::cost::spell_cost;
use super::design::{EffectStep, Spell, Step, WhereStep};
use super::effect::Effect;
use super::ending::Ends;
use super::filters::Filters;
use super::harm::harm_before_resisting;
use super::knock::knock_units;
use super::lasting::{POISONED, lasting, poison_per_second, seconds_of};
use super::mana::{mana_drained, mana_given};
use super::missile::{MISSILE_RANGE, missile_speed};
use super::part::Part;
use super::place::Where;
use super::point::POINT_RANGE;
use super::surround::SURROUND_RING;
use super::touch::touch_half_arc;
use crate::bonus::Amount;
use crate::facing::TURN;

/// The most cells ice freezes or steam clouds round where it lands
/// (`acts::frozen_cells`, `cloud_cells`).
const MOST_CELLS: u32 = 5;

/// A spell's tip: its cost and its steps in order.
#[derive(Clone, Debug, PartialEq)]
pub struct SpellTip {
    pub cost: Points,
    pub steps: Vec<StepTip>,
}

/// One step: its wheres (with the first step's filters), or its effects.
#[derive(Clone, Debug, PartialEq)]
pub enum StepTip {
    Wheres {
        filters: Filters,
        wheres: Vec<WhereTip>,
    },
    Effects(Vec<OutcomeTip>),
}

/// One where in a step, stacked `count` times, and how far it reaches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WhereTip {
    pub place: Where,
    pub count: u8,
    pub reach: Reach,
}

/// How far a where reaches (cells are whole: the ranges are).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    /// Arm's reach, in an arc this many degrees wide.
    Touch { degrees: u32 },
    /// Up to `cells`, flying `speed` cells a second.
    Missile { cells: u32, speed: u32 },
    /// Everyone within `cells` round its centre.
    Surround { cells: u32 },
    /// `beams` beams of `cells`, `degrees` apart.
    Beam {
        beams: u32,
        cells: u32,
        degrees: u32,
    },
    /// `creatures` within `cells`.
    Point { creatures: u32, cells: u32 },
}

/// One outcome of an effect step (Void read), with how many and what it does.
#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeTip {
    pub outcome: Outcome,
    pub count: u32,
    pub facts: Vec<Fact>,
}

/// One thing an outcome does, with its numbers. Amounts of Health or Mana in
/// `Points`; distances in thousandths of a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fact {
    /// Health lost, before resisting.
    Harms(Points),
    /// Health restored.
    Heals(Points),
    /// Dries the wet, who take half the harm.
    Dries,
    /// Lights fireplaces with kindling.
    LightsFires,
    /// Puts out fires.
    Douses,
    /// Wet for so long.
    Wets { seconds: u64 },
    /// Slowed to `percent` speed for so long, if it takes hold.
    Chills { seconds: u64, percent: u32 },
    /// Water within `cells` frozen for so long.
    Freezes { seconds: u64, cells: u32 },
    /// A cloud within `cells` for so long.
    Clouds { seconds: u64, cells: u32 },
    /// Knocked away (or pulled in) this far.
    Knocks { milli_cells: u64, away: bool },
    /// Health lost each second for so long, before resisting.
    Poisons { per_second: Points, seconds: u64 },
    /// A lasting effect for so long: its percent (or cells, for Light) if it
    /// has one.
    Lasts {
        part: Part,
        amount: Option<i32>,
        seconds: u64,
    },
    /// Ends these effects.
    Ends(Ends),
    /// Finds out what is secret about what is worn, and secret doors.
    Identifies,
    /// Hides it again.
    Conceals,
    /// Mana given, shared between those it lands on.
    GivesMana(Points),
    /// Mana drained, before resisting.
    DrainsMana(Points),
}

/// What `spell` is, for a caster with `skills`.
pub fn spell_tip(spell: &Spell, skills: Option<&Skills>) -> SpellTip {
    let cost = spell_cost(spell, skills);
    let steps = (spell.steps.iter())
        .map(|step| match step {
            Step::Where(step) => where_tip(step),
            Step::Effects(step) => StepTip::Effects(effect_tips(step, cost, skills)),
        })
        .collect();
    SpellTip { cost, steps }
}

fn where_tip(step: &WhereStep) -> StepTip {
    let wheres = (step.parts())
        .map(|part| WhereTip {
            place: part.place,
            count: part.count,
            reach: reach(part.place, part.count),
        })
        .collect();
    StepTip::Wheres {
        filters: step.filters,
        wheres,
    }
}

/// How far a where ×`count` reaches.
pub fn reach(place: Where, count: u8) -> Reach {
    let cells = |units: u64| (units / CELL_SPAN as u64) as u32;
    let degrees = |angle: u32| (angle * 360).div_ceil(TURN);
    match place {
        Where::Touch => Reach::Touch {
            degrees: degrees(u32::from(touch_half_arc(count)) * 2),
        },
        Where::Missile => Reach::Missile {
            cells: cells(MISSILE_RANGE),
            speed: cells(missile_speed(count)),
        },
        Where::Surround => Reach::Surround {
            cells: cells(SURROUND_RING * u64::from(count.max(1))),
        },
        Where::Beam => Reach::Beam {
            beams: u32::from(count.max(1)),
            cells: cells(BEAM_RANGE),
            degrees: degrees(u32::from(BEAM_SPREAD)),
        },
        Where::Point => Reach::Point {
            creatures: u32::from(count.max(1)),
            cells: cells(POINT_RANGE),
        },
    }
}

fn effect_tips(step: &EffectStep, cost: Points, skills: Option<&Skills>) -> Vec<OutcomeTip> {
    (outcomes(step).into_iter())
        .map(|(outcome, count)| OutcomeTip {
            outcome,
            count,
            facts: facts(outcome, count, cost, skills),
        })
        .collect()
}

/// What `count` of `outcome` do, by a caster with `skills`, in a spell
/// costing `cost`.
pub fn facts(outcome: Outcome, count: u32, cost: Points, skills: Option<&Skills>) -> Vec<Fact> {
    let does = acts(outcome);
    let mut facts = Vec::new();
    if outcome == Outcome::Part(Part::plain(Effect::Heal)) {
        facts.push(Fact::Heals(heal_restores(count, skills)));
    }
    if does.harms {
        facts.push(Fact::Harms(harm_before_resisting(outcome, count, skills)));
    }
    let seconds = |each: u64| each * u64::from(count);
    let cells = count.min(MOST_CELLS);
    let flags = [
        (does.dries, Fact::Dries),
        (does.lights, Fact::LightsFires),
        (does.douses, Fact::Douses),
        (
            does.wets,
            Fact::Wets {
                seconds: seconds(WET_SECONDS),
            },
        ),
        (
            does.chills,
            Fact::Chills {
                seconds: seconds(CHILL_SECONDS),
                percent: CHILL_SPEED_PERCENT,
            },
        ),
        (
            does.freezes,
            Fact::Freezes {
                seconds: seconds(ICE_SECONDS),
                cells,
            },
        ),
        (
            does.clouds,
            Fact::Clouds {
                seconds: seconds(STEAM_SECONDS),
                cells,
            },
        ),
        (does.identifies, Fact::Identifies),
        (does.conceals, Fact::Conceals),
    ];
    facts.extend(
        flags
            .into_iter()
            .filter(|(on, _)| *on)
            .map(|(_, fact)| fact),
    );
    if let Some(knock) = does.knocks {
        facts.push(Fact::Knocks {
            milli_cells: knock_units(count, skills) * 1000 / CELL_SPAN as u64,
            away: knock == super::acts::Knock::Away,
        });
    }
    match does.lasts {
        Some(POISONED) => facts.push(Fact::Poisons {
            per_second: poison_per_second(skills),
            seconds: seconds_of(POISONED, count),
        }),
        Some(part) => facts.push(Fact::Lasts {
            part,
            amount: amount_of(part, count, skills),
            seconds: seconds_of(part, count),
        }),
        None => {}
    }
    facts.extend(does.ends.map(Fact::Ends));
    if does.gives_mana {
        facts.push(Fact::GivesMana(mana_given(cost, 1, skills)));
    }
    if does.drains_mana {
        facts.push(Fact::DrainsMana(mana_drained(count, skills)));
    }
    facts
}

/// A lasting part's first bonus as a whole number: a percent, or Light's
/// cells; none for one with no bonus (paralysis, Calm, Teleport).
fn amount_of(part: Part, count: u32, skills: Option<&Skills>) -> Option<i32> {
    let timed = lasting(part, count, skills, 0)?;
    let first = timed.bonuses.first()?;
    Some(match first.amount {
        Amount::Percent(milli) | Amount::Flat(milli) => milli / 1000,
    })
}

#[cfg(test)]
mod tests;
