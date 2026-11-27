//! A spell's tip in words (`lang/en/spell-tip.ftl`): its cost, then a line
//! for each step (an effect step's outcomes each on a line of their own,
//! indented), then a note that the numbers are the caster's now, before the
//! target resists. The client shows these lines under the spell's name.

use super::ending::Ends;
use super::filters::Filters;
use super::tip::{Fact, OutcomeTip, Reach, SpellTip, StepTip, WhereTip};
use crate::lang::{Args, Lang};
use crate::skill::Skills;
use crate::vitals::{PER_POINT, Points};

use super::design::Spell;
use super::tip::spell_tip;

/// One line of a tip; `indented` under the step before it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TipLine {
    pub text: String,
    pub indented: bool,
}

/// `spell`'s tip for a caster with `skills`, in `lang`'s language.
pub fn tip_lines(spell: &Spell, skills: Option<&Skills>, lang: &Lang) -> Vec<TipLine> {
    let tip = spell_tip(spell, skills);
    worded(&tip, lang)
}

/// A tip's facts in words.
pub fn worded(tip: &SpellTip, lang: &Lang) -> Vec<TipLine> {
    let line = |text: String, indented| TipLine { text, indented };
    let mut lines = vec![line(
        say(
            lang,
            "spell-tip-cost",
            Args::new().text("mana", whole(tip.cost)),
        ),
        false,
    )];
    for (at, step) in tip.steps.iter().enumerate() {
        let number = at as i64 + 1;
        match step {
            StepTip::Wheres { filters, wheres } => {
                let what = (wheres.iter())
                    .map(|place| where_words(place, lang))
                    .collect::<Vec<_>>()
                    .join(&say(lang, "spell-tip-then", Args::new()));
                let args = Args::new().count("step", number).text("what", what);
                lines.push(line(say(lang, "spell-tip-step", args), false));
                if filters.any() {
                    lines.push(line(filter_words(*filters, lang), true));
                }
            }
            StepTip::Effects(outcomes) => {
                let args = Args::new().count("step", number);
                lines.push(line(say(lang, "spell-tip-effects", args), false));
                for outcome in outcomes {
                    lines.push(line(outcome_words(outcome, lang), true));
                }
            }
        }
    }
    lines.push(line(say(lang, "spell-tip-note", Args::new()), false));
    lines
}

fn where_words(place: &WhereTip, lang: &Lang) -> String {
    let args = Args::new().text("suffix", times(place.count.into(), lang));
    let args = match place.reach {
        Reach::Touch { degrees } => args.count("degrees", i64::from(degrees)),
        Reach::Missile { cells, speed } => args
            .count("cells", i64::from(cells))
            .count("speed", i64::from(speed)),
        Reach::Surround { cells } => args.count("cells", i64::from(cells)),
        Reach::Beam {
            beams,
            cells,
            degrees,
        } => args
            .count("beams", i64::from(beams))
            .count("cells", i64::from(cells))
            .count("degrees", i64::from(degrees)),
        Reach::Point { creatures, cells } => args
            .count("creatures", i64::from(creatures))
            .count("cells", i64::from(cells)),
    };
    say(lang, &format!("spell-tip-{}", place.place.key()), args)
}

fn filter_words(filters: Filters, lang: &Lang) -> String {
    let names = [
        (filters.caster, "self"),
        (filters.friend, "friend"),
        (filters.enemy, "enemy"),
    ];
    let named: Vec<String> = (names.iter())
        .filter(|(on, _)| *on)
        .map(|(_, key)| say(lang, &format!("spell-tip-filter-{key}"), Args::new()))
        .collect();
    let args = Args::new().text("filters", named.join(", "));
    say(lang, "spell-tip-filters", args)
}

fn outcome_words(outcome: &OutcomeTip, lang: &Lang) -> String {
    let noun = say(
        lang,
        &format!("spell-part-{}", outcome.outcome.key()),
        Args::new(),
    );
    let facts: Vec<String> = (outcome.facts.iter())
        .map(|fact| fact_words(*fact, lang))
        .collect();
    let args = Args::new()
        .text("noun", noun)
        .text("suffix", times(outcome.count, lang))
        .text("facts", facts.join("; "));
    let id = if facts.is_empty() {
        "spell-tip-outcome-bare"
    } else {
        "spell-tip-outcome"
    };
    say(lang, id, args)
}

fn fact_words(fact: Fact, lang: &Lang) -> String {
    let seconds = |args: Args, seconds: u64| args.count("seconds", seconds as i64);
    let (id, args) = match fact {
        Fact::Harms(amount) => ("harms", Args::new().text("amount", whole(amount))),
        Fact::Heals(amount) => ("heals", Args::new().text("amount", whole(amount))),
        Fact::Dries => ("dries", Args::new()),
        Fact::LightsFires => ("lights", Args::new()),
        Fact::Douses => ("douses", Args::new()),
        Fact::Wets { seconds: s } => ("wets", seconds(Args::new(), s)),
        Fact::Chills {
            seconds: s,
            percent,
        } => (
            "chills",
            seconds(Args::new(), s).count("percent", i64::from(percent)),
        ),
        Fact::Freezes { seconds: s, cells } => (
            "freezes",
            seconds(Args::new(), s).count("cells", i64::from(cells)),
        ),
        Fact::Clouds { seconds: s, cells } => (
            "clouds",
            seconds(Args::new(), s).count("cells", i64::from(cells)),
        ),
        Fact::Knocks { milli_cells, away } => (
            if away { "knocks" } else { "pulls" },
            Args::new()
                .text("cells", tenths(milli_cells))
                .count("plural", if milli_cells == 1000 { 1 } else { 2 }),
        ),
        Fact::Poisons {
            per_second,
            seconds: s,
        } => (
            "poisons",
            seconds(Args::new(), s).text("amount", tenths(u64::from(per_second.milli()))),
        ),
        Fact::Lasts {
            part,
            amount,
            seconds: s,
        } => (
            "lasts",
            seconds(Args::new(), s)
                .text("part", part.key())
                .count("amount", i64::from(amount.unwrap_or(0).abs()))
                .count("range", super::teleport::TELEPORT_RANGE),
        ),
        Fact::Ends(ends) => ("ends", Args::new().text("ends", ends_key(ends))),
        Fact::Identifies => ("identifies", Args::new()),
        Fact::Conceals => ("conceals", Args::new()),
        Fact::GivesMana(amount) => ("gives-mana", Args::new().text("amount", whole(amount))),
        Fact::DrainsMana(amount) => ("drains-mana", Args::new().text("amount", whole(amount))),
    };
    say(lang, &format!("spell-tip-{id}"), args)
}

/// What a removing outcome ends, as `spell-tip-ends` selects it.
fn ends_key(ends: Ends) -> &'static str {
    match ends {
        Ends::Poison => "poison",
        Ends::Curse => "curse",
        Ends::Illusion => "illusion",
        Ends::Spells => "spells",
        Ends::Levitation => "levitation",
        Ends::Light => "light",
        Ends::Darkness => "darkness",
    }
}

/// " ×3" after a where or outcome stacked more than once; nothing for one.
fn times(count: u32, lang: &Lang) -> String {
    let args = Args::new().count("times", i64::from(count));
    say(lang, "spell-tip-times", args)
}

/// Whole points as a player sees them (rounded up).
fn whole(points: Points) -> String {
    points.shown().to_string()
}

/// Thousandths to one place, the place left off when it is 0 ("1.5", "2").
fn tenths(milli: u64) -> String {
    let tenths = milli.div_ceil(PER_POINT as u64 / 10);
    match tenths % 10 {
        0 => (tenths / 10).to_string(),
        rest => format!("{}.{rest}", tenths / 10),
    }
}

/// A message rendered, or as much of it as could be.
fn say(lang: &Lang, id: &str, args: Args) -> String {
    lang.render(id, &args).unwrap_or_else(|error| error.text)
}
