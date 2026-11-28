//! The trade window as markup (trading plan PRs 4 and 6, laid out again by
//! trading tweaks PR 3): who to trade with at the top, then the Selling box
//! (8 × 8: what is sold or offered) on the left with its total in coins under
//! it, the Trade button (or with a character the Trade toggle) between the
//! boxes with the difference under it, and on the right the Buying box (a
//! shop's: 8 wide, what is being bought, dragged there from its stock; or a
//! character's 8 × 8, seen but not touched) with its total, and under it a
//! shop's stock (8 wide, in a scroll). Once a trade with a character settles,
//! all that gives way to the receive box, with Take all. Coins are drawn by
//! `common::show_coins` from the templates at the end, as the item popup's.

use crate::dom::Template;
use crate::html;

/// The own side's size (a test checks the grid says so).
pub(super) const OWN_COLUMNS: u8 = 8;
/// The shop's grid's columns (a test checks).
pub(super) const STOCK_COLUMNS: u8 = 8;

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
      <div id="own_column" class="tight">
        <container id="own_frame" title="Selling">
          <object-grid id="own" type="item" places="packed" stacks="false" columns="8"/>
        </container>
        <span class="columns" nowrap valign="middle">
          <p fills/>
          <span id="own_total" class="columns" nowrap valign="middle" hidden/>
        </span>
      </div>
      <div id="middle" width="150" class="tight" margin-top="112">
        <button id="complete" class="main" disabled>Trade</button>
        <button id="lock" hidden>Trade</button>
        <span class="columns" nowrap valign="middle">
          <p fills/>
          <span id="difference" class="columns" nowrap valign="middle" hidden/>
          <p fills/>
        </span>
        <p id="their_lock" max-width="150" hidden>They are ready to trade.</p>
      </div>
      <div id="their_column" class="tight">
        <container id="their_frame" title="Buying">
          <p id="asking" hidden>Asking...</p>
          <object-grid id="buying" type="item" places="packed" stacks="false" columns="8"/>
          <object-grid id="theirs" type="item" places="packed" stacks="false" columns="8" hidden/>
        </container>
        <span class="columns" nowrap valign="middle">
          <p fills/>
          <span id="their_total" class="columns" nowrap valign="middle" hidden/>
        </span>
        <container id="stock_frame" title="Stock">
          <scroll id="scroll" height="260">
            <object-grid id="stock" type="item" places="packed" stacks="false" columns="8"/>
          </scroll>
        </container>
      </div>
    </span>
    <container id="received_frame" title="Received" hidden>
      <object-grid id="received" type="item" places="packed" stacks="false" columns="8"/>
      <footer>
        <button id="take_all" class="main">Take all</button>
      </footer>
    </container>
    <p id="said" class="warning" hidden/>
    <template id="coin_gold">
      <span id="coin_gold" class="joined" nowrap valign="middle">
        <img class="gold" src="icon:delapouite/token" width="16" height="16"/>
        <p id="gold_count">.</p>
      </span>
    </template>
    <template id="coin_silver">
      <span id="coin_silver" class="joined" nowrap valign="middle">
        <img class="silver" src="icon:delapouite/token" width="16" height="16"/>
        <p id="silver_count">.</p>
      </span>
    </template>
    <template id="coin_copper">
      <span id="coin_copper" class="joined" nowrap valign="middle">
        <img class="copper" src="icon:delapouite/token" width="16" height="16"/>
        <p id="copper_count">.</p>
      </span>
    </template>
    <template id="value_words"><p id="value_words"/></template>
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
