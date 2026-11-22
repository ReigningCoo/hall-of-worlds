//! The Cooking panel as markup: a moving panel with two tabs. Cooking: the
//! tools along the top, the grid of rows under it (canvases `grid` fills),
//! the beyond-skill note, the kitchen in words, and on the right Start, the
//! count with its buttons, the culture's space, the product's frame and what
//! one run would make. Recipes: the recipe book, a copy of the template
//! `recipe` each, with Read, Make and Remove; Remove asks first.

use crate::dom::Template;
use crate::html;

pub(super) static COOKING: Template = html!(
    "cooking",
    r#"
<form id="cooking">
  <panel id="panel" title="Cooking" moveable left="46" top="22">
    <tab-bar id="tabs">
      <option>Cooking</option>
      <option>Recipes</option>
    </tab-bar>
    <div id="cooking_page">
      <p id="words" class="faint" width="760"/>
      <canvas id="tools"/>
      <hr/>
      <span id="body" nowrap valign="top">
        <div id="left" width="520">
          <p id="steps"/>
          <canvas id="grid"/>
          <p id="beyond" class="warning" width="500" hidden/>
          <p id="kitchen" class="faint" width="500"/>
        </div>
        <div width="240" class="tight">
          <button id="start" class="main">Start</button>
          <span nowrap valign="middle">
            <button id="fewest" title="Once">|&lt;</button>
            <button id="fewer" title="One fewer">&lt;</button>
            <p id="count" width="36" align="center">1</p>
            <button id="more" title="One more">&gt;</button>
            <button id="most" title="As many as the ingredients allow">&gt;|</button>
          </span>
          <span nowrap valign="middle">
            <canvas id="culture_space"/>
            <p class="faint" width="190">Culture, for what ferments</p>
          </span>
          <span nowrap valign="middle">
            <object-slot id="product"/>
            <p id="product_name" width="190"/>
          </span>
          <p id="chance"/>
          <p id="about" width="240"/>
        </div>
      </span>
    </div>
    <div id="recipes_page" width="760" hidden>
      <p class="faint" width="760">Read a recipe, or Make it: its steps go onto the Cooking tab, filled from what you carry, with anything missing shown in red. Recipe scrolls are read by dropping them on yourself.</p>
      <p id="no_recipes" class="faint" hidden>Your recipe book is empty. Read a recipe scroll, or cook something: whatever you cook is remembered here.</p>
      <scroll id="recipes_scroll" max-height="440">
        <table id="recipes" class="striped" cellpadding="4"/>
      </scroll>
    </div>
  </panel>
  <dialog id="asking" title="Remove recipe" depth="1" hidden>
    <p id="question" class="medium" width="360"/>
    <footer>
      <button id="really_remove" class="danger">Remove</button>
      <button id="keep">Keep</button>
    </footer>
  </dialog>
  <template id="recipe">
    <tr>
      <td fills>
        <span nowrap valign="middle">
          <img id="quill" src="icon:lorc/quill-ink" hidden/>
          <p id="dish"/>
          <p id="own_dish" class="hand" hidden/>
        </span>
        <p id="whose" class="faint"/>
        <p id="recipe_beyond" class="warning" hidden/>
      </td>
      <td><button id="read">Read</button></td>
      <td><button id="make" class="main">Make</button></td>
      <td><button id="remove">Remove</button></td>
    </tr>
  </template>
</form>
"#
);

/// The words along the top.
pub(super) const WORDS: &str = "Drag a tool down onto a place in a row, or click it to add \
    it to the top row; drag steps to move them, or off the grid to take them out; click a \
    step to join both rows there. Drag foods, water, milk and honey from the Inventory into \
    the spaces under each step (again for one more); click one to take it out.";
