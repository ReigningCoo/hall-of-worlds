//! Browsers play sound only after the player has done something on the page
//! (a click, a tap, a key), so in a browser music waits for that. Natively it
//! can start at once.

use bevy::input::touch::Touches;
use bevy::prelude::*;

/// Whether sound may start: always natively, after the first click, tap or
/// key in a browser.
#[derive(Resource, Debug, PartialEq, Eq)]
pub(super) struct MayPlay(pub bool);

impl Default for MayPlay {
    fn default() -> Self {
        Self(!cfg!(target_arch = "wasm32"))
    }
}

/// Notes the first click, tap or key.
pub(super) fn watch_for_gesture(
    mut may_play: ResMut<MayPlay>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
) {
    if may_play.0 {
        return;
    }
    if mouse.get_just_released().next().is_some()
        || keys.get_just_released().next().is_some()
        || touches.iter_just_released().next().is_some()
    {
        may_play.0 = true;
    }
}
