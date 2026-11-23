//! Harm from spells (magic plan PR 11): how much a harmful outcome does
//! before resisting, the caster's power with its skill, and the target's
//! resistance, their Aura with their Warding skill. Resisting works as
//! Defence does against blows (`combat::past_defence`): power ÷ (power +
//! resistance) of the harm gets through, so equals take half and the weaker
//! side always lands something. The ratio contest's chance, as a share,
//! so harm is never all or nothing. Numbers are first guesses.

use crate::combat::{past_defence, power, skill_percent};
use crate::magic::{HEAL_MANA, RATIO_AT_BEST, RATIO_AT_FIRST};
use crate::rng::Rng;
use crate::sheet::{Attribute, Attributes};
use crate::skill::{SkillId, Skills};
use crate::vitals::{PER_POINT, Points};

use super::combination::{Mixture, Outcome};
use super::effect::Effect;

/// The Warding node, which adds to Aura in resisting harmful magic.
pub fn warding_skill() -> SkillId {
    Effect::Warding.skill()
}

/// Steam's harm for each pair of fire and water, per mille of fire's: half
/// again as much.
pub const STEAM_HARM: u64 = 1500;

/// The skills an outcome's harm leans on: Fire for fire and cold, Healing
/// for harm, Fire and Water for steam; none for what does no harm.
pub fn harm_skills(outcome: Outcome) -> &'static [Effect] {
    let Outcome::Part(part) = outcome else {
        return &[Effect::Fire, Effect::Water];
    };
    match (part.effect, part.reversed) {
        (Effect::Fire, _) => &[Effect::Fire],
        (Effect::Heal, true) => &[Effect::Heal],
        _ => &[],
    }
}

/// Whether an outcome does harm to what it lands on as Health lost.
pub fn does_harm(outcome: Outcome) -> bool {
    !harm_skills(outcome).is_empty()
}

/// How much Health `count` of `outcome` take before resisting: Heal's line
/// (`magic::heal_amount`, 12 to 120 by skill) each, by its own skill (two
/// skills counted as one: the line by each, averaged), steam's half again.
pub fn harm_before_resisting(outcome: Outcome, count: u32, skills: Option<&Skills>) -> Points {
    let leans = harm_skills(outcome);
    if leans.is_empty() {
        return Points::ZERO;
    }
    let each: u64 = (leans.iter())
        .map(|&effect| u64::from(line_by(effect, skills).milli()))
        .sum::<u64>()
        / leans.len() as u64;
    let each = match outcome {
        Outcome::Mixture(Mixture::Steam) => each * STEAM_HARM / 1000,
        Outcome::Part(_) => each,
    };
    Points((each * u64::from(count)).min(u64::from(u32::MAX)) as u32)
}

/// Heal's line by another effect's skill: the same Health one Heal gives at
/// that skill's value.
pub(super) fn line_by(effect: Effect, skills: Option<&Skills>) -> Points {
    let percent = skills.map_or(0, |skills| skill_percent(skills.value(effect.skill())));
    let ratio = RATIO_AT_FIRST + (RATIO_AT_BEST - RATIO_AT_FIRST) * percent / 100;
    let mana = u64::from(HEAL_MANA) * u64::from(PER_POINT);
    Points((mana * ratio / 1000) as u32)
}

/// The caster's power with an outcome's harm: the attributes its skills lean
/// on and their totals (`combat::power`), averaged over its skills.
pub fn spell_power(outcome: Outcome, attributes: &Attributes, skills: Option<&Skills>) -> u64 {
    let leans = harm_skills(outcome);
    if leans.is_empty() {
        return 0;
    }
    let sum: u64 = (leans.iter())
        .map(|&effect| skill_power(effect.skill(), attributes, skills))
        .sum();
    sum / leans.len() as u64
}

/// The caster's power with one effect's skill: what a contest for an effect
/// that isn't harm (Earth's paralysis, a Curse) goes by.
pub fn effect_power(effect: Effect, attributes: &Attributes, skills: Option<&Skills>) -> u64 {
    skill_power(effect.skill(), attributes, skills)
}

/// Power with one skill.
fn skill_power(skill: SkillId, attributes: &Attributes, skills: Option<&Skills>) -> u64 {
    let total = skills.map_or(0, |skills| skills.total(skill));
    power(attributes, skill.leans(), total)
}

/// What a target resists harmful magic with: its Aura and its Warding skill
/// (`combat::power`, so doubling either doubles it).
pub fn resistance(attributes: &Attributes, skills: Option<&Skills>) -> u64 {
    let total = skills.map_or(0, |skills| skills.total(warding_skill()));
    power(attributes, &[Attribute::Aura], total)
}

/// What a target resists effects of the mind with (paralysis, Calm, Fear,
/// illusions): its Will and its Warding skill.
pub fn will(attributes: &Attributes, skills: Option<&Skills>) -> u64 {
    let total = skills.map_or(0, |skills| skills.total(warding_skill()));
    power(attributes, &[Attribute::Will], total)
}

/// What gets past a target's resistance: `power ÷ (power + resistance)` of
/// the harm.
pub fn resisted(harm: Points, power: u64, resistance: u64) -> Points {
    let left = past_defence(u64::from(harm.milli()), power, resistance);
    Points(left.min(u64::from(u32::MAX)) as u32)
}

/// Whether a harmful effect that is not Health lost (ice's chill) takes
/// hold: the ratio contest (`abilities-skills.md`), a roll from 0 to
/// `power + resistance − 1` under `power`.
pub fn takes_hold(power: u64, resistance: u64, rng: &mut Rng) -> bool {
    let power = power.max(1);
    rng.range_u64(0, power.saturating_add(resistance)) < power
}
