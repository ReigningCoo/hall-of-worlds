//! Closing the panel: Esc, its close button, walking out of reach, leaving
//! the map, or a new level (`server.rs`).

use bevy::prelude::*;

use super::BenchPanel;
use super::state::in_reach;
use crate::common::{GameView, SpotPosition};
use crate::dom::{UiEvent, UiValue};
use crate::player::Player;

/// Esc closes the panel rather than opening the system menu.
pub(super) fn close_on_escape(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut panel: ResMut<BenchPanel>,
    nodes: Query<(), With<Node>>,
    mut commands: Commands,
) {
    let open = (panel.0.as_ref()).is_some_and(|open| nodes.contains(open.panel));
    if open && keys.clear_just_pressed(KeyCode::Escape) {
        close_panel(&mut panel, &mut commands);
    }
}

/// Walking out of reach, leaving the map or the close button closes it.
pub(super) fn close_when_left(
    mut events: MessageReader<UiEvent>,
    view: Res<State<GameView>>,
    player: Single<&SpotPosition, With<Player>>,
    mut panel: ResMut<BenchPanel>,
    mut commands: Commands,
) {
    let closed = (events.read())
        .any(|event| event.id.parts() == ("bench", "panel") && event.value == UiValue::Closed);
    let Some(open) = &panel.0 else {
        return;
    };
    let on_map = *view.get() == GameView::Dungeon;
    if closed || !on_map || !in_reach(player.0, &open.piece) {
        close_panel(&mut panel, &mut commands);
    }
}

/// Closes the panel, if open.
pub(super) fn close_panel(panel: &mut BenchPanel, commands: &mut Commands) {
    if let Some(open) = panel.0.take() {
        commands.entity(open.panel).try_despawn();
    }
}
