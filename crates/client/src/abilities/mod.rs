//! Abilities: the Character panel's Abilities page, every ability the character
//! knows, and the action bar either side of the Health and Mana circle. See
//! `docs/context/client/abilities.md`.

use bevy::prelude::*;

use crate::common::PointerSystems;
use crate::dom::DomInput;
use crate::dom::{DrawObjects, HoldObjects};
use crate::player::PlayerSystems;

mod bar;
mod brew;
mod cancel;
mod drag;
mod ferment;
mod grind;
mod held;
mod icons;
mod item_look;
mod manual;
mod mark;
mod own_bar;
mod page;
mod press_juice;
mod press_oil;
mod sneak;
mod spots;
mod use_ability;
mod vitals_now;
mod wait;
mod woodcutting;

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<own_bar::OwnBar>()
            .init_resource::<drag::Drag>()
            .init_resource::<drag::Aim>()
            .init_resource::<manual::Manual>()
            .init_resource::<press_oil::PressChooser>()
            .init_resource::<press_juice::JuiceChooser>()
            .init_resource::<grind::Mixing>()
            .init_resource::<brew::Brewing>()
            .init_resource::<ferment::Fermenting>()
            .add_message::<use_ability::UseSlot>()
            .add_systems(
                Startup,
                (
                    bar::spawn_bar,
                    press_oil::spawn_requester,
                    press_juice::spawn_requester,
                    grind::spawn_mixing,
                    brew::spawn_panel,
                    ferment::spawn_panel,
                ),
            )
            // Before widgets take presses on panels, and the map clicks.
            // Before the system menu sees Esc.
            .add_systems(PreUpdate, cancel::cancel_by_escape.before(DomInput::Front))
            .add_systems(
                PreUpdate,
                // Either side of `dom` moving what is dragged.
                (
                    drag::drag_abilities
                        .after(PointerSystems)
                        .before(DomInput::Presses)
                        .before(HoldObjects),
                    drag::drop_abilities.after(HoldObjects),
                ),
            )
            // Once the page and the slots have drawn themselves.
            .add_systems(PostUpdate, mark::mark_abilities.after(DrawObjects))
            .add_systems(
                Update,
                (
                    (page::fill_pages, page::name_abilities),
                    own_bar::take_bar,
                    held::use_by_key,
                    use_ability::take_timing,
                    sneak::take_sneaking,
                    // Moving first, then what was chosen by hand, then
                    // automatic abilities if nothing else is waiting.
                    cancel::cancel_by_moving,
                    manual::use_slots,
                    manual::ask_wanted,
                    use_ability::use_automatic,
                    (bar::place_bar, bar::show_bar),
                    wait::show_wait,
                    (press_oil::answer_requester, press_oil::show_requester),
                    (press_juice::answer_requester, press_juice::show_requester),
                    (grind::answer_mixing, grind::show_mixing),
                    (brew::answer_panel, brew::show_panel),
                    (ferment::answer_panel, ferment::show_panel),
                )
                    .chain()
                    // Knowing whether the player is moving this frame.
                    .after(PlayerSystems::Travel),
            );
    }
}

#[cfg(test)]
mod tests;
