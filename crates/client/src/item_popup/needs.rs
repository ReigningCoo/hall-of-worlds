//! An item's requirements in its popup (`shared::requirement`): what wearing it
//! asks ("Needs 110 Strength") and who it is for ("Dwarves only"), split into
//! what the viewer meets and what they don't, with how well it works for them
//! when short ("Working at 49%").

use shared::bonus::Stat;
use shared::creature::CreatureKind;
use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::requirement::{FULL, Requirement, effectiveness, item_requirements};
use shared::sheet::Attributes;

use crate::common::kinds_words;
use crate::common::{stat_words, written};

/// Who is looking at an item: the player's attributes with effects, and the
/// kind of creature they are.
#[derive(Clone, Copy, Debug)]
pub(super) struct Viewer<'a> {
    pub attributes: &'a Attributes,
    pub kind: CreatureKind,
}

/// The lines about what an item asks: those the viewer meets, then those they
/// don't (shown as a warning), ending with how well it works if not in full.
/// With no viewer (no character yet), every line is met. A stack's are its
/// top's.
pub(super) fn need_lines(
    item: &Item,
    viewer: Option<Viewer>,
    lang: &Lang,
) -> (Vec<String>, Vec<String>) {
    let item = item.top();
    let requirements = item_requirements(item);
    let (mut met, mut unmet) = (Vec::new(), Vec::new());
    for requirement in &requirements {
        let line = match requirement {
            Requirement::Attribute { attribute, min } => {
                let args = Args::new()
                    .text("min", min.to_string())
                    .text("stat", stat_words(Stat::Attribute(*attribute), lang));
                written(lang.render("item-needs", &args))
            }
            Requirement::Kinds(kinds) => kinds_line(kinds, lang),
            Requirement::Skill { .. } => continue,
        };
        let meets = |viewer: Viewer| {
            requirement.met_by(viewer.attributes) && requirement.suits(viewer.kind)
        };
        match viewer {
            Some(viewer) if !meets(viewer) => unmet.push(line),
            _ => met.push(line),
        }
    }
    if let Some(with) = viewer.map(|viewer| viewer.attributes) {
        let factor = effectiveness(&requirements, with);
        if factor < FULL {
            let percent = factor * 100 / FULL;
            let args = Args::new().text("percent", percent.to_string());
            unmet.push(written(lang.render("item-working", &args)));
        }
    }
    (met, unmet)
}

/// "Dwarves only", "Orcs and goblins only".
fn kinds_line(kinds: &[CreatureKind], lang: &Lang) -> String {
    let kinds = lang.rules().capitalise(&kinds_words(kinds, lang));
    written(lang.render("item-kinds-only", &Args::new().text("kinds", kinds)))
}

#[cfg(test)]
mod tests;
