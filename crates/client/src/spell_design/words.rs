//! The panel's words: what is wrong with a design, what it costs step by
//! step, and when the next where unlocks.

use shared::skill::{SkillId, Skills, VALUE_SCALE};
use shared::spell::{DesignError, Spell, Where, step_cost};
use shared::vitals::PER_POINT;

use super::piece::where_name;

/// What is wrong, in words.
pub(super) fn problem_words(error: DesignError) -> String {
    match error {
        DesignError::Empty => "Add a where or an effect to begin.".to_owned(),
        DesignError::TooManySteps => {
            format!("A spell has at most {} steps.", shared::spell::MAX_STEPS)
        }
        DesignError::NoEffect => "Add an effect: without one it does nothing.".to_owned(),
        DesignError::WhereAtEnd => "The last step is a where: add an effect after it.".to_owned(),
        DesignError::EmptyStep => "A step holds no effects.".to_owned(),
        DesignError::ZeroCount => "A where or an effect is stacked no times.".to_owned(),
        DesignError::FilterNotFirst => "Filters work only in the first step.".to_owned(),
        DesignError::Locked(place) => format!("{} isn't unlocked yet.", where_name(place)),
    }
}

/// Thousandths of a point as a number of points: whole, or to one place.
fn points(thousandths: u64) -> String {
    let tenths = thousandths.div_ceil(PER_POINT as u64 / 10);
    match tenths % 10 {
        0 => format!("{}", tenths / 10),
        rest => format!("{}.{rest}", tenths / 10),
    }
}

/// What it costs: "Costs 10 Mana (2 × 5)", each step's share when there are
/// several.
pub(super) fn cost_words(spell: &Spell, skills: &Skills, total: u32) -> String {
    let steps: Vec<String> = (spell.steps.iter())
        .map(|step| points(step_cost(step, Some(skills))))
        .collect();
    match steps.len() {
        0 => String::new(),
        1 => format!("Costs {total} Mana."),
        _ => format!("Costs {total} Mana ({}).", steps.join(" × ")),
    }
}

/// Hundredths of a percent as a percentage, to two places.
fn percent(hundredths: u64) -> String {
    let scale = u64::from(VALUE_SCALE);
    format!("{}.{:02}%", hundredths / scale, hundredths % scale)
}

/// When the next where unlocks: at Magic ⌈n³ ÷ 6⌉%, n wheres in all; none
/// once all are.
pub(super) fn next_words(unlocked: usize, skills: &Skills) -> String {
    let Some(next) = Where::ALL.get(unlocked) else {
        return "Every where is unlocked.".to_owned();
    };
    let n = unlocked as u64 + 1;
    let needed = (n * n * n * u64::from(VALUE_SCALE)).div_ceil(6);
    let magic = SkillId::find("magic").map_or(0, |id| u64::from(skills.value(id)));
    format!(
        "{} unlocks at Magic {} (now {}).",
        where_name(*next),
        percent(needed),
        percent(magic)
    )
}
