//! The talk panel as markup: one of several alike, by building
//! (`talk#<key>/...`). Its title names who is behind the door. The quests
//! page is a list of what the giver has, or one quest in full with its
//! buttons; it is written out by `quests_page`. A shop's stock is the trade
//! window's (`trade`).

use crate::dom::Template;
use crate::html;

pub(super) static TALK: Template = html!(
    "talk",
    r#"
<form id="talk">
  <panel id="panel" moveable>
    <div id="quests_page" width="460">
      <div id="list" class="tight"/>
      <div id="detail" hidden>
        <h2 id="title"/>
        <div id="paras"/>
        <p id="steps_head"/>
        <div id="steps" class="tight"/>
        <p id="reward"/>
        <span>
          <button id="accept" class="main">Accept</button>
          <button id="turn_in" class="main">Complete</button>
          <button id="enter" class="main" hidden/>
          <button id="decline">Decline</button>
          <button id="back">Back</button>
        </span>
      </div>
      <p id="refused" class="warning" hidden/>
    </div>
    <template id="offer"><button id="offer" class="quiet"/></template>
    <template id="hint"><p id="hint" class="faint"/></template>
    <template id="para"><p id="para"/></template>
    <template id="step"><p id="step"/></template>
  </panel>
</form>
"#
);
