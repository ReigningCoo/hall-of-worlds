//! A bench's panel as markup: its tools in a grid, unlabelled, with no
//! pictures or tips saying what it takes (owner's), and a line for a
//! refusal.

use crate::dom::Template;
use crate::html;

/// The grid's columns are `COLUMNS` (a test checks).
pub(super) static BENCH: Template = html!(
    "bench",
    r#"
<form id="bench">
  <panel id="panel" moveable>
    <object-grid id="tools" type="item" places="packed" stacks="false" columns="6"/>
    <p id="refused" class="warning" hidden/>
  </panel>
</form>
"#
);

/// What a refused tool is told by: never why (owner's).
pub(super) const REFUSED: &str = "The bench won't take that.";
