//! The spell design panel (magic plan PR 9, `docs/context/magic-system.md`):
//! opened by the Magic ability, it builds a spell from the wheres unlocked,
//! the filters and the effects along the top, clicked or dragged down into a
//! row of steps; a step holds wheres (the same one, as often as it was added,
//! and in the first step the filters) or effects in order, never both, with
//! no counts: one used twice is added twice (owner, 2026-11-23). Steps, and
//! the wheres and effects in them, are moved by dragging, and dragged off to
//! take them out. What it costs and
//! what is wrong show as it is built (`shared::spell`, the server's own
//! rules), and Save keeps it in the spellbook with a name or the automatic
//! one. A spell taken out of the book (`EditSpell`) comes back here to be
//! changed. See `docs/context/client/spell_design.md`.

#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod draft;
mod drag;
mod markup;
mod nodes;
mod octagon;
mod open;
mod piece;
mod places;
mod rows;
mod side;
mod spots;
mod steps;
mod units;
mod words;

use bevy::prelude::*;
use shared::spell::BookSpell;

use crate::common::PointerSystems;
use crate::dom::{DomInput, HoldObjects};

/// Opens the panel, or closes it if it is open (the Magic ability used, by
/// `abilities`).
#[derive(Message, Clone, Copy, Debug, Default)]
pub struct OpenSpellDesign;

/// Puts a spell taken out of the spellbook on the panel to be changed, in
/// place of the design there, opening the panel if closed (`abilities`, on
/// Remove).
#[derive(Message, Clone, Debug)]
pub struct EditSpell(pub BookSpell);

pub struct SpellDesignPlugin;

impl Plugin for SpellDesignPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<octagon::OctagonMaterial>::default())
            .init_resource::<draft::Draft>()
            .init_resource::<drag::DesignDrag>()
            .init_resource::<drag::DesignAim>()
            .add_message::<OpenSpellDesign>()
            .add_message::<EditSpell>()
            .add_systems(
                PreUpdate,
                (
                    drag::press_design
                        .after(PointerSystems)
                        .after(DomInput::Find)
                        .before(DomInput::Presses)
                        .before(HoldObjects),
                    drag::drop_on_design.after(HoldObjects),
                ),
            )
            .add_systems(
                Update,
                (
                    open::open_panel,
                    open::bind_panel,
                    open::notice_closed,
                    open::tell_open,
                    side::answer_panel,
                    rows::draw_rows,
                    octagon::give_octagons,
                    side::show_side,
                )
                    .chain(),
            );
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
