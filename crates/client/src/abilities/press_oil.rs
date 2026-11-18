//! Choosing what Press Oil presses (brewing plan PR 5): using it opens a
//! requester listing what is carried with a bottle's worth at least (nuts,
//! seeds, vegetables; `shared::potion::Inventory::press_choices`), how many
//! and how many bottles of which oil it would fill. Press asks the server with
//! that stack as the target; it then presses bottle after bottle by itself.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::potion::PressChoice;
use shared::protocol::ClientMessage;

use crate::common::HerbTable;
use crate::connection::ToServer;
use crate::dom::{Dom, Template, UiEvent, UiValue, document};
use crate::html;
use crate::inventory::Carried;

/// Whether the requester is up, and what it offers, in the dropdown's order.
#[derive(Resource, Default)]
pub(super) struct PressChooser {
    pub(super) open: bool,
    choices: Vec<PressChoice>,
    chosen: usize,
}

pub(super) static PRESS_OIL: Template = html!(
    "press_oil",
    r#"
<form id="press_oil">
  <dialog id="asking" title="Press Oil" depth="1" hidden>
    <p id="words" class="medium" width="400"/>
    <select id="what" width="400">
      <option>Nothing</option>
    </select>
    <footer>
      <button id="press">Press</button>
      <button id="cancel">Cancel</button>
    </footer>
  </dialog>
</form>
"#
);

const NOTHING: &str = "Nothing carried can be pressed. It takes nuts, seeds or \
    vegetables, enough for a bottle, and an empty bottle to fill.";
const PICK: &str = "What to press? Bottles are filled one after another until \
    there is too little left, or you move.";

pub(super) fn spawn_requester(mut commands: Commands) {
    commands.spawn(document(&PRESS_OIL));
}

/// Up while choosing, its choices from what is carried now.
pub(super) fn show_requester(
    mut chooser: ResMut<PressChooser>,
    carried: Res<Carried>,
    herbs: Res<HerbTable>,
    mut dom: Dom,
) {
    let opened = chooser.is_changed() && chooser.open;
    if opened || (chooser.open && (carried.is_changed() || herbs.is_changed())) {
        let choices = carried.0.press_choices(&herbs.0);
        if choices != chooser.choices || opened {
            let options = choices.iter().enumerate().map(|(at, choice)| {
                let name = carried
                    .0
                    .find(choice.stack)
                    .map_or_else(String::new, |item| item.name());
                (at.to_string(), words(&name, choice))
            });
            let options: Vec<_> = options.collect();
            if !options.is_empty() {
                dom.set_options("press_oil/what", options);
                dom.set("press_oil/what", "value", "0");
            }
            let none = choices.is_empty();
            dom.set_text("press_oil/words", if none { NOTHING } else { PICK });
            dom.set_flag("press_oil/what", "hidden", none);
            dom.set_flag("press_oil/press", "disabled", none);
            chooser.chosen = 0;
            chooser.choices = choices;
        }
    }
    if chooser.is_changed() {
        dom.set_flag("press_oil/asking", "hidden", !chooser.open);
    }
}

/// One choice in words: "Walnut (24): 2 bottles of Nut Oil".
fn words(name: &str, choice: &PressChoice) -> String {
    let bottles = if choice.bottles == 1 {
        "bottle"
    } else {
        "bottles"
    };
    let oil = choice.oil.name();
    format!(
        "{name} ({}): {} {bottles} of {oil}",
        choice.count, choice.bottles
    )
}

/// The answer: Press asks the server, Cancel or a press outside closes it.
pub(super) fn answer_requester(
    mut events: MessageReader<UiEvent>,
    mut chooser: ResMut<PressChooser>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("press_oil", name) = event.id.parts() else {
            continue;
        };
        match (&event.value, name) {
            (UiValue::Choice { index, .. }, "what") => chooser.chosen = *index,
            (UiValue::Pressed, "press") => {
                if let Some(choice) = chooser.choices.get(chooser.chosen) {
                    to_server.write(ToServer(ClientMessage::UseAbility {
                        ability: Ability::PressOil,
                        target: Some(choice.stack),
                    }));
                }
                chooser.open = false;
            }
            (UiValue::Pressed, "cancel") | (UiValue::Dismissed, _) => chooser.open = false,
            _ => {}
        }
    }
}
