//! A set item's section in its popup (quality bonuses PR 3): the set's name,
//! every piece under it, grey when the viewer isn't wearing it, then the set
//! bonuses by how many pieces must be worn ("2 worn:"), each worded as the
//! item's own bonuses, grey until that many are worn.

use shared::item::Item;
use shared::item_set::pieces_worn;
use shared::lang::{Args, Lang};

use super::words::worn_line;
use crate::common::written;
use crate::dom::Dom;

/// How far an indented line is moved in, in the interface's pixels.
const INDENT: &str = "16";

/// One line of the section.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct SetLine {
    pub(super) words: String,
    /// Under a heading: a piece, or a bonus under its count.
    pub(super) indented: bool,
    /// Grey: a piece not worn, or a bonus not reached.
    pub(super) off: bool,
}

/// The section for the item (a stack's top), against what the viewer
/// wears; nothing for an item of no set.
pub(super) fn set_lines(item: &Item, worn: &[Item], lang: &Lang) -> Vec<SetLine> {
    let Some(piece) = item.top().set else {
        return Vec::new();
    };
    let set = piece.whole();
    let have = pieces_worn(worn, set.number);
    let line = |words: String, indented, off| SetLine {
        words,
        indented,
        off,
    };
    let mut lines = vec![line(set.name.clone(), false, false)];
    for (at, kind) in set.pieces.iter().enumerate() {
        let off = !have.contains(&(at as u8));
        lines.push(line(kind.def().name.to_owned(), true, off));
    }
    let mut count = None;
    for each in &set.bonuses {
        let off = usize::from(each.worn) > have.len();
        if count != Some(each.worn) {
            count = Some(each.worn);
            let args = Args::new().count("count", i64::from(each.worn));
            lines.push(line(
                written(lang.render("item-set-worn", &args)),
                false,
                off,
            ));
        }
        lines.push(line(worn_line(&each.bonus, lang), true, off));
    }
    lines
}

/// Writes the section into popup `form`'s `set`, a line a copy of
/// `set_line` (or `set_off`, grey), hiding it when there is none.
pub(super) fn show_set(dom: &mut Dom, form: &str, lines: Vec<SetLine>) {
    let into = format!("{form}/set");
    dom.set_flag(&into, "hidden", lines.is_empty());
    dom.empty(&into);
    for (at, line) in lines.into_iter().enumerate() {
        let template = if line.off { "set_off" } else { "set_line" };
        dom.copy_template(&format!("{form}/{template}"), &into, at);
        let id = format!("{form}/{template}.{at}");
        dom.set_text(&id, line.words);
        if line.indented {
            dom.set(&id, "margin-left", INDENT);
        }
    }
}

#[cfg(test)]
mod tests;
