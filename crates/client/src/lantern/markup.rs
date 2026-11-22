//! The fuel panel as markup: the lantern and how much oil it has, and a
//! space to drop a bottle of oil on.

use crate::dom::Template;
use crate::html;

pub(super) static LANTERN: Template = html!(
    "lantern",
    r#"
<form id="lantern">
  <panel id="panel" title="Lantern" moveable>
    <span nowrap valign="middle">
      <object-slot id="lamp"/>
      <div class="tight">
        <p id="oil" width="230"/>
        <p id="left" class="faint" width="230"/>
      </div>
    </span>
    <span nowrap valign="middle">
      <object-slot id="fill"/>
      <p id="fill_words" class="faint" width="230"/>
    </span>
  </panel>
</form>
"#
);
