//! Choosing what Press Juice presses (drinks plan PR 3): using it opens a
//! requester listing the fruit carried that can be juiced while a carried
//! cask or vessel takes its juice (`shared::juice`'s
//! `Inventory::juice_choices`), with how many and how much juice. Press asks
//! the server with that stack as the target; it then presses again and again
//! by itself until the fruit runs out, the cask or vessel is full, or the
//! player moves.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::juice::JuiceChoice;
use shared::protocol::ClientMessage;

use crate::connection::ToServer;
use crate::dom::{Dom, Template, UiEvent, UiValue, document};
use crate::html;
use crate::inventory::Carried;

/// Whether the requester is up, and what it offers, in the dropdown's order.
#[derive(Resource, Default)]
pub(super) struct JuiceChooser {
    pub(super) open: bool,
    choices: Vec<JuiceChoice>,
    chosen: usize,
}

pub(super) static PRESS_JUICE: Template = html!(
    "press_juice",
    r#"
<form id="press_juice">
  <dialog id="asking" title="Press Juice" depth="1" hidden>
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

const NOTHING: &str = "Nothing carried can be pressed into juice. It takes \
    juicy fruit, and a cask or vessel to take the juice, empty or holding the \
    same.";
const PICK: &str = "What to press? It is pressed a couple of kilograms at a \
    time until it runs out, the cask or vessel is full, or you move.";

pub(super) fn spawn_requester(mut commands: Commands) {
    commands.spawn(document(&PRESS_JUICE));
}

/// Up while choosing, its choices from what is carried now.
pub(super) fn show_requester(
    mut chooser: ResMut<JuiceChooser>,
    carried: Res<Carried>,
    mut dom: Dom,
) {
    let opened = chooser.is_changed() && chooser.open;
    if opened || (chooser.open && carried.is_changed()) {
        let choices = carried.0.juice_choices();
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
                dom.set_options("press_juice/what", options);
                dom.set("press_juice/what", "value", "0");
            }
            let none = choices.is_empty();
            dom.set_text("press_juice/words", if none { NOTHING } else { PICK });
            dom.set_flag("press_juice/what", "hidden", none);
            dom.set_flag("press_juice/press", "disabled", none);
            chooser.chosen = 0;
            chooser.choices = choices;
        }
    }
    if chooser.is_changed() {
        dom.set_flag("press_juice/asking", "hidden", !chooser.open);
    }
}

/// One choice in words: "Apple (12): 1.3 L of juice, 0.5 L room".
fn words(name: &str, choice: &JuiceChoice) -> String {
    format!(
        "{name} ({}): {} of juice, {} room",
        choice.count,
        litres(choice.ml),
        litres(choice.room)
    )
}

/// Millilitres as litres to a tenth: "1.3 L".
fn litres(ml: u32) -> String {
    format!("{}.{} L", ml / 1000, ml % 1000 / 100)
}

/// The answer: Press asks the server, Cancel or a press outside closes it.
pub(super) fn answer_requester(
    mut events: MessageReader<UiEvent>,
    mut chooser: ResMut<JuiceChooser>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("press_juice", name) = event.id.parts() else {
            continue;
        };
        match (&event.value, name) {
            (UiValue::Choice { index, .. }, "what") => chooser.chosen = *index,
            (UiValue::Pressed, "press") => {
                if let Some(choice) = chooser.choices.get(chooser.chosen) {
                    to_server.write(ToServer(ClientMessage::UseAbility {
                        ability: Ability::PressJuice,
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
