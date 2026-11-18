//! The mixing panel as markup: a dialog with the herbs (a checkbox each, from
//! `herb_row`), the oil and doses to choose, what it would make, and the mixes
//! made before (a Use button each, from `recipe_row`).

use bevy::prelude::*;

use crate::dom::{Template, document};
use crate::html;

pub(in crate::abilities) static GRIND: Template = html!(
    "grind",
    r#"
<form id="grind">
  <dialog id="mixing" title="Grind" depth="1" hidden>
    <p id="words" class="medium" width="460"/>
    <scroll id="herb_scroll" max-height="35%">
      <div id="herbs" class="tight"/>
    </scroll>
    <span nowrap valign="middle">
      <label for="oil" width="70">Oil</label>
      <select id="oil" width="300">
        <option>None</option>
      </select>
    </span>
    <span nowrap valign="middle">
      <label for="doses" width="70">Doses</label>
      <select id="doses" width="100">
        <option>1</option>
      </select>
    </span>
    <p id="makes" width="460">.</p>
    <p id="before_title" class="medium" hidden>Made before</p>
    <scroll id="recipe_scroll" max-height="25%" hidden>
      <div id="recipes" class="tight"/>
    </scroll>
    <template id="herb_row">
      <input id="herb" type="checkbox" label="Herb" width="440"/>
    </template>
    <template id="recipe_row">
      <span id="recipe_line" nowrap valign="middle">
        <button id="again">Use</button>
        <small><p id="recipe" width="360">.</p></small>
      </span>
    </template>
    <footer>
      <button id="grind">Grind</button>
      <button id="cancel">Cancel</button>
    </footer>
  </dialog>
</form>
"#
);

/// The words above the herbs.
pub(super) const PICK: &str = "Tick one of each herb to grind with an oil. Only one \
    of each kind counts; more doses use more of each.";
pub(super) const NO_HERBS: &str = "No herbs are carried. Grinding takes herbs and a \
    bottle of oil, ground together in a mortar and pestle.";

pub(in crate::abilities) fn spawn_mixing(mut commands: Commands) {
    commands.spawn(document(&GRIND));
}
