//! An item's description in its popup: for kinds with one in `item.ftl`
//! (`about-<kind>`, such as `about-dwarven-bread`), a line under its name; a
//! cask's says how many litres it holds too.

use shared::item::{Item, ItemKind};
use shared::lang::{Args, Lang};

use crate::common::written;

/// What the language files say of this kind of item, if anything. A stack's
/// is its top's.
pub(super) fn about(item: &Item, lang: &Lang) -> Option<String> {
    let id = about_id(item.top().kind);
    let about = lang
        .has(&id)
        .then(|| written(lang.render(&id, &Args::new())));
    let Some(cask) = item.top().cask() else {
        return about;
    };
    let args = Args::new()
        .count("litres", i64::from(cask.litres))
        .count("most", i64::from(cask.size.litres()));
    let holds = written(lang.render("item-cask-holds", &args));
    Some(match about {
        Some(about) => format!("{about}\n{holds}"),
        None => holds,
    })
}

/// `about-` and the kind's name in kebab case: `about-dwarven-bread`.
pub(super) fn about_id(kind: ItemKind) -> String {
    let mut id = String::from("about");
    for letter in format!("{kind:?}").chars() {
        if letter.is_uppercase() {
            id.push('-');
        }
        id.push(letter.to_ascii_lowercase());
    }
    id
}

#[cfg(test)]
mod tests;
