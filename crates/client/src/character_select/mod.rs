//! Choosing a character: the account's characters in a grid, each drawn to the
//! same scale, the latest played first, with a "+" to make another; the one chosen
//! described on the right, with Play, Delete (after asking) and Cancel, and
//! Change User (and Link Discord, or that it is linked) at the bottom left
//! while logged in to an account. Shown while
//! `common::CharacterSelect` says; what is chosen goes to `connection` as
//! `common::CharacterChoice`. See `docs/context/client/character_select.md`.

use bevy::prelude::*;

use crate::common::CharacterSelect;

mod cells;
mod change_user;
mod chosen;
mod discord;
mod info;
mod presses;
mod requester;
mod screen;

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        let open = |select: Res<CharacterSelect>| select.open;
        app.init_resource::<chosen::Chosen>()
            .init_resource::<discord::Linking>()
            .init_resource::<discord::LinkFirst>()
            .add_systems(Startup, screen::spawn_screen)
            .add_systems(
                Update,
                (
                    (screen::bind_screen, screen::show_screen),
                    (change_user::show_change_user, discord::show_discord),
                    (
                        chosen::keep_chosen,
                        cells::fill_cells,
                        presses::press_buttons,
                        cells::show_chosen,
                        info::show_info,
                        requester::show_requester,
                    )
                        .chain()
                        .run_if(open),
                )
                    .chain(),
            );
    }
}
