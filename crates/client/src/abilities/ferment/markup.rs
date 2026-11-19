//! Ferment's panel as markup: dropdowns for the cask, what is added, the
//! yeast and how many, what it would make, Ferment and Cancel.

use bevy::prelude::*;

use crate::dom::{Template, document};
use crate::html;

pub(in crate::abilities) static FERMENT: Template = html!(
    "ferment",
    r#"
<form id="ferment">
  <dialog id="asking" title="Ferment" depth="1" hidden>
    <p id="words" class="medium" width="460"/>
    <span nowrap valign="middle">
      <label for="cask" width="90">Cask</label>
      <select id="cask" width="360">
        <option>None</option>
      </select>
    </span>
    <span nowrap valign="middle">
      <label for="add" width="90">Add</label>
      <select id="add" width="360">
        <option>Nothing</option>
      </select>
    </span>
    <span nowrap valign="middle">
      <label for="yeast" width="90">Yeast</label>
      <select id="yeast" width="260">
        <option>None</option>
      </select>
      <select id="count" width="90">
        <option>1</option>
      </select>
    </span>
    <p id="makes" width="460">.</p>
    <footer>
      <button id="ferment">Ferment</button>
      <button id="cancel">Cancel</button>
    </footer>
  </dialog>
</form>
"#
);

/// The words at the top.
pub(super) const PICK: &str = "Choose a cask of juice, or of water with fruit or \
    grain mashed in, and yeast. Each yeast is used once until it is fed again; \
    more of them start a large cask faster.";

pub(in crate::abilities) fn spawn_panel(mut commands: Commands) {
    commands.spawn(document(&FERMENT));
}
