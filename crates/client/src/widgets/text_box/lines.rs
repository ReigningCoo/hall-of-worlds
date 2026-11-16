//! Keys that go by lines in a box of several lines: Up and Down keep the caret
//! as near as they can to the column it was in (while pressed one after
//! another), and Home and End go to the ends of its line (Ctrl with them, to
//! the ends of all the writing, as in a box of one line).

use bevy::input::keyboard::{Key, KeyboardInput};

use super::edit::{Edited, Motion};
use super::keys::Held;
use super::spots::Spots;

/// What a key does by lines; `false` if it is not one of these, or the box
/// has not been laid out yet. `goal` is how far across Up and Down keep to.
pub(super) fn by_line(
    key: &KeyboardInput,
    held: &Held,
    edited: &mut Edited,
    spots: Option<&Spots>,
    goal: &mut Option<f32>,
) -> bool {
    let Some(spots) = spots else {
        return false;
    };
    let select = held.shift();
    match &key.logical_key {
        Key::ArrowUp | Key::ArrowDown => {
            let down = key.logical_key == Key::ArrowDown;
            let (to, across) = spots.vertical(edited.caret, *goal, down);
            edited.move_caret(Motion::To(to), select);
            *goal = Some(across);
        }
        Key::Home | Key::End if !held.word() => {
            let (first, last) = spots.line_ends(edited.caret);
            let to = if key.logical_key == Key::Home {
                first
            } else {
                last
            };
            edited.move_caret(Motion::To(to), select);
            *goal = None;
        }
        _ => return false,
    }
    true
}
