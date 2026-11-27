//! Bonuses and effects in words (`shared::bonus`, through `item.ftl`): shared
//! by item popups and the effect bar, so both say the same thing.

use shared::bonus::{Amount, Bonus, Effect, SKILL_LEVEL, Stat};
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::lang::{Args, Lang, RenderError};
use shared::skill::SkillId;
use shared::vitals::Vital;

/// A bonus's amount in thousandths, flat or a percentage.
pub fn amount_of(bonus: &Bonus) -> i32 {
    match bonus.amount {
        Amount::Flat(milli) | Amount::Percent(milli) => milli,
    }
}

/// An eaten thing's bonuses, each with how long it lasts and what ends it.
pub fn effect_lines(effect: &Effect, lang: &Lang) -> Vec<String> {
    let ends = match (effect.ends_on.moving, effect.ends_on.hurt) {
        (true, true) => "both",
        (true, false) => "moving",
        (false, true) => "hurt",
        (false, false) => "nothing",
    };
    let ticks = effect.end.map(|end| end.saturating_sub(effect.start));
    let bonuses = effect.bonuses.iter().filter(|bonus| amount_of(bonus) != 0);
    bonuses
        .map(|bonus| {
            let mut line = bonus_line(bonus, lang);
            if let Some(ticks) = ticks {
                let seconds = ticks as f64 / ACTION_TICKS_PER_SECOND;
                let args = Args::new()
                    .text("line", line)
                    .text("seconds", number(seconds));
                line = written(lang.render("item-lasts", &args));
            }
            let args = Args::new().text("line", line).text("ends", ends);
            written(lang.render("item-ends", &args))
        })
        .collect()
}

/// "+40 Toughness", "−8 Agility", "+10% Strength", "Restores 1 Health a second".
pub fn bonus_line(bonus: &Bonus, lang: &Lang) -> String {
    let stat = stat_words(bonus.stat, lang);
    let (id, milli) = match (bonus.stat, bonus.amount) {
        (Stat::Restore(_), Amount::Flat(milli)) => {
            let amount = number(f64::from(milli) / 1000.0);
            let args = Args::new().text("amount", amount).text("stat", stat);
            return written(lang.render("item-restores", &args));
        }
        (Stat::Drain(_), Amount::Flat(milli)) => {
            let args = Args::new()
                .text("amount", signed(-milli))
                .text("stat", stat);
            return written(lang.render("item-drains", &args));
        }
        (Stat::DeadlyDrain(_), Amount::Flat(milli)) => {
            let args = Args::new()
                .text("amount", signed(-milli))
                .text("stat", stat);
            return written(lang.render("item-drains-deadly", &args));
        }
        (Stat::Skill(skill), Amount::Flat(milli)) => return skill_levels(skill, milli, lang),
        (_, Amount::Flat(milli)) => ("item-bonus", milli),
        (_, Amount::Percent(milli)) => ("item-bonus-percent", milli),
    };
    let args = Args::new().text("amount", signed(milli)).text("stat", stat);
    written(lang.render(id, &args))
}

/// A skill raised or lowered by levels, each one percentage point
/// (`bonus::SKILL_LEVEL`): "+1 level in Sword", "−2 levels in Melee".
fn skill_levels(skill: SkillId, milli: i32, lang: &Lang) -> String {
    let levels = i64::from(milli) * 1000 / i64::from(SKILL_LEVEL);
    let levels = levels.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
    let whole = (levels.unsigned_abs() + 500) / 1000;
    let args = Args::new()
        .text("amount", signed(levels))
        .count("count", i64::from(whole))
        .text("skill", skill.name());
    written(lang.render("item-skill-levels", &args))
}

/// What a stat does, worded to suit a bonus either way: "carrying and how
/// hard you hit".
pub fn does_words(stat: Stat, lang: &Lang) -> String {
    let vital = |vital: Vital| match vital {
        Vital::Health => "health",
        Vital::Mana => "mana",
    };
    let id = match stat {
        Stat::Attribute(attribute) => format!("does-{}", attribute.name().to_lowercase()),
        Stat::Most(of) => format!("does-most-{}", vital(of)),
        Stat::Restore(of) | Stat::Drain(of) | Stat::DeadlyDrain(of) => {
            format!("does-restore-{}", vital(of))
        }
        Stat::Darkvision => "does-darkvision".to_owned(),
        Stat::Defence => "does-defence".to_owned(),
        Stat::BlowHarm => "does-blow-harm".to_owned(),
        Stat::HarmTaken => "does-harm-taken".to_owned(),
        Stat::Resistance => "does-resistance".to_owned(),
        Stat::Speed => "does-speed".to_owned(),
        Stat::Light => "does-light".to_owned(),
        Stat::Skill(_) => "does-skill".to_owned(),
    };
    written(lang.render(&id, &Args::new()))
}

/// What a stat is called: "Toughness", "most Mana".
pub fn stat_words(stat: Stat, lang: &Lang) -> String {
    let vital = |vital: Vital| match vital {
        Vital::Health => "health",
        Vital::Mana => "mana",
    };
    let id = match stat {
        Stat::Attribute(attribute) => format!("stat-{}", attribute.name().to_lowercase()),
        Stat::Most(of) => format!("stat-most-{}", vital(of)),
        Stat::Restore(of) | Stat::Drain(of) | Stat::DeadlyDrain(of) => {
            format!("stat-{}", vital(of))
        }
        Stat::Darkvision => "stat-darkvision".to_owned(),
        Stat::Defence => "stat-defence".to_owned(),
        Stat::BlowHarm => "stat-blow-harm".to_owned(),
        Stat::HarmTaken => "stat-harm-taken".to_owned(),
        Stat::Resistance => "stat-resistance".to_owned(),
        Stat::Speed => "stat-speed".to_owned(),
        Stat::Light => "stat-light".to_owned(),
        Stat::Skill(skill) => {
            let args = Args::new().text("skill", skill.name());
            return written(lang.render("stat-skill", &args));
        }
    };
    written(lang.render(&id, &Args::new()))
}

/// Thousandths as a signed number: "+40", "−0.6" (a true minus sign), to one
/// place at most.
pub fn signed(milli: i32) -> String {
    let sign = if milli < 0 { "\u{2212}" } else { "+" };
    format!("{sign}{}", number(f64::from(milli.unsigned_abs()) / 1000.0))
}

/// A number to one place at most, without a needless ".0": "40", "12.3".
pub fn number(value: f64) -> String {
    let tenths = (value * 10.0).round() as i64;
    match tenths % 10 {
        0 => (tenths / 10).to_string(),
        _ => format!("{:.1}", tenths as f64 / 10.0),
    }
}

/// What could be written, any part missing left in braces (and logged).
pub fn written(words: Result<String, RenderError>) -> String {
    words.unwrap_or_else(|error| {
        bevy::log::warn!("Words missing in {}: {:?}", error.id, error.problems);
        error.text
    })
}

#[cfg(test)]
mod tests;
