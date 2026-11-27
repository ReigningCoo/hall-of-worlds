//! The Music panel (Play Tune): a dropdown of every tune, following the one
//! playing until one is chosen, which then plays alone, round and round;
//! a clipboard button copying its title, Attack forcing combat music on, where the tune shown
//! is played, and Reset back to music by place (`music_systems`).

use bevy::prelude::*;

use crate::dom::Template;
use crate::html;
use crate::widgets::{Pressable, TextSize, UiId, spawn_copy_button};

/// The panel's form, for its ids.
pub(super) const FORM: &str = "debug.music";

/// The panel's contents, as markup: the tunes are given by `fill_tunes`.
pub(super) static MUSIC: Template = html!(
    "debug_menu.music",
    r#"
<form id="debug.music">
  <panel id="panel" moveable>
    <span nowrap valign="middle">
      <select id="tune" searchable width="340"><option>Tune</option></select>
      <div id="copy_holder" width="34" height="34"/>
      <button id="attack">Attack</button>
    </span>
    <p id="used" width="460">.</p>
    <span nowrap align="right">
      <button id="reset">Reset</button>
    </span>
  </panel>
</form>
"#
);

/// The clipboard button (`widgets::spawn_copy_button`, which markup has no
/// element for), put in its holder once built.
#[derive(Component)]
pub(in crate::debug_menu) struct CopyTitle;

/// Puts the clipboard button into its holder as the panel is built.
pub(super) fn bind(bound: &mut EntityCommands, id: &UiId) {
    if id.parts().1 != "copy_holder" {
        return;
    }
    bound.with_children(|holder| {
        let node = Node {
            width: px(34),
            height: px(34),
            flex_shrink: 0.0,
            ..default()
        };
        spawn_copy_button(holder, "Copy the tune's title", TextSize::Medium, node).insert((
            CopyTitle,
            UiId::new(FORM, "copy"),
            Pressable,
        ));
    });
}

/// The tune chosen in the panel, playing alone until Reset (none: the
/// dropdown follows what plays).
#[derive(Resource, Default)]
pub(in crate::debug_menu) struct ChosenTune(pub(super) Option<usize>);

/// Adds the panel's systems to the debug menu's plugin.
pub(in crate::debug_menu) fn add_music_panel(app: &mut App) {
    use super::music_systems::{forget_choice, show_music, take_music_events};
    app.init_resource::<ChosenTune>().add_systems(
        Update,
        (take_music_events, forget_choice, show_music).chain(),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_panel_is_good_markup() {
        super::MUSIC.assert_good();
    }
}
