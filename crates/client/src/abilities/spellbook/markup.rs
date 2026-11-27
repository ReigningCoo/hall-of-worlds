//! The Spells tab's markup: Design a Spell (opening the spell design panel),
//! a word on what to do, the list (a row per spell), and the requesters for
//! renaming and removing.

use crate::dom::Template;
use crate::html;

pub(super) static SPELLBOOK: Template = html!(
    "abilities.spellbook",
    r#"
<form id="spellbook">
  <div>
    <button id="design">Design a Spell</button>
    <p class="faint" width="520">Drag a spell onto the action bar, or click it to cast it. Favourites stay at the top.</p>
    <p id="empty" class="faint" hidden>Your spellbook is empty.</p>
    <scroll id="scroll" max-height="440">
      <table id="spells" class="striped" cellpadding="4"/>
    </scroll>
  </div>
  <dialog id="renaming" title="Rename spell" depth="1" hidden>
    <p class="medium" width="360">Leave it empty for the name made from its steps.</p>
    <input id="new_name" type="text" maxlength="40"/>
    <footer>
      <button id="really_rename" class="main">Rename</button>
      <button id="cancel_rename">Cancel</button>
    </footer>
  </dialog>
  <dialog id="asking" title="Remove spell" depth="1" hidden>
    <p id="question" class="medium" width="360"/>
    <footer>
      <button id="really_remove" class="danger">Remove</button>
      <button id="keep">Keep</button>
    </footer>
  </dialog>
  <template id="spell">
    <tr>
      <td><object id="picture" type="ability"/></td>
      <td fills>
        <p id="name"/>
        <p id="about" class="faint"/>
      </td>
      <td><button id="favourite" class="small" title="Favourite"><img src="icon:delapouite/round-star"/></button></td>
      <td><button id="rename">Rename</button></td>
      <td><button id="remove">Remove</button></td>
    </tr>
  </template>
</form>
"#
);
