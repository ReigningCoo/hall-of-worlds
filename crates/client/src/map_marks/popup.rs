//! The marks' popups on the full map (owner's design): a click or tap on open
//! land opens one with a row of icons over a row of colours (the colour first,
//! remembered; an icon places the mark); one on a mark opens one with Travel
//! Here and Remove.

use bevy::prelude::*;
use shared::id::Id;
use shared::map_mark::{MarkColour, may_add_by_hand};

use crate::common::{MapMarks, Preferences, Wording, mark_name};
use crate::dom::{Dom, Template, document};
use crate::html;

/// How far from where it was opened a popup goes.
const OFFSET: Vec2 = Vec2::new(12.0, 12.0);

/// Placing a mark: the icons, the colours, and what is said when there are
/// too many.
static PLACE: Template = html!(
    "map_marks.place",
    r#"
<form id="place_mark">
  <popup id="box" depth="0">
    <span id="icons" nowrap>
      <button id="icon.flag" title="Flag"><img src="picture:mark.icon.flag"/></button>
      <button id="icon.house" title="House"><img src="picture:mark.icon.house"/></button>
      <button id="icon.cave" title="Cave"><img src="picture:mark.icon.cave"/></button>
      <button id="icon.mine" title="Mine"><img src="picture:mark.icon.mine"/></button>
      <button id="icon.farm" title="Farm"><img src="picture:mark.icon.farm"/></button>
      <button id="icon.orchard" title="Orchard"><img src="picture:mark.icon.orchard"/></button>
      <button id="icon.bag" title="Bag"><img src="picture:mark.icon.bag"/></button>
      <button id="icon.logs" title="Logs"><img src="picture:mark.icon.logs"/></button>
      <button id="icon.camp" title="Camp"><img src="picture:mark.icon.camp"/></button>
      <button id="icon.danger" title="Danger"><img src="picture:mark.icon.danger"/></button>
    </span>
    <p id="full" class="faint" hidden>Too many marks.</p>
    <span nowrap>
      <button id="colour.red" title="Red"><img src="picture:mark.colour.red"/></button>
      <button id="colour.orange" title="Orange"><img src="picture:mark.colour.orange"/></button>
      <button id="colour.yellow" title="Yellow"><img src="picture:mark.colour.yellow"/></button>
      <button id="colour.green" title="Green"><img src="picture:mark.colour.green"/></button>
      <button id="colour.cyan" title="Cyan"><img src="picture:mark.colour.cyan"/></button>
      <button id="colour.blue" title="Blue"><img src="picture:mark.colour.blue"/></button>
      <button id="colour.purple" title="Purple"><img src="picture:mark.colour.purple"/></button>
      <button id="colour.white" title="White"><img src="picture:mark.colour.white"/></button>
    </span>
  </popup>
</form>
"#
);

/// One of the player's marks.
static MARK: Template = html!(
    "map_marks.mark",
    r#"
<form id="mark">
  <popup id="box" depth="0">
    <p id="name">Your mark</p>
    <span nowrap>
      <button id="travel">Travel Here</button>
      <button id="remove" class="danger">Remove</button>
    </span>
  </popup>
</form>
"#
);

/// The popup open, if any.
#[derive(Resource, Default, Clone, Copy, PartialEq, Debug)]
pub enum MarkPopup {
    #[default]
    Closed,
    /// Placing a mark at this cell, opened at `at` (interface pixels).
    Place { cell: (i64, i64), at: Vec2 },
    /// About this mark.
    Mark { id: Id, at: Vec2 },
}

/// Spawns and despawns the popups' documents as `MarkPopup` changes, and
/// says where they are and what they show.
pub(super) fn show_popup(
    popup: Res<MarkPopup>,
    marks: Res<MapMarks>,
    preferences: Res<Preferences>,
    wording: Res<Wording>,
    mut dom: Dom,
    mut commands: Commands,
    mut open: Local<Option<(MarkPopup, Entity)>>,
) {
    let new = open.map(|(shown, _)| shown) != Some(*popup);
    if new {
        if let Some((_, document)) = open.take() {
            commands.entity(document).try_despawn();
        }
        let (template, form, at) = match *popup {
            MarkPopup::Closed => return,
            MarkPopup::Place { at, .. } => (&PLACE, "place_mark", at),
            MarkPopup::Mark { at, .. } => (&MARK, "mark", at),
        };
        *open = Some((*popup, commands.spawn(document(template)).id()));
        let at = at + OFFSET;
        dom.set(&format!("{form}/box"), "left", &at.x.round().to_string());
        dom.set(&format!("{form}/box"), "top", &at.y.round().to_string());
        if let MarkPopup::Mark { id, .. } = *popup
            && let Some(mark) = marks.0.iter().find(|mark| mark.id == id)
        {
            dom.set_text("mark/name", mark_name(&wording.0, mark));
        }
    }
    if matches!(*popup, MarkPopup::Place { .. })
        && (new || preferences.is_changed() || marks.is_changed())
    {
        let full = !may_add_by_hand(&marks.0);
        dom.set_flag("place_mark/icons", "hidden", full);
        dom.set_flag("place_mark/full", "hidden", !full);
        if full {
            dom.set_text("place_mark/full", super::words::too_many(&wording));
        }
        for colour in MarkColour::ALL {
            let id = format!("place_mark/colour.{}", colour.key());
            dom.set_flag(&id, "selected", colour == preferences.mark_colour);
        }
    }
}
