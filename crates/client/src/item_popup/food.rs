//! A cooked food's popup lines (cooking plan PR 6): what it is mostly made
//! with, its diet labels, and an intermediate's rest: how long until it turns
//! or proves, wild or cultured.

use shared::cooking::{Ingredients, Portion, food_name};
use shared::food::{Curve, RestClock};
use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::yeast::SECONDS_PER_HOUR;

use crate::common::written;

/// The most foods named in "Made with".
const NAMED: usize = 4;

/// Its lines, or `None` for anything not cooked.
pub(super) fn food_lines(item: &Item, lang: &Lang) -> Option<String> {
    let food = item.top().food()?;
    let mut foods: Vec<(u32, String)> = (food.portions.iter())
        .filter_map(|portion| match portion {
            Portion::Food {
                source, key, grams, ..
            } => Some((*grams, food_name(*source, key))),
            Portion::Water { .. } => None,
        })
        .collect();
    foods.sort_by_key(|food| std::cmp::Reverse(food.0));
    let mut names: Vec<String> = Vec::new();
    for (_, name) in foods {
        if !names.contains(&name) && names.len() < NAMED {
            names.push(name);
        }
    }
    let mut lines = Vec::new();
    if !names.is_empty() {
        let args = Args::new().text("foods", names.join(", "));
        lines.push(written(lang.render("food-made-with", &args)));
    }
    let labels = food.diet(Ingredients::built_in()).labels();
    if !labels.is_empty() {
        let args = Args::new().text("labels", labels.join(", "));
        lines.push(written(lang.render("food-diet", &args)));
    }
    lines.extend(food.rest.as_ref().and_then(|rest| rest_line(rest, lang)));
    Some(lines.join("\n"))
}

/// How an intermediate is resting; `None` for a stable one.
fn rest_line(rest: &RestClock, lang: &Lang) -> Option<String> {
    let hours = |secs: u32| secs.div_ceil(SECONDS_PER_HOUR).to_string();
    let secs = rest.seconds();
    let line = match rest.kind.curve() {
        Curve::Stable => return None,
        Curve::Ages { every, most } => {
            let steps = (secs / every.max(1)).min(most);
            if steps >= most {
                written(lang.render("food-aged", &Args::new()))
            } else {
                let left = every - secs % every.max(1);
                let args = Args::new()
                    .count("steps", i64::from(steps))
                    .text("hours", hours(left));
                written(lang.render("food-ageing", &args))
            }
        }
        Curve::Turns { into, .. } => {
            let left = rest.turns_in().unwrap_or(0);
            let args = Args::new()
                .text("into", into.name())
                .text("hours", hours(left));
            let turns = written(lang.render("food-turns", &args));
            let how = match (rest.kind.culture(), rest.culture) {
                (None, _) => return Some(turns),
                (Some(_), None) => "food-wild",
                (Some(_), Some(_)) => "food-cultured",
            };
            format!("{turns}, {}", written(lang.render(how, &Args::new())))
        }
        Curve::Proves {
            better_from,
            best_until,
            worse_from,
            ..
        } => {
            let (id, until) = match secs {
                s if s < better_from => ("food-proving", better_from - s),
                s if s < best_until => ("food-proved", best_until - s),
                s if s < worse_from => ("food-resting", worse_from - s),
                _ => ("food-over-proved", 0),
            };
            let args = Args::new().text("hours", hours(until));
            written(lang.render(id, &args))
        }
    };
    Some(line)
}
