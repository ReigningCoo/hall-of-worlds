//! The Herbs panel: the server's real herbs (`common::HerbTable`) in a dropdown,
//! the chosen one described in full (what players only find out bit by bit),
//! with what the character knows of it; Learn teaches it all, Learn All every
//! herb, Forget All forgets them (`DevHerbs`).

use bevy::prelude::*;
use shared::herb::{HerbDef, HerbKnowledge, Herbs};
use shared::protocol::{ClientMessage, DevHerbs};
use shared::role::DebugTool;

use crate::character::CharacterSheet;
use crate::common::{DebugTools, HerbTable};
use crate::connection::ToServer;
use crate::dom::{Dom, Template};
use crate::html;
use crate::widgets::{ButtonPressed, ButtonState, DropdownChosen, UiId};

pub(super) const FORM: &str = "debug.herbs";

/// The panel's contents, as markup: the herbs are given by `fill_herbs`.
pub(super) static HERBS: Template = html!(
    "debug_menu.herbs",
    r#"
<form id="debug.herbs">
  <panel id="panel" moveable>
    <span nowrap valign="middle">
      <select id="herb" searchable width="260"><option>Herb</option></select>
      <button id="learn">Learn</button>
    </span>
    <p id="about" width="420">.</p>
    <p id="known">.</p>
    <span nowrap>
      <button id="learn_all">Learn All</button>
      <button id="forget_all">Forget All</button>
    </span>
  </panel>
</form>
"#
);

/// The herb dropdown, with the keys it lists, and the one chosen.
#[derive(Component, Default)]
pub(in crate::debug_menu) struct HerbChoice {
    keys: Vec<String>,
    chosen: usize,
}

/// A button, and what it asks.
#[derive(Component, Clone, Copy)]
pub(in crate::debug_menu) enum HerbButton {
    Learn,
    LearnAll,
    ForgetAll,
}

/// What a built part is, from its id.
pub(super) fn bind(bound: &mut EntityCommands, id: &UiId) {
    match id.parts().1 {
        "herb" => bound.insert(HerbChoice::default()),
        "learn" => bound.insert(HerbButton::Learn),
        "learn_all" => bound.insert(HerbButton::LearnAll),
        "forget_all" => bound.insert(HerbButton::ForgetAll),
        _ => bound,
    };
}

/// Adds the panel's systems to the debug menu's plugin.
pub(in crate::debug_menu) fn add_herbs_panel(app: &mut App) {
    app.add_systems(
        Update,
        (
            fill_herbs,
            choose_herb,
            show_herb,
            press_buttons,
            grey_buttons,
        )
            .chain(),
    );
}

/// Lists the herbs by name, again when the table changes.
fn fill_herbs(table: Res<HerbTable>, mut choices: Query<&mut HerbChoice>, mut dom: Dom) {
    for mut choice in &mut choices {
        if !(choice.is_added() || table.is_changed()) {
            continue;
        }
        let mut named: Vec<_> = table.0.iter().map(|def| (&def.name, &def.key)).collect();
        named.sort();
        choice.keys = named.iter().map(|(_, key)| (*key).clone()).collect();
        choice.chosen = 0;
        let names = named.into_iter().map(|(name, _)| name.clone()).enumerate();
        let id = format!("{FORM}/herb");
        dom.set_options(&id, names.map(|(at, name)| (at.to_string(), name)));
        dom.set(&id, "value", "0");
    }
}

fn choose_herb(mut chosen: MessageReader<DropdownChosen>, mut choices: Query<&mut HerbChoice>) {
    for &DropdownChosen(dropdown, index) in chosen.read() {
        if let Ok(mut choice) = choices.get_mut(dropdown) {
            choice.chosen = index;
        }
    }
}

/// Describes the chosen herb, and says what is known.
fn show_herb(
    choices: Query<Ref<HerbChoice>>,
    table: Res<HerbTable>,
    sheet: Res<CharacterSheet>,
    mut dom: Dom,
) {
    for choice in &choices {
        if !(choice.is_changed() || table.is_changed() || sheet.is_changed()) {
            continue;
        }
        let def = choice
            .keys
            .get(choice.chosen)
            .and_then(|key| table.0.get(key));
        let about = def.map_or_else(String::new, describe);
        let known = &sheet.0.herbs;
        dom.set_text(&format!("{FORM}/about"), about);
        dom.set_text(&format!("{FORM}/known"), known_words(def, known, &table.0));
    }
}

/// "Roseroot (Rhodiola rosea): Root. Rare, in Tundra, BorealForest, -8 to 6 °C.
/// Toughness +2, Will +2, Intellect +1, Empathy -1."
fn describe(def: &HerbDef) -> String {
    let biomes: Vec<String> = def
        .biomes
        .iter()
        .map(|biome| format!("{biome:?}"))
        .collect();
    let effects: Vec<String> = def
        .effects
        .iter()
        .map(|effect| format!("{:?} {:+}", effect.stat, effect.change))
        .collect();
    format!(
        "{} ({}): {}. {:?}, in {}{}, {} to {} °C.\n{}.",
        def.name,
        def.latin,
        def.part_name,
        def.abundance,
        biomes.join(", "),
        if def.marsh { " and marshes" } else { "" },
        def.temperature.min,
        def.temperature.max,
        effects.join(", "),
    )
}

/// "Known: its name and 1 of 4 effects. 3 of 82 herbs known."
fn known_words(def: Option<&HerbDef>, known: &HerbKnowledge, herbs: &Herbs) -> String {
    let named = herbs
        .iter()
        .filter(|def| known.knows_name(&def.key))
        .count();
    let all = format!("{named} of {} herbs' names known.", herbs.len());
    let Some(def) = def else {
        return all;
    };
    let of = known.of(&def.key);
    let effects = of.effects.iter().filter(|&&known| known).count();
    let name = if of.name { "its name" } else { "not its name" };
    format!(
        "Known: {name} and {effects} of {} effects. {all}",
        of.effects.len()
    )
}

fn press_buttons(
    mut pressed: MessageReader<ButtonPressed>,
    buttons: Query<&HerbButton>,
    choices: Query<&HerbChoice>,
    mut to_server: MessageWriter<ToServer>,
) {
    for press in pressed.read() {
        let Ok(button) = buttons.get(press.0) else {
            continue;
        };
        let change = match button {
            HerbButton::Learn => {
                let chosen = choices.iter().next();
                let key = chosen.and_then(|choice| choice.keys.get(choice.chosen));
                let Some(key) = key else {
                    continue;
                };
                DevHerbs::Learn(key.clone())
            }
            HerbButton::LearnAll => DevHerbs::LearnAll,
            HerbButton::ForgetAll => DevHerbs::ForgetAll,
        };
        to_server.write(ToServer(ClientMessage::DevHerbs(change)));
    }
}

/// Greys the buttons without permission (the panel can be open from before).
fn grey_buttons(tools: Res<DebugTools>, mut buttons: Query<&mut ButtonState, With<HerbButton>>) {
    let state = match tools.may(DebugTool::LearnSkills) {
        true => ButtonState::Normal,
        false => ButtonState::Disabled,
    };
    for mut button in &mut buttons {
        button.set_if_neq(state);
    }
}
