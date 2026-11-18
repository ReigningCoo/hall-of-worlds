//! Brew's panel (brewing plan PR 7): using Brew opens a dialog to choose an
//! ointment and a bottle of water carried, saying what they would make
//! (`Inventory::check_brew`, the server's own rule): a potion, or nothing, both
//! wasted all the same. Brew asks the server (`ClientMessage::Brew`), which
//! winds up and brews it.

use bevy::prelude::*;
use shared::brewing::{BrewOrder, BrewRefused, Brewed, OintmentStack, WaterStack};
use shared::protocol::ClientMessage;

use super::grind::doses;
use crate::connection::ToServer;
use crate::dom::{Dom, Template, UiEvent, UiValue, document};
use crate::html;
use crate::inventory::Carried;

/// Whether the panel is up, what it offers, in the dropdowns' order, and what
/// is chosen.
#[derive(Resource, Default)]
pub(super) struct Brewing {
    pub(super) open: bool,
    ointments: Vec<OintmentStack>,
    ointment: usize,
    waters: Vec<WaterStack>,
    water: usize,
    /// Set when what is chosen changes, so what it makes is shown again.
    dirty: bool,
}

impl Brewing {
    /// The order for what is chosen, if both are there.
    fn order(&self) -> Option<BrewOrder> {
        Some(BrewOrder {
            ointment: self.ointments.get(self.ointment)?.stack,
            water: self.waters.get(self.water)?.stack,
        })
    }
}

pub(super) static BREW: Template = html!(
    "brew",
    r#"
<form id="brew">
  <dialog id="asking" title="Brew" depth="1" hidden>
    <p id="words" class="medium" width="440"/>
    <span nowrap valign="middle">
      <label for="ointment" width="90">Ointment</label>
      <select id="ointment" width="340">
        <option>None</option>
      </select>
    </span>
    <span nowrap valign="middle">
      <label for="water" width="90">Water</label>
      <select id="water" width="340">
        <option>None</option>
      </select>
    </span>
    <p id="makes" width="440">.</p>
    <footer>
      <button id="brew">Brew</button>
      <button id="cancel">Cancel</button>
    </footer>
  </dialog>
</form>
"#
);

const PICK: &str = "Brew an ointment with a bottle of water in the cauldron: the \
    potion takes the water's place in the bottle.";

pub(super) fn spawn_panel(mut commands: Commands) {
    commands.spawn(document(&BREW));
}

/// Up while choosing, its choices from what is carried now.
pub(super) fn show_panel(mut brewing: ResMut<Brewing>, carried: Res<Carried>, mut dom: Dom) {
    let opened = brewing.is_changed() && brewing.open;
    if opened || (brewing.open && carried.is_changed()) {
        let ointments = carried.0.ointment_stacks();
        let waters = carried.0.water_stacks();
        if opened || ointments != brewing.ointments || waters != brewing.waters {
            let rows = ointments.iter().map(ointment_row);
            fill(&mut dom, "brew/ointment", rows.collect(), "No ointment");
            let rows = waters.iter().map(water_row);
            fill(&mut dom, "brew/water", rows.collect(), "No water");
            dom.set_text("brew/words", PICK);
            brewing.ointment = 0;
            brewing.water = 0;
            brewing.ointments = ointments;
            brewing.waters = waters;
            brewing.dirty = true;
        }
    }
    if brewing.is_changed() {
        dom.set_flag("brew/asking", "hidden", !brewing.open);
    }
    if brewing.open && brewing.dirty {
        brewing.dirty = false;
        let made = brewing.order().map(|order| carried.0.check_brew(&order));
        let missing = match brewing.ointments.is_empty() {
            true => BrewRefused::Ointment,
            false => BrewRefused::Water,
        };
        let made = made.unwrap_or(Err(missing));
        dom.set_text("brew/makes", makes(&made));
        dom.set_flag("brew/brew", "disabled", made.is_err());
    }
}

/// A dropdown's options, or one saying there is none.
fn fill(dom: &mut Dom, id: &str, rows: Vec<String>, none: &str) {
    let rows = match rows.is_empty() {
        true => vec![none.to_owned()],
        false => rows,
    };
    let options = rows.into_iter().enumerate();
    dom.set_options(id, options.map(|(at, row)| (at.to_string(), row)));
    dom.set(id, "value", "0");
}

/// "Common (2): +4 Strength, −1 Will".
fn ointment_row(stack: &OintmentStack) -> String {
    let held = doses(&stack.contents);
    format!("{:?} ({}): {held}", stack.quality, stack.count)
}

/// "Moon Water (3)".
fn water_row(stack: &WaterStack) -> String {
    format!("{} ({})", stack.water.name(), stack.count)
}

/// What the choice would make, or why it can't be brewed.
fn makes(made: &Result<Brewed, BrewRefused>) -> String {
    match made {
        Ok(Brewed {
            potion: Some(potion),
            ..
        }) => {
            let quality = shared::brewing::quality(potion);
            format!("Makes a {quality:?} potion: {}.", doses(potion))
        }
        Ok(_) => "No potion would form: the ointment and the water would be wasted.".to_owned(),
        Err(BrewRefused::Ointment) => "It needs an ointment: grind one first.".to_owned(),
        Err(BrewRefused::Water) => "It needs a bottle of water.".to_owned(),
    }
}

/// The answer: Brew asks the server, Cancel or a press outside closes it.
pub(super) fn answer_panel(
    mut events: MessageReader<UiEvent>,
    mut brewing: ResMut<Brewing>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("brew", name) = event.id.parts() else {
            continue;
        };
        match (&event.value, name) {
            (UiValue::Choice { index, .. }, "ointment") => brewing.ointment = *index,
            (UiValue::Choice { index, .. }, "water") => brewing.water = *index,
            (UiValue::Pressed, "brew") => {
                if let Some(order) = brewing.order() {
                    to_server.write(ToServer(ClientMessage::Brew(order)));
                }
                brewing.open = false;
            }
            (UiValue::Pressed, "cancel") | (UiValue::Dismissed, _) => brewing.open = false,
            _ => continue,
        }
        brewing.dirty = true;
    }
}
