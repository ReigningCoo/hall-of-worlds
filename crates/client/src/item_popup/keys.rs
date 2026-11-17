//! What a key's popup says it remembers (`item_form::KeyForm`): where it was
//! found ("Found in Ashford (1204, −388)") and the door it has turned in ("Has
//! unlocked a door in Ashford (1204, −388)"), worded through `item.ftl`. A
//! place on another planet says so, and where it was found shows that planet's
//! runes under the lines.

use shared::hall::GateCode;
use shared::item::Item;
use shared::item_form::{ItemForm, KeyFound, KeyPlace};
use shared::lang::{Args, Lang};
use shared::world::WorldId;

use crate::common::written;

/// What a key remembers, as lines, and the runes of the planet it was found
/// on when that is not `here`. Nothing for anything else, or a key that
/// remembers nothing. A stack's is its top's.
pub(super) fn key_lines(
    item: &Item,
    lang: &Lang,
    here: Option<WorldId>,
) -> (Vec<String>, Option<GateCode>) {
    let Some(ItemForm::Key(key)) = &item.top().form else {
        return (Vec::new(), None);
    };
    let mut lines = Vec::new();
    let mut runes = None;
    if let KeyFound::At(place) = &key.found {
        let args = Args::new().text("place", place_words(place, lang, here));
        lines.push(written(lang.render("key-found", &args)));
        if let KeyPlace::Planet {
            world, runes: code, ..
        } = place
            && Some(*world) != here
        {
            runes = *code;
        }
    }
    if let Some(used) = &key.used {
        let did = match (used.locked, used.unlocked) {
            (true, true) => "both",
            (true, false) => "locked",
            _ => "unlocked",
        };
        let args = Args::new()
            .text("did", did)
            .text("place", place_words(&used.at, lang, here));
        lines.push(written(lang.render("key-turned", &args)));
    }
    (lines, runes)
}

/// "in Ashford (1204, −388)", "at (3, 4)", "in the Hall of Worlds", with ",
/// on another planet" when it isn't `here`.
fn place_words(place: &KeyPlace, lang: &Lang, here: Option<WorldId>) -> String {
    let KeyPlace::Planet {
        world, name, x, y, ..
    } = place
    else {
        return written(lang.render("key-place-hall", &Args::new()));
    };
    let args = Args::new().text("x", signed(*x)).text("y", signed(*y));
    let words = match name {
        Some(name) => written(lang.render("key-place-in", &args.text("place", name))),
        None => written(lang.render("key-place-at", &args)),
    };
    if Some(*world) == here {
        return words;
    }
    written(lang.render("key-place-elsewhere", &Args::new().text("place", words)))
}

/// A whole number with a true minus sign: "−388".
fn signed(n: i64) -> String {
    match n < 0 {
        true => format!("−{}", n.unsigned_abs()),
        false => n.to_string(),
    }
}

#[cfg(test)]
mod tests;
