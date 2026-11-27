//! A right-click on a where, effect or filter in the spell's steps takes it
//! out at once, as dragging it off the row does (its step too once empty)
//! (owner, 2026-11-27). Touch has no counterpart: dragging off is easier
//! there (owner).

use bevy::prelude::*;

use super::draft::Draft;
use super::spots::{DesignSpots, Spot};
use crate::common::UiPointer;
use crate::dom::HeldObject;

/// Takes out what was right-clicked in the steps, before anything else sees
/// the click.
pub(super) fn remove_on_right_click(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    pointer: Res<UiPointer>,
    spots: DesignSpots,
    mut draft: ResMut<Draft>,
    object: Res<HeldObject>,
) {
    if !draft.is_open() || draft.naming || object.thing().is_some() {
        return;
    }
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }
    let Some(spot) = pointer.0.and_then(|at| spots.at(at)) else {
        return;
    };
    if remove(&mut draft, spot) {
        mouse.clear_just_pressed(MouseButton::Right);
    }
}

/// Takes out the where, effect or filter at this spot; whether it was one.
pub(super) fn remove(draft: &mut Draft, spot: Spot) -> bool {
    match spot {
        Spot::Entry(step, n) => draft.remove_unit(step, n).is_some(),
        Spot::Filter(filter) => {
            draft.set_filter(filter, false);
            true
        }
        Spot::Piece(_) | Spot::Step(_) | Spot::End => false,
    }
}
