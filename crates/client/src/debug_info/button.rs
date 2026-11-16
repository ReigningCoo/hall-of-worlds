//! The bug button: at the top right, just left of the system menu's gear
//! (the same size, an inked ladybird on it). Pressed or tapped, it opens Debug Info.
//! Hidden while connecting, as the corner buttons at the bottom are.

use bevy::prelude::*;

use crate::common::GameView;
use crate::dom::{Template, document, top_strip};
use crate::html;
use crate::widgets::{ButtonTooltip, INK, UiId};

/// 8 from the window's top, and 6 left of the gear (8 from the right, 34
/// across). No form round it, as for the gear.
pub(super) static BUTTON: Template = html!(
    "debug_info.button",
    r#"
<button id="debug_info" form="hud" top="8" right="48" width="34" height="34">
  <img src="icon:delapouite/ladybug"/>
</button>
"#
);

/// Marks the button.
#[derive(Component)]
pub(super) struct BugButton;

pub(super) fn spawn_button(mut commands: Commands) {
    // Above the maps, as the gear is.
    let button = (document(&BUTTON), GlobalZIndex(15));
    commands.spawn(button).insert(top_strip());
}

/// Marks the button once it is built, with its tip, and inks its ladybird
/// (the icon is white, and a button shows its picture as it is).
pub(super) fn bind_button(
    new: Query<(Entity, &UiId), Added<UiId>>,
    children: Query<&Children>,
    mut pictures: Query<&mut ImageNode>,
    mut commands: Commands,
) {
    for (entity, id) in &new {
        if id.parts() == ("hud", "debug_info") {
            let tip = ButtonTooltip("Debug Info: report a problem".into());
            commands.entity(entity).insert((BugButton, tip));
            for inside in children.iter_descendants(entity) {
                if let Ok(mut picture) = pictures.get_mut(inside) {
                    picture.color = INK;
                }
            }
        }
    }
}

/// The button is hidden while connecting.
pub(super) fn show_button(
    view: Res<State<GameView>>,
    button: Option<Single<&mut Visibility, With<BugButton>>>,
) {
    // (Not there until its markup is built.)
    let Some(mut button) = button else {
        return;
    };
    button.set_if_neq(if *view.get() == GameView::Connecting {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    });
}
