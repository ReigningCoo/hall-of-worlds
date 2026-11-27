//! The spell design panel as markup: a moving panel with the pieces along the
//! top and the steps under them (both canvases `pieces` and `steps` fill),
//! then the picture it would have, its name and what it costs, what is wrong,
//! Save and Clear; and the Save
//! requester asking for a name.

use crate::dom::Template;
use crate::html;

pub(super) static SPELL_DESIGN: Template = html!(
    "spell_design",
    r#"
<form id="spell_design">
  <panel id="panel" title="Spell Design" moveable left="700" top="22">
    <p id="words" class="faint" width="640"/>
    <canvas id="pieces"/>
    <p id="next" class="faint"/>
    <hr/>
    <canvas id="steps"/>
    <span nowrap valign="middle">
      <object id="preview" type="ability" for="spell_design.preview"/>
      <div fills>
        <p id="name"/>
        <p id="cost"/>
      </div>
    </span>
    <p id="problem" width="640"/>
    <span nowrap valign="middle">
      <button id="save" class="main">Save</button>
      <button id="clear">Clear</button>
    </span>
  </panel>
  <dialog id="naming" title="Save spell" depth="1" hidden>
    <p class="medium" width="360">Name it, or leave it empty for the name made from its steps.</p>
    <input id="typed" type="text" maxlength="40"/>
    <footer>
      <button id="really_save" class="main">Save</button>
      <button id="cancel_save">Cancel</button>
    </footer>
  </dialog>
</form>
"#
);

/// The words along the top.
pub(super) const WORDS: &str = "Click a where, filter or effect, or drag it down, to add it \
    to the spell. A step holds wheres or effects, never both; filters go on a first where. \
    To use one more than once, add it again. Drag a where onto a where step to merge it, \
    leading on from the one above. Drag steps, wheres and effects to move them, \
    or off the row (or right-click them) to take them out.";
