//! The Alchemy panel as markup: a moving panel with the apparatus known along
//! the top, the process row under it (both canvases `rows` fills), and on the
//! right Start, the count with its buttons, the product's frame and what it
//! would make.

use crate::dom::Template;
use crate::html;

pub(super) static ALCHEMY: Template = html!(
    "alchemy",
    r#"
<form id="alchemy">
  <panel id="panel" title="Alchemy" moveable left="46" top="22">
    <p id="words" class="faint" width="700"/>
    <canvas id="apparatus"/>
    <hr/>
    <span id="body" nowrap valign="top">
      <div id="left" width="480">
        <p id="steps"/>
        <p id="at_bench" class="faint" hidden/>
        <canvas id="process"/>
      </div>
      <div width="220" class="tight">
        <button id="start" class="main">Start</button>
        <span nowrap valign="middle">
          <button id="fewest" title="Once">|&lt;</button>
          <button id="fewer" title="One fewer">&lt;</button>
          <p id="count" width="36" align="center">1</p>
          <button id="more" title="One more">&gt;</button>
          <button id="most" title="As many as the ingredients allow">&gt;|</button>
        </span>
        <span nowrap valign="middle">
          <object-slot id="product"/>
          <p id="product_name" width="170"/>
        </span>
        <p id="chance"/>
        <p id="effects" width="220"/>
      </div>
    </span>
  </panel>
</form>
"#
);

/// The words along the top.
pub(super) const WORDS: &str = "Drag an apparatus down to add a step, or click it; drag \
    steps to reorder them, or off the row to take them out. Drag herbs, oils and \
    liquids from the Inventory into the spaces under each step; click one to take \
    it out.";

/// Under the steps while at an Alchemy Bench (`common::BenchHere`).
pub(super) const AT_BENCH: &str = "At the Alchemy Bench: its apparatus are used \
    first, and work using any of them is 10% purer.";

/// Under the top row when no apparatus is known.
pub(super) const NONE_KNOWN: &str = "No apparatus is known yet: pick up a Mortar and \
    Pestle or a Retort to learn its use.";
