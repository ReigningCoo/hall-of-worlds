//! An ointment's lines in its popup: what rubbing one on does
//! (`brewing::skin_effect`), its attributes with how long they last and its
//! Health or Mana at once ("+5 Health at once").

use shared::bonus::Stat;
use shared::brewing::skin_effect;
use shared::item::Item;
use shared::lang::{Args, Lang};

use crate::common::{effect_lines, stat_words, written};

/// One line for each dose a stack's top gives when rubbed on.
pub(super) fn ointment_lines(item: &Item, lang: &Lang) -> Vec<String> {
    let Some(form) = item.top().ointment() else {
        return Vec::new();
    };
    let applied = skin_effect(&form.contents, 0);
    let mut lines: Vec<String> = applied
        .effect
        .iter()
        .flat_map(|effect| effect_lines(effect, lang))
        .collect();
    for (vital, milli) in applied.vitals {
        let sign = if milli < 0 { "\u{2212}" } else { "+" };
        let amount = format!("{sign}{}", milli.unsigned_abs() / 1000);
        let args = Args::new()
            .text("amount", amount)
            .text("stat", stat_words(Stat::Restore(vital), lang));
        lines.push(written(lang.render("item-at-once", &args)));
    }
    if lines.is_empty() {
        lines.push(written(lang.render("item-too-weak", &Args::new())));
    }
    lines
}
