//! The trade window's button, right of the Quest Log's at the bottom left (the
//! same size, two arrows passing on it): pressed or tapped, it does what T
//! does. Hidden while connecting, as the Character button is.

use bevy::prelude::*;

use super::icon::trade_icon;
use crate::common::GameView;
use crate::dom::{Pictures, Template, bottom_strip, document};
use crate::html;
use crate::widgets::{ButtonTooltip, UiId};

/// 8 from the window's bottom, and 6 right of the Quest Log's button (48 from
/// the left, 34 across). No form round it, as for the Character button.
pub(super) static BUTTON: Template = html!(
    "trade.button",
    r#"
<button id="trade" form="hud" left="88" bottom="8" width="34" height="34">
  <img src="picture:trade"/>
</button>
"#
);

/// Marks the button.
#[derive(Component)]
pub(super) struct TradeButton;

pub(super) fn spawn_button(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut pictures: ResMut<Pictures>,
) {
    pictures.name("trade", images.add(trade_icon()));
    // Above the maps, as the other corner buttons are.
    let button = (document(&BUTTON), GlobalZIndex(15));
    commands.spawn(button).insert(bottom_strip());
}

/// Marks the button once it is built, with its tip.
pub(super) fn bind_button(new: Query<(Entity, &UiId), Added<UiId>>, mut commands: Commands) {
    for (entity, id) in &new {
        if id.parts() == ("hud", "trade") {
            let tip = ButtonTooltip("Trade".into());
            commands.entity(entity).insert((TradeButton, tip));
        }
    }
}

/// The button is hidden while connecting.
pub(super) fn show_button(
    view: Res<State<GameView>>,
    button: Option<Single<&mut Visibility, With<TradeButton>>>,
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
