//! The trade window as markup (trading plan PR 4): who to trade with at the
//! top, the player's side (8 × 8, what is being sold), the middle (what each
//! side is worth and who gains), the shop's side (its whole stock), and along
//! the bottom Complete trade with what is due beside it.

use crate::dom::Template;
use crate::html;

/// The own side's size (a test checks the grid says so).
pub(super) const OWN_COLUMNS: u8 = 8;
/// The shop's grid's columns (a test checks).
pub(super) const STOCK_COLUMNS: u8 = 12;

pub(super) static TRADE: Template = html!(
    "trade",
    r#"
<form id="trade">
  <panel id="panel" title="Trade" moveable left="37%" top="12%">
    <span nowrap valign="middle">
      <label for="with" width="60">With</label>
      <select id="with" width="320" disabled>
        <option>Nobody near enough</option>
      </select>
    </span>
    <span nowrap valign="top">
      <container id="own_frame" title="Yours">
        <object-grid id="own" type="item" places="packed" stacks="false" columns="8"/>
      </container>
      <div id="middle" width="190" class="tight">
        <p id="own_value" max-width="190">.</p>
        <p id="their_value" max-width="190">.</p>
        <p id="gain" max-width="190">.</p>
      </div>
      <container id="their_frame" title="Theirs">
        <scroll id="scroll" max-height="55%">
          <object-grid id="stock" type="item" places="packed" stacks="false" columns="12"/>
        </scroll>
      </container>
    </span>
    <p id="said" class="warning" hidden/>
    <span nowrap valign="middle">
      <button id="complete" class="main" disabled>Complete trade</button>
      <p id="due" fills align="right">.</p>
    </span>
  </panel>
</form>
"#
);
