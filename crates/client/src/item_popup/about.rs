//! An item's description in its popup: for kinds with one in `item.ftl`
//! (`about-<kind>`, such as `about-dwarven-bread`), a line under its name; a
//! cask's or vessel's says how much it holds too, and whether it is sealed; a
//! yeast's when each in the stack next needs feeding (`yeast.rs`).

use shared::item::{Item, ItemKind};
use shared::lang::{Args, Lang};
use shared::pour::Held;

use crate::common::written;

/// What the language files say of this kind of item, if anything. A stack's
/// is its top's.
pub(super) fn about(item: &Item, lang: &Lang) -> Option<String> {
    let id = about_id(item.top().kind);
    let about = lang
        .has(&id)
        .then(|| written(lang.render(&id, &Args::new())));
    let more = holds(item.top(), lang).or_else(|| super::yeast::yeast_lines(item, lang));
    let Some(holds) = more else {
        return about;
    };
    Some(match about {
        Some(about) => format!("{about}\n{holds}"),
        None => holds,
    })
}

/// How much a cask or vessel holds of how much, and whether it is sealed:
/// "Holds 19.5 L of 20 L, opened".
fn holds(item: &Item, lang: &Lang) -> Option<String> {
    if item.bottle().is_some_and(|bottle| bottle.sealed) {
        return Some(written(lang.render("item-sealed-bottle", &Args::new())));
    }
    let (most, sealed) = match (item.cask(), item.vessel()) {
        (Some(cask), _) => (cask.most_ml(), (!cask.is_empty()).then_some(!cask.opened)),
        (_, Some(vessel)) => (vessel.vessel.most_ml(), Some(vessel.sealed)),
        _ => return None,
    };
    let held = Held::of(item)?;
    let args = Args::new()
        .text("amount", amount(held.ml))
        .text("most", amount(most));
    let holds = written(lang.render("item-holds", &args));
    let state = match sealed {
        Some(true) => "item-sealed",
        Some(false) if held.ml > 0 => "item-opened",
        _ => return Some(holds),
    };
    Some(format!(
        "{holds}, {}",
        written(lang.render(state, &Args::new()))
    ))
}

/// Millilitres as people say them: "250 ml" under a litre, else "19.5 L".
pub(super) fn amount(ml: u32) -> String {
    if ml < 1000 {
        return format!("{ml} ml");
    }
    let litres = format!("{:.2}", f64::from(ml) / 1000.0);
    let litres = litres.trim_end_matches('0').trim_end_matches('.');
    format!("{litres} L")
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
