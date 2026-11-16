//! What the Place Furniture panel's widgets do, sending the pieces clicked in
//! Furniture Mode, and keeping that mode to where it can be used.

use bevy::prelude::*;
use shared::furniture::Material;
use shared::protocol::{ClientMessage, DevFurniture};
use shared::role::DebugTool;

use super::create_mob_systems::can_place;
use super::furniture_order::{FurnitureOrder, Kind};
use super::place_furniture::{FACINGS, FORM};
use super::{FurnitureChoice, FurnitureClickToPlace, FurniturePlace, OpenPanels, new_seed};
use crate::common::{ClickMode, CurrentDungeon, DebugTools, FurnitureClicked, GameView};
use crate::connection::ToServer;
use crate::debug_menu::DebugAction;
use crate::dom::Dom;
use crate::widgets::{ButtonPressed, ButtonState, DropdownChosen};

/// Adds the panel's systems to the debug menu's plugin.
pub(in crate::debug_menu) fn add_place_furniture_panel(app: &mut App) {
    app.init_resource::<FurnitureOrder>().add_systems(
        Update,
        (
            start_furniture_panel,
            choose_furniture,
            press_place,
            keep_furniture_mode_where_it_works,
            show_furniture_mode,
            place_furniture,
        )
            .chain(),
    );
}

/// Whether Furniture Mode can be used: as Mob Mode, with its own permission.
fn usable(view: &GameView, dungeon: &CurrentDungeon, tools: &DebugTools) -> bool {
    can_place(view, dungeon, tools.may(DebugTool::PlaceFurniture))
}

/// The dropdown's choices for the order's kind, and the one chosen.
fn fill(dom: &mut Dom, choice: FurnitureChoice, order: &FurnitureOrder) {
    let selected = match choice {
        FurnitureChoice::Kind => Kind::ALL.iter().position(|&k| k == order.kind),
        FurnitureChoice::Size => Some(order.size),
        FurnitureChoice::Facing => FACINGS.iter().position(|&(_, s)| s == order.facing),
        FurnitureChoice::Material => order
            .material
            .and_then(|m| Material::ALL.iter().position(|&it| it == m))
            .map(|at| at + 1),
    };
    // Each choice's value is its place in the list.
    let id = format!("{FORM}/{}", choice.ui_name());
    let options = choice.options(order.kind).into_iter().enumerate();
    dom.set_options(&id, options.map(|(at, name)| (at.to_string(), name)));
    dom.set(&id, "value", &selected.unwrap_or(0).to_string());
}

/// A panel opening shows the order as it was left.
fn start_furniture_panel(
    dropdowns: Query<&FurnitureChoice, Added<FurnitureChoice>>,
    order: Res<FurnitureOrder>,
    mut dom: Dom,
) {
    for &choice in &dropdowns {
        fill(&mut dom, choice, &order);
    }
}

fn choose_furniture(
    mut chosen: MessageReader<DropdownChosen>,
    choices: Query<&FurnitureChoice>,
    mut order: ResMut<FurnitureOrder>,
    mut dom: Dom,
) {
    for &DropdownChosen(dropdown, index) in chosen.read() {
        match choices.get(dropdown) {
            Ok(FurnitureChoice::Kind) => {
                order.kind = Kind::ALL[index];
                // Its own sizes, the largest kept where it has it.
                order.size = order.size.min(order.kind.sizes().len() - 1);
                fill(&mut dom, FurnitureChoice::Size, &order);
            }
            Ok(FurnitureChoice::Size) => order.size = index,
            Ok(FurnitureChoice::Facing) => order.facing = FACINGS[index].1,
            Ok(FurnitureChoice::Material) => {
                order.material = index.checked_sub(1).map(|at| Material::ALL[at]);
            }
            Err(_) => {}
        }
    }
}

/// Place turns Furniture Mode on and off.
fn press_place(
    mut pressed: MessageReader<ButtonPressed>,
    place: Query<&ButtonState, With<FurniturePlace>>,
    mut mode: ResMut<ClickMode>,
) {
    for press in pressed.read() {
        if let Ok(state) = place.get(press.0)
            && *state != ButtonState::Disabled
        {
            *mode = if *mode == ClickMode::PlaceFurniture {
                ClickMode::Normal
            } else {
                ClickMode::PlaceFurniture
            };
        }
    }
}

/// Furniture Mode goes off with the panel, or anywhere it can't be used.
fn keep_furniture_mode_where_it_works(
    open: Res<OpenPanels>,
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    tools: Res<DebugTools>,
    mut mode: ResMut<ClickMode>,
) {
    let works = usable(view.get(), &dungeon, &tools);
    let open = open.is_open(DebugAction::PlaceFurniture);
    if *mode == ClickMode::PlaceFurniture && (!open || !works) {
        *mode = ClickMode::Normal;
    }
}

/// Place shows Furniture Mode as on, greyed where it can't be used, with
/// "Click to place" beside it while on.
fn show_furniture_mode(
    mode: Res<ClickMode>,
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    tools: Res<DebugTools>,
    mut place: Query<&mut ButtonState, With<FurniturePlace>>,
    mut words: Query<&mut Visibility, With<FurnitureClickToPlace>>,
) {
    let on = *mode == ClickMode::PlaceFurniture;
    let state = match (usable(view.get(), &dungeon, &tools), on) {
        (false, _) => ButtonState::Disabled,
        (true, true) => ButtonState::Selected,
        (true, false) => ButtonState::Normal,
    };
    for mut button in &mut place {
        button.set_if_neq(state);
    }
    let shown = if on {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    for mut visibility in &mut words {
        visibility.set_if_neq(shown);
    }
}

/// A cell clicked in Furniture Mode: the piece for it, to the server.
fn place_furniture(
    mut clicked: MessageReader<FurnitureClicked>,
    mode: Res<ClickMode>,
    mut order: ResMut<FurnitureOrder>,
    mut rolls: Local<i64>,
    time: Res<Time<Real>>,
    mut to_server: MessageWriter<ToServer>,
) {
    for &FurnitureClicked { x, y } in clicked.read() {
        if *mode != ClickMode::PlaceFurniture {
            continue;
        }
        *rolls += 1;
        let roll = new_seed(time.elapsed().as_nanos() as u64, *rolls);
        let piece = order.click((x, y), roll);
        let message = ClientMessage::DevFurniture(DevFurniture { piece });
        to_server.write(ToServer(message));
    }
}
