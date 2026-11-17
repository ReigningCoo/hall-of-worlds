//! What an item's popup shows: its name in its quality's colour, its
//! description if it has one, anything about where it is, a line for each
//! bonus, the size it was made in when not the viewer's, what it asks of its
//! wearer (what the viewer falls short of as a warning), what a key remembers
//! (with the runes of the planet it was found on, if not this one), its
//! weight, and for staff with debug tools its id with a copy button
//! (`copy.rs`).

use super::about::about;
use super::herb::HerbWords;
use super::needs::{Viewer, need_lines};
use super::open::{FORM, ItemPopups};
use super::size::size_line;
use super::words::bonus_lines;
use crate::common::{NO_QUALITY_NAME, quality_colour};
use crate::dom::{Dom, Template};
use crate::html;
use crate::widgets::{Pointed, TextSize, UiId, weight_words};
use bevy::prelude::*;
use shared::creature::Size;
use shared::id::Id;
use shared::lang::Lang;

/// The popup: `name` is filled with the name in its colour once built; the
/// rest are set as it opens.
pub(super) static ITEM_POPUP: Template = html!(
    "item_popup",
    r#"
<form id="item_popup">
  <popup id="box" hover>
    <canvas id="name" margin-right="20"/>
    <p id="about" hidden>.</p>
    <p id="notes" hidden>.</p>
    <p id="bonuses" hidden>.</p>
    <p id="size" hidden>.</p>
    <p id="needs" hidden>.</p>
    <p id="unmet" class="warning" hidden>.</p>
    <p id="keys" hidden>.</p>
    <img id="planet" hidden/>
    <p id="weight">.</p>
    <span id="id_row" class="columns" nowrap valign="middle" hidden>
      <p id="item_id">.</p>
      <div id="copy_holder" width="28" height="28"/>
    </span>
  </popup>
</form>
"#
);

/// What a key's popup says it remembers (`keys::key_lines`), and the name of
/// the picture of the runes to show under it (`dom::Pictures`), if any.
pub(super) struct KeyWords {
    pub(super) lines: Vec<String>,
    pub(super) runes: Option<String>,
}

/// Places the popup about `of` at `at` and writes what it says, its
/// requirements against the `viewer` (none: all met) and its size against
/// theirs (`yours`); its id too for `staff`.
pub(super) fn fill(
    dom: &mut Dom,
    of: Id,
    pointed: &Pointed,
    at: Vec2,
    (lang, viewer, yours): (&Lang, Option<Viewer>, Size),
    (keys, herb, staff): (KeyWords, Option<HerbWords>, bool),
) {
    let form = UiId::of(FORM, of, "box").form();
    let id = |name: &str| format!("{form}/{name}");
    dom.set(&id("box"), "left", &at.x.to_string());
    dom.set(&id("box"), "top", &at.y.to_string());
    // A herb's part ("Root"), and its effects as far as they are known.
    let (part, effects) = herb.map_or((None, None), |herb| (herb.part, Some(herb.effects)));
    let about = part.or_else(|| about(&pointed.item, lang));
    dom.set_flag(&id("about"), "hidden", about.is_none());
    dom.set_text(&id("about"), about.unwrap_or_default());
    let notes = &pointed.notes;
    dom.set_flag(&id("notes"), "hidden", notes.is_empty());
    dom.set_text(&id("notes"), notes.clone());
    let lines = effects.unwrap_or_else(|| bonus_lines(&pointed.item, lang));
    dom.set_flag(&id("bonuses"), "hidden", lines.is_empty());
    dom.set_text(&id("bonuses"), lines.join("\n"));
    let size = size_line(&pointed.item, yours, lang);
    dom.set_flag(&id("size"), "hidden", size.is_none());
    dom.set_text(&id("size"), size.unwrap_or_default());
    let (met, unmet) = need_lines(&pointed.item, viewer, lang);
    for (name, lines) in [("needs", met), ("unmet", unmet)] {
        dom.set_flag(&id(name), "hidden", lines.is_empty());
        dom.set_text(&id(name), lines.join("\n"));
    }
    dom.set_flag(&id("keys"), "hidden", keys.lines.is_empty());
    dom.set_text(&id("keys"), keys.lines.join("\n"));
    if let Some(runes) = &keys.runes {
        dom.set(&id("planet"), "src", &format!("picture:{runes}"));
    }
    dom.set_flag(&id("planet"), "hidden", keys.runes.is_none());
    let weight = weight_words(pointed.item.total_weight(), None);
    dom.set_text(&id("weight"), weight);
    dom.set_flag(&id("id_row"), "hidden", !staff);
    dom.set_text(&id("item_id"), of.to_string());
}

/// Writes each new popup's name, by its top (a herb's as known: `herb.rs`), in
/// its quality's colour, as Give Item's lists do.
pub(super) fn bind_names(
    new: Query<(Entity, &UiId), Added<UiId>>,
    popups: Res<ItemPopups>,
    mut commands: Commands,
) {
    for (entity, id) in &new {
        if id.parts() != (FORM, "name") {
            continue;
        }
        let open = popups.open.iter().find(|open| Some(open.of) == id.which());
        let Some(open) = open else {
            continue;
        };
        let top = open.item.top();
        let colour = top.quality.map_or(NO_QUALITY_NAME, quality_colour);
        let name = open.name.clone().unwrap_or_else(|| top.name().to_owned());
        let words = (Text::new(name), TextSize::Medium.font());
        commands
            .entity(entity)
            .with_child((words, TextColor(colour)));
    }
}
