//! A bench's panel as markup: its tools in a grid, unlabelled, with no
//! pictures or tips saying what it takes (owner's), and a line for a
//! refusal; at a bar, the carried vessel to fill and a row for each drink
//! stored in the room, with Pour (`pour.rs`).

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
    <div id="bar" hidden>
      <span nowrap valign="middle">
        <label for="into" width="50">Fill</label>
        <select id="into" width="320">
          <option>No vessel carried</option>
        </select>
      </span>
      <p id="no_drinks" class="faint" width="380">No drinks are stored in this room.</p>
      <scroll id="drinks_scroll" max-height="240" hidden>
        <table id="drinks" class="striped" cellpadding="4"/>
      </scroll>
    </div>
  </panel>
  <template id="drink">
    <tr>
      <td fills><p id="name" width="300"/></td>
      <td><button id="pour">Pour</button></td>
    </tr>
  </template>
</form>
"#
);

/// What a refused tool is told by: never why (owner's).
pub(super) const REFUSED: &str = "The bench won't take that.";
