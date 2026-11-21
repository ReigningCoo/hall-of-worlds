//! The Cooking panel as markup: a moving panel with the tools along the top,
//! the grid of rows under it (canvases `grid` fills), the kitchen in words,
//! and on the right Start, the count with its buttons, the culture's space,
//! the product's frame and what one run would make.

use crate::dom::Template;
use crate::html;

pub(super) static COOKING: Template = html!(
    "cooking",
    r#"
<form id="cooking">
  <panel id="panel" title="Cooking" moveable left="46" top="22">
    <p id="words" class="faint" width="760"/>
    <canvas id="tools"/>
    <hr/>
    <span nowrap valign="top">
      <div width="520">
        <p id="steps"/>
        <canvas id="grid"/>
        <p id="kitchen" class="faint" width="500"/>
      </div>
      <div width="240" class="tight">
        <button id="start" class="main">Start</button>
        <span nowrap valign="middle">
          <button id="fewest" title="Once">|&lt;</button>
          <button id="fewer" title="One fewer">&lt;</button>
          <p id="count" width="36" align="center">1</p>
          <button id="more" title="One more">&gt;</button>
          <button id="most" title="As many as the ingredients allow">&gt;|</button>
        </span>
        <span nowrap valign="middle">
          <canvas id="culture_space"/>
          <p class="faint" width="190">Culture, for what ferments</p>
        </span>
        <span nowrap valign="middle">
          <object-slot id="product"/>
          <p id="product_name" width="190"/>
        </span>
        <p id="chance"/>
        <p id="about" width="240"/>
      </div>
    </span>
  </panel>
</form>
"#
);

/// The words along the top.
pub(super) const WORDS: &str = "Drag a tool down onto a place in a row, or click it to add \
    it to the top row; drag steps to move them, or off the grid to take them out; click a \
    step to join both rows there. Drag foods, water, milk and honey from the Inventory into \
    the spaces under each step (again for one more); click one to take it out.";
