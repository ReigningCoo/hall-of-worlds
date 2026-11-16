//! The panel as markup: the box describing the problem, then the debug string
//! (selectable, never changed by hand) with the copy button beside it, put in
//! by `panel` as the markup has no copy button; then, for staff allowed it
//! (`load`), a box to paste someone's string into, Load, and their words.

use crate::dom::Template;
use crate::html;

/// The most characters the description may have.
pub(super) const MOST_PROBLEM: usize = 1000;

pub(super) static PANEL: Template = html!(
    "debug_info.panel",
    r#"
<form id="debug_info">
  <panel id="panel" title="Debug Info" moveable left="160" top="90">
    <div width="460">
      <label for="problem">Describe the problem</label>
      <textarea id="problem" rows="5" maxlength="1000"
                placeholder="What happened, and what you expected to happen"/>
      <p>Copy this into your bug report, with what you wrote above:</p>
      <span class="columns" nowrap>
        <textarea id="string" rows="4" width="416" readonly/>
        <div id="copy_holder" width="34" height="34"/>
      </span>
      <div id="load" hidden>
        <hr/>
        <label for="paste">Paste someone's debug string to go there</label>
        <textarea id="paste" rows="3" placeholder="HOW1..."/>
        <span nowrap valign="middle">
          <input id="set_time" type="checkbox" label="Also set time"/>
          <button id="load_button">Load</button>
        </span>
        <p id="load_status"/>
        <div id="reported_box" hidden>
          <label for="reported">Reported problem</label>
          <textarea id="reported" rows="4" readonly/>
        </div>
      </div>
    </div>
  </panel>
</form>
"#
);
