//! Other Floors: sliders for how a building's other floors are drawn (below
//! and above the player's) (`common::OtherFloorLooks`), to tune them by eye, and a switch for
//! remembered slices. Only this client's drawing, for this run.

use bevy::prelude::*;

use super::super::actions::DebugAction;
use super::OpenPanels;
use crate::common::OtherFloorLooks;
use crate::dom::{Dom, Template, UiEvent, UiValue};
use crate::html;

/// The panel's form, for its ids.
pub(super) const FORM: &str = "debug.other_floors";

/// The panel's contents, as markup: a slider and its value per number.
pub(super) static OTHER_FLOORS: Template = html!(
    "debug_menu.other_floors",
    r#"
<form id="debug.other_floors">
  <panel id="panel" moveable>
    <table cellspacing="4">
      <tr>
        <td><label for="day_dark">Darker per storey by day</label></td>
        <td><input id="day_dark" type="range" min="0" max="0.9" step="0.05" value="0.45"/></td>
        <td><span id="day_dark_value">0.45</span></td>
      </tr>
      <tr>
        <td><label for="night_dark">Darker per storey at night</label></td>
        <td><input id="night_dark" type="range" min="0" max="0.9" step="0.05" value="0.7"/></td>
        <td><span id="night_dark_value">0.70</span></td>
      </tr>
      <tr>
        <td><label for="ground_range">Ground shown (cells)</label></td>
        <td><input id="ground_range" type="range" min="0" max="24" step="1" value="24"/></td>
        <td><span id="ground_range_value">24</span></td>
      </tr>
      <tr>
        <td><label for="storey">Storey height (walls)</label></td>
        <td><input id="storey" type="range" min="1" max="2" step="0.05" value="1.5"/></td>
        <td><span id="storey_value">1.50</span></td>
      </tr>
      <tr>
        <td><label for="wedge">Hidden above, towards camera (degrees)</label></td>
        <td><input id="wedge" type="range" min="0" max="360" step="5" value="120"/></td>
        <td><span id="wedge_value">120.00</span></td>
      </tr>
      <tr>
        <td><label for="remembered">Remembered slices</label></td>
        <td><input id="remembered" type="checkbox" switch checked/></td>
        <td/>
      </tr>
    </table>
  </panel>
</form>
"#
);

/// The sliders, by id, and the number each sets.
const SLIDERS: [&str; 5] = ["day_dark", "night_dark", "ground_range", "storey", "wedge"];

/// Adds the panel's systems to the debug menu's plugin.
pub(in crate::debug_menu) fn add_other_floors_panel(app: &mut App) {
    app.add_systems(Update, (show_on_opening, take_changes).chain());
}

/// A number of the looks, by its slider's id.
fn number<'a>(looks: &'a mut OtherFloorLooks, id: &str) -> Option<&'a mut f32> {
    match id {
        "day_dark" => Some(&mut looks.day_dark),
        "night_dark" => Some(&mut looks.night_dark),
        "ground_range" => Some(&mut looks.ground_range),
        "storey" => Some(&mut looks.storey),
        "wedge" => Some(&mut looks.wedge),
        _ => None,
    }
}

/// On opening, sets the sliders to the numbers in use (changed since the
/// markup's, if it was open before).
fn show_on_opening(
    open: Res<OpenPanels>,
    looks: Res<OtherFloorLooks>,
    mut was_open: Local<bool>,
    mut dom: Dom,
) {
    let is_open = open.is_open(DebugAction::OtherFloors);
    if is_open && !*was_open {
        let mut looks = *looks;
        for id in SLIDERS {
            if let Some(value) = number(&mut looks, id) {
                dom.set(&format!("{FORM}/{id}"), "value", &value.to_string());
                dom.set_text(&format!("{FORM}/{id}_value"), format!("{value:.2}"));
            }
        }
        dom.set_flag(&format!("{FORM}/remembered"), "checked", looks.remembered);
    }
    *was_open = is_open;
}

/// Takes what the sliders and switch say.
fn take_changes(
    mut events: MessageReader<UiEvent>,
    mut looks: ResMut<OtherFloorLooks>,
    mut dom: Dom,
) {
    for event in events.read() {
        let (FORM, id) = event.id.parts() else {
            continue;
        };
        match &event.value {
            UiValue::Number(value) => {
                if let Some(number) = number(&mut looks, id) {
                    *number = *value;
                    dom.set_text(&format!("{FORM}/{id}_value"), format!("{value:.2}"));
                }
            }
            UiValue::Flag(on) if id == "remembered" => looks.remembered = *on,
            _ => {}
        }
    }
}
