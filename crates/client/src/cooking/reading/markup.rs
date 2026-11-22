//! The reading panel as markup: a moving panel drawn as an unrolled scroll,
//! titled with the dish; where the dish is from, or a quill and "Your
//! recipe" in ink for the reader's own; the recipe's grid (the canvas
//! `grid`); the note when it is beyond the reader's Cooking skill; and the
//! written steps, one copy of `step` each, then what it makes.

use crate::dom::Template;
use crate::html;

pub(super) static READING: Template = html!(
    "recipe",
    r#"
<form id="recipe">
  <panel id="panel" title="Recipe" moveable rolled left="30" top="12">
    <p id="cuisine" class="faint"/>
    <span id="own" class="hand" nowrap valign="middle" hidden>
      <img src="icon:lorc/quill-ink"/>
      <p id="own_words" class="hand"/>
    </span>
    <canvas id="grid"/>
    <p id="beyond" class="warning" max-width="560" hidden/>
    <hr/>
    <div id="steps" class="tight"/>
    <p id="makes" max-width="560"/>
    <template id="step"><p id="step" max-width="560"/></template>
  </panel>
</form>
"#
);
