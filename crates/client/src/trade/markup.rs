//! The trade window as markup (trading plan PRs 4 and 6): who to trade with
//! at the top, the player's side (8 × 8: what is sold or offered), the middle
//! (what each side is worth and who gains, and with a character the Trade
//! toggle), and their side: a shop's whole stock, or a character's 8 × 8,
//! seen but not touched. Along the bottom, for a shop, Complete trade with
//! what is due beside it. Once a trade with a character settles, all that
//! gives way to the receive box, with Take all.

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
    <span id="trading" nowrap valign="top">
      <container id="own_frame" title="Yours">
        <object-grid id="own" type="item" places="packed" stacks="false" columns="8"/>
      </container>
      <div id="middle" width="190" class="tight">
        <p id="own_value" max-width="190">.</p>
        <p id="their_value" max-width="190">.</p>
        <p id="gain" max-width="190">.</p>
        <button id="lock" hidden>Trade</button>
        <p id="their_lock" max-width="190" hidden>They are ready to trade.</p>
      </div>
      <container id="their_frame" title="Theirs">
        <p id="asking" hidden>Asking...</p>
        <scroll id="scroll" max-height="55%">
          <object-grid id="stock" type="item" places="packed" stacks="false" columns="12"/>
        </scroll>
        <object-grid id="theirs" type="item" places="packed" stacks="false" columns="8" hidden/>
      </container>
    </span>
    <container id="received_frame" title="Received" hidden>
      <object-grid id="received" type="item" places="packed" stacks="false" columns="8"/>
      <footer>
        <button id="take_all" class="main">Take all</button>
      </footer>
    </container>
    <p id="said" class="warning" hidden/>
    <span id="bottom" nowrap valign="middle">
      <button id="complete" class="main" disabled>Complete trade</button>
      <p id="due" fills align="right">.</p>
    </span>
  </panel>
</form>
"#
);

/// Asked to trade by another player (their Trading setting Ask): Allow or
/// Deny, a requester over the game; a press outside denies.
pub(super) static ASKED: Template = html!(
    "trade_asked",
    r#"
<form id="trade_asked">
  <dialog id="dialog" title="Trade" depth="1" hidden>
    <p id="who" class="medium" width="360">.</p>
    <footer>
      <button id="deny">Deny</button>
      <button id="allow" class="main">Allow</button>
    </footer>
  </dialog>
</form>
"#
);
