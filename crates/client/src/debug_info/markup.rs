//! The panel as markup: the box describing the problem, then the debug string
//! (selectable, never changed by hand) with the copy button beside it, put in
//! by `panel` as the markup has no copy button.

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
    </div>
  </panel>
</form>
"#
);
