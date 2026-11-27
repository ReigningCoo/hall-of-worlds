//! The trade window as markup: a moving panel, empty for now (trading plan
//! PR 2); the window for shops (PR 4) and for players (PR 6) fill it.

use crate::dom::Template;
use crate::html;

pub(super) static TRADE: Template = html!(
    "trade",
    r#"
<form id="trade">
  <panel id="panel" title="Trade" moveable left="35%" top="25%">
    <p id="nobody" class="faint">Nobody to trade with yet</p>
  </panel>
</form>
"#
);
