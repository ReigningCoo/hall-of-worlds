//! What the Create Crafting Bench panel's widgets do: its dropdowns, Create
//! and Rotate, the ghost's order kept in `common::BenchGhost` while Bench Mode
//! is on, right click turning it, and the bench sent for the cell clicked.

use bevy::prelude::*;
use shared::crafting_bench::Craft;
use shared::item::Quality;
use shared::protocol::{BenchAsk, ClientMessage};
use shared::role::DebugTool;

use super::OpenPanels;
use super::create_bench::{
    BenchClickToPlace, BenchCreate, BenchOrder, BenchRotate, BenchTool, BenchToolList, BenchType,
    FORM, craft_name, fill_tools,
};
use super::create_mob_systems::can_place;
use crate::common::{
    BenchGhost, ClickMode, CurrentDungeon, DebugTools, FurnitureClicked, GameView,
};
use crate::connection::ToServer;
use crate::debug_menu::DebugAction;
use crate::dom::{Dom, DomInput, Hit};
use crate::widgets::{ButtonPressed, ButtonState, DropdownChosen};

/// Adds the panel's systems to the debug menu's plugin.
pub(in crate::debug_menu) fn add_create_bench_panel(app: &mut App) {
    app.init_resource::<BenchOrder>()
        .add_systems(PreUpdate, turn_by_right_click.after(DomInput::Find))
        .add_systems(
            Update,
            (
                start_bench_panel,
                choose_bench,
                press_bench_buttons,
                keep_bench_mode_where_it_works,
                show_bench_mode,
                place_bench,
                keep_ghost_order,
            )
                .chain(),
        );
}

/// Whether Bench Mode can be used: as Mob Mode, with its own permission.
fn usable(view: &GameView, dungeon: &CurrentDungeon, tools: &DebugTools) -> bool {
    can_place(view, dungeon, tools.may(DebugTool::CreateCraftingBench))
}

/// A tool's dropdown: "None", then each quality; its value its place.
fn fill_tool(dom: &mut Dom, at: usize, quality: Option<Quality>) {
    let id = format!("{FORM}/quality.{at}");
    let names = std::iter::once("None".to_owned()).chain(Quality::ALL.map(|q| format!("{q:?}")));
    dom.set_options(
        &id,
        names.enumerate().map(|(i, name)| (i.to_string(), name)),
    );
    let chosen = quality.and_then(|q| Quality::ALL.iter().position(|&it| it == q));
    dom.set(&id, "value", &chosen.map_or(0, |i| i + 1).to_string());
}

/// A panel opening shows the order as it was left.
fn start_bench_panel(
    types: Query<(), Added<BenchType>>,
    lists: Query<(), Added<BenchToolList>>,
    tools: Query<&BenchTool, Added<BenchTool>>,
    order: Res<BenchOrder>,
    mut dom: Dom,
) {
    if !types.is_empty() {
        let id = format!("{FORM}/type");
        let crafts = Craft::ALL.iter().enumerate();
        dom.set_options(
            &id,
            crafts.map(|(i, &c)| (i.to_string(), craft_name(c).to_owned())),
        );
        dom.set(&id, "value", &order.craft.index().to_string());
    }
    if !lists.is_empty() {
        fill_tools(&mut dom, order.craft);
    }
    for &BenchTool(at) in &tools {
        fill_tool(&mut dom, at, order.tools.get(at).copied().flatten());
    }
}

fn choose_bench(
    mut chosen: MessageReader<DropdownChosen>,
    (types, tools): (Query<(), With<BenchType>>, Query<&BenchTool>),
    mut order: ResMut<BenchOrder>,
    mut dom: Dom,
) {
    for &DropdownChosen(dropdown, index) in chosen.read() {
        if types.contains(dropdown) {
            let craft = Craft::ALL[index];
            if craft != order.craft {
                // Another craft's tools: its rows anew, with none chosen.
                *order = BenchOrder::of(craft, order.facing);
                fill_tools(&mut dom, craft);
            }
        } else if let Ok(&BenchTool(at)) = tools.get(dropdown)
            && let Some(tool) = order.tools.get_mut(at)
        {
            *tool = index.checked_sub(1).map(|i| Quality::ALL[i]);
        }
    }
}

/// Create turns Bench Mode on and off; Rotate turns the bench.
fn press_bench_buttons(
    mut pressed: MessageReader<ButtonPressed>,
    buttons: Query<(&ButtonState, Has<BenchCreate>, Has<BenchRotate>)>,
    mut mode: ResMut<ClickMode>,
    mut order: ResMut<BenchOrder>,
) {
    for press in pressed.read() {
        let Ok((state, create, rotate)) = buttons.get(press.0) else {
            continue;
        };
        if *state == ButtonState::Disabled {
            continue;
        }
        if create {
            *mode = match *mode {
                ClickMode::PlaceBench => ClickMode::Normal,
                _ => ClickMode::PlaceBench,
            };
        } else if rotate {
            *order = order.turned();
        }
    }
}

/// In Bench Mode a right click on the map turns the bench, and is taken, so
/// it doesn't also target or clear the target.
fn turn_by_right_click(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mode: Res<ClickMode>,
    hit: Res<Hit>,
    mut order: ResMut<BenchOrder>,
) {
    if *mode == ClickMode::PlaceBench
        && mouse.just_pressed(MouseButton::Right)
        && hit.target.is_none()
    {
        mouse.clear_just_pressed(MouseButton::Right);
        *order = order.turned();
    }
}

/// Bench Mode goes off with the panel, or anywhere it can't be used.
fn keep_bench_mode_where_it_works(
    open: Res<OpenPanels>,
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    tools: Res<DebugTools>,
    mut mode: ResMut<ClickMode>,
) {
    let works = usable(view.get(), &dungeon, &tools);
    let open = open.is_open(DebugAction::CreateCraftingBench);
    if *mode == ClickMode::PlaceBench && (!open || !works) {
        *mode = ClickMode::Normal;
    }
}

/// Create and Rotate.
type BenchButtons = Or<(With<BenchCreate>, With<BenchRotate>)>;

/// Create shows Bench Mode as on, greyed where it can't be used; Rotate is
/// live only while it is on, with "Click to place" beside them.
fn show_bench_mode(
    mode: Res<ClickMode>,
    view: Res<State<GameView>>,
    dungeon: Res<CurrentDungeon>,
    tools: Res<DebugTools>,
    mut buttons: Query<(&mut ButtonState, Has<BenchCreate>), BenchButtons>,
    mut words: Query<&mut Visibility, With<BenchClickToPlace>>,
) {
    let on = *mode == ClickMode::PlaceBench;
    let works = usable(view.get(), &dungeon, &tools);
    for (mut button, create) in &mut buttons {
        let state = match (works, on, create) {
            (false, _, _) | (true, false, false) => ButtonState::Disabled,
            (true, true, true) => ButtonState::Selected,
            (true, _, _) => ButtonState::Normal,
        };
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

/// A cell clicked in Bench Mode: the bench there to the server if it fits,
/// and Create off.
fn place_bench(
    mut clicked: MessageReader<FurnitureClicked>,
    mut mode: ResMut<ClickMode>,
    ghost: Res<BenchGhost>,
    order: Res<BenchOrder>,
    mut to_server: MessageWriter<ToServer>,
) {
    for &FurnitureClicked { x, y } in clicked.read() {
        if *mode != ClickMode::PlaceBench {
            continue;
        }
        let shown = ghost.order.as_ref().map(|it| it.piece.at);
        // Only where the ghost shows it fitting: elsewhere it does nothing.
        if ghost.fits != Some(true) || shown != Some((x, y)) {
            continue;
        }
        let bench = order.bench((x, y));
        to_server.write(ToServer(ClientMessage::Bench(BenchAsk::Dev(bench))));
        *mode = ClickMode::Normal;
    }
}

/// While Bench Mode is on, the ghost is the order, wherever `dungeon_map`
/// last put it; off, there is none.
fn keep_ghost_order(mode: Res<ClickMode>, order: Res<BenchOrder>, mut ghost: ResMut<BenchGhost>) {
    let next = if *mode == ClickMode::PlaceBench {
        let at = ghost.order.as_ref().map_or((0, 0), |it| it.piece.at);
        let fits = ghost.fits;
        BenchGhost {
            order: Some(order.bench(at)),
            fits,
        }
    } else {
        BenchGhost::default()
    };
    ghost.set_if_neq(next);
}
