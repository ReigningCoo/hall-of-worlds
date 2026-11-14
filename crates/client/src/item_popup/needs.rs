//! An item's requirements in its popup (`shared::requirement`): what wearing it
//! asks ("Needs 110 Strength"), split into what the viewer meets and what they
//! don't, with how well it works for them when short ("Working at 49%").

use shared::bonus::Stat;
use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::requirement::{FULL, Requirement, effectiveness, item_requirements};
use shared::sheet::Attributes;

use super::words::{stat_words, written};

/// The lines about what an item asks: those `attributes` meet, then those they
/// don't (shown as a warning), ending with how well it works if not in full.
/// With no viewer (no character yet), every line is met. A stack's are its
/// top's.
pub(super) fn need_lines(
    item: &Item,
    attributes: Option<&Attributes>,
    lang: &Lang,
) -> (Vec<String>, Vec<String>) {
    let item = item.top();
    let requirements = item_requirements(item);
    let (mut met, mut unmet) = (Vec::new(), Vec::new());
    for requirement in &requirements {
        let Requirement::Attribute { attribute, min } = *requirement else {
            continue;
        };
        let args = Args::new()
            .text("min", min.to_string())
            .text("stat", stat_words(Stat::Attribute(attribute), lang));
        let line = written(lang.render("item-needs", &args));
        match attributes {
            Some(with) if !requirement.met_by(with) => unmet.push(line),
            _ => met.push(line),
        }
    }
    if let Some(with) = attributes {
        let factor = effectiveness(&requirements, with);
        if factor < FULL {
            let percent = factor * 100 / FULL;
            let args = Args::new().text("percent", percent.to_string());
            unmet.push(written(lang.render("item-working", &args)));
        }
    }
    (met, unmet)
}

#[cfg(test)]
mod tests;
