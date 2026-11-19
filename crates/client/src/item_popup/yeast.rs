//! A yeast's popup lines (drinks PR 4): when each in the stack next needs
//! feeding, the one needing it soonest first, and whether it has been used
//! since it was fed.

use shared::item::{Item, Quality};
use shared::lang::{Args, Lang};
use shared::yeast::{SECONDS_PER_HOUR, YeastForm};

use crate::common::written;

/// Most yeasts listed one by one; the rest are counted.
const LISTED: usize = 5;

/// One line for each yeast in the stack, the one needing feeding soonest
/// first: "Feed in 7 h", "Feed now: weakens in 5 h". `None` for anything else.
pub(super) fn yeast_lines(item: &Item, lang: &Lang) -> Option<String> {
    let mut yeasts: Vec<(&YeastForm, Option<Quality>)> = (item.members())
        .filter_map(|one| Some((one.yeast()?, one.quality)))
        .collect();
    if yeasts.is_empty() {
        return None;
    }
    yeasts.sort_by_key(|(yeast, _)| std::cmp::Reverse(yeast.unfed));
    let mut lines: Vec<String> = (yeasts.iter().take(LISTED))
        .map(|&(yeast, quality)| line(yeast, quality, lang))
        .collect();
    if yeasts.len() > LISTED {
        let args = Args::new().count("count", (yeasts.len() - LISTED) as i64);
        lines.push(written(lang.render("yeast-more", &args)));
    }
    Some(lines.join("\n"))
}

/// One yeast's line.
fn line(yeast: &YeastForm, quality: Option<Quality>, lang: &Lang) -> String {
    let hours = |secs: u32| secs.div_ceil(SECONDS_PER_HOUR).to_string();
    let feed = match yeast.feed_in() {
        0 => {
            let poor = if quality == Some(Quality::Poor) {
                "yes"
            } else {
                "no"
            };
            let args = (Args::new().text("poor", poor)).text("hours", hours(yeast.weakens_in()));
            lang.render("yeast-feed-now", &args)
        }
        secs => lang.render("yeast-feed-in", &Args::new().text("hours", hours(secs))),
    };
    let feed = written(feed);
    match yeast.used {
        true => format!(
            "{feed}, {}",
            written(lang.render("yeast-used", &Args::new()))
        ),
        false => feed,
    }
}
