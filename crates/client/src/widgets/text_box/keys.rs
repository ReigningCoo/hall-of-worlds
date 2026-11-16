//! Who has the keyboard, and what keys do to their box.

use bevy::input::ButtonState as KeyState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use super::edit::{Edited, Motion};
use super::layout::Labels;
use super::lines::by_line;
use super::{Cursor, PressedThisFrame, SoftKeyPressed, TextBox, TextEdited, TextFocus, allowed};
use crate::common::{Action, Preferences};
use crate::widgets::{ButtonPressed, clipboard, soft_keyboard};

/// The modifier keys held, followed from the key events themselves, as a box
/// with the keyboard clears the pressed keys every frame.
#[derive(Resource, Default)]
pub(in crate::widgets) struct Held(Vec<KeyCode>);

impl Held {
    fn follow(&mut self, key: &KeyboardInput) {
        let modifier = matches!(
            key.key_code,
            KeyCode::ControlLeft
                | KeyCode::ControlRight
                | KeyCode::SuperLeft
                | KeyCode::SuperRight
                | KeyCode::ShiftLeft
                | KeyCode::ShiftRight
                | KeyCode::AltLeft
                | KeyCode::AltRight
        );
        if modifier {
            self.0.retain(|held| *held != key.key_code);
            if key.state == KeyState::Pressed {
                self.0.push(key.key_code);
            }
        }
    }

    fn any(&self, keys: [KeyCode; 2]) -> bool {
        self.0.iter().any(|held| keys.contains(held))
    }

    /// Ctrl, or Cmd on a Mac: with C, X, V and A. Not AltGr, which Windows
    /// sends as Ctrl with right Alt, and which types letters such as @ and €.
    fn command(&self) -> bool {
        let alt_gr = self.any([KeyCode::AltRight, KeyCode::AltRight]);
        (self.any([KeyCode::ControlLeft, KeyCode::ControlRight]) && !alt_gr)
            || self.any([KeyCode::SuperLeft, KeyCode::SuperRight])
    }

    pub(super) fn shift(&self) -> bool {
        self.any([KeyCode::ShiftLeft, KeyCode::ShiftRight])
    }

    /// Ctrl, or Option on a Mac: moves and deletes by words.
    pub(super) fn word(&self) -> bool {
        self.any([KeyCode::ControlLeft, KeyCode::ControlRight])
            || self.any([KeyCode::AltLeft, KeyCode::AltRight])
    }

    /// Cmd: with Left and Right, to the start or end, as on a Mac.
    fn line(&self) -> bool {
        self.any([KeyCode::SuperLeft, KeyCode::SuperRight])
    }
}

/// What the box says with its caret, as last left: at the end if its owner
/// changed the writing since.
pub(super) fn current(text_box: &TextBox, cursor: &Cursor) -> Edited {
    let mut edited = Edited::at_end(&text_box.text, text_box.accepts_items);
    edited.limit = text_box.max_length;
    if cursor.text != text_box.text {
        return edited;
    }
    let len = edited.caret;
    edited.caret = cursor.caret.min(len);
    edited.anchor = cursor.anchor.min(len);
    edited
}

/// Keeps an edit; `true` if the writing changed. `now` is when, to keep the
/// caret solid a while if it moved.
pub(super) fn keep(edited: Edited, text_box: &mut TextBox, cursor: &mut Cursor, now: f32) -> bool {
    let moved = (cursor.caret, cursor.anchor) != (edited.caret, edited.anchor);
    let changed = edited.text != text_box.text;
    if moved || changed {
        cursor.moved_at = now;
    }
    (cursor.caret, cursor.anchor) = (edited.caret, edited.anchor);
    if changed {
        text_box.text.clone_from(&edited.text);
    }
    cursor.text = edited.text;
    changed
}

/// What a box lets keys do, from its `TextBox`.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Rules {
    pub digits_only: bool,
    pub multiline: bool,
    pub read_only: bool,
}

impl Rules {
    fn of(text_box: &TextBox) -> Self {
        Self {
            digits_only: text_box.digits_only,
            multiline: text_box.multiline,
            read_only: text_box.read_only,
        }
    }
}

/// What one key does to the box; `false` if it is not one of ours. A box
/// that is read only still moves, selects and copies.
fn press(key: &KeyboardInput, held: &Held, edited: &mut Edited, rules: Rules) -> bool {
    let select = held.shift();
    let motion = |plain, word| if held.word() { word } else { plain };
    let keeps = |typed: &str| allowed(typed, rules.digits_only, rules.multiline);
    let edits = !rules.read_only;
    match &key.logical_key {
        Key::ArrowLeft if held.line() => edited.move_caret(Motion::Home, select),
        Key::ArrowRight if held.line() => edited.move_caret(Motion::End, select),
        Key::ArrowLeft => edited.move_caret(motion(Motion::Left, Motion::WordLeft), select),
        Key::ArrowRight => edited.move_caret(motion(Motion::Right, Motion::WordRight), select),
        Key::Home => edited.move_caret(Motion::Home, select),
        Key::End => edited.move_caret(Motion::End, select),
        Key::Backspace if edits => edited.backspace(held.word()),
        Key::Delete if edits => edited.delete(held.word()),
        Key::Enter if edits && rules.multiline => edited.insert("\n"),
        _ if held.command() => match key.key_code {
            KeyCode::KeyA => edited.select_all(),
            KeyCode::KeyC => copy(&edited.selected()),
            KeyCode::KeyX if edits => copy(&edited.cut()),
            KeyCode::KeyV if edits => {
                if let Some(pasted) = clipboard::paste() {
                    edited.insert(&keeps(&pasted));
                }
            }
            // Other shortcuts type nothing.
            _ => {}
        },
        Key::Space if edits => edited.insert(&keeps(" ")),
        Key::Character(typed) if edits => edited.insert(&keeps(typed)),
        _ => return false,
    }
    true
}

fn copy(text: &str) {
    if !text.is_empty() {
        clipboard::copy_to_clipboard(text);
    }
}

/// Who has the keyboard, and what they type.
#[allow(clippy::too_many_arguments)]
pub(in crate::widgets) fn take_text(
    mut focus: ResMut<TextFocus>,
    mut boxes: Query<(&mut TextBox, &mut Cursor)>,
    mut pressed: MessageReader<ButtonPressed>,
    noted: Res<PressedThisFrame>,
    mut typed: MessageReader<KeyboardInput>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut edited_boxes: MessageWriter<TextEdited>,
    mut soft_keys: MessageWriter<SoftKeyPressed>,
    mut told_soft_keyboard: Local<(Option<Entity>, Edited)>,
    mut held: ResMut<Held>,
    (preferences, labels): (Res<Preferences>, Labels),
    time: Res<Time<Real>>,
) {
    let now = time.elapsed_secs();
    let on_box = pressed
        .read()
        .map(|press| press.0)
        .filter(|&entity| boxes.contains(entity))
        .last();
    let holds_keys = |focus: &TextFocus, boxes: &Query<(&mut TextBox, &mut Cursor)>| {
        focus
            .0
            .and_then(|entity| boxes.get(entity).ok())
            .is_some_and(|(text_box, _)| text_box.takes_keys)
    };
    if on_box.is_some() {
        focus.0 = on_box;
    } else if noted.0 && holds_keys(&focus, &boxes) {
        focus.0 = None;
    }
    let typed: Vec<KeyboardInput> = typed.read().cloned().collect();
    let soft: Vec<_> = soft_keyboard::take_keys();
    for &key in &soft {
        soft_keys.write(SoftKeyPressed(key));
    }
    let pasted = clipboard::arrived();
    let Some(entity) = focus.0 else {
        typed.iter().for_each(|key| held.follow(key));
        return;
    };
    let Ok((mut text_box, mut cursor)) = boxes.get_mut(entity) else {
        focus.0 = None;
        return;
    };

    let rules = Rules::of(&text_box);
    // Return in the browser's input starts a new line in a box of several
    // lines (the line break arrives with what is typed there).
    let escape = soft_keyboard::Key::Escape;
    let gave_up = (soft.iter()).any(|&key| !rules.multiline || key == escape);
    let mut edited = current(&text_box, &cursor);
    let before = edited.clone();
    let spots = rules
        .multiline
        .then(|| labels.spots(entity, &edited))
        .flatten();
    let (controls, mut goal) = (&preferences.controls, cursor.goal);
    for key in &typed {
        held.follow(key);
        // The debug menu's key opens and closes it.
        if key.state == KeyState::Pressed && !controls.uses_key(Action::DebugMenu, key.key_code) {
            let lines =
                rules.multiline && by_line(key, &held, &mut edited, spots.as_ref(), &mut goal);
            if !lines && press(key, &held, &mut edited, rules) {
                goal = None;
            }
        }
    }
    cursor.goal = goal;
    if let Some(pasted) = pasted
        && !rules.read_only
    {
        edited.insert(&allowed(&pasted, rules.digits_only, rules.multiline));
    }
    if edited == before
        && !rules.read_only
        && let Some((typed, caret)) = soft_keyboard::typed(&edited.text)
    {
        // The browser's hidden input, typed into on a device without keys.
        // It counts characters, not places. What the box doesn't keep is put
        // right there too.
        edited.text = text_box.keeps(&typed);
        if edited.text != typed {
            told_soft_keyboard.0 = None;
        }
        let byte = edited.text.char_indices().nth(caret);
        edited.caret = edited.place_of_byte(byte.map_or(edited.text.len(), |(at, _)| at));
        edited.anchor = edited.caret;
    }
    if told_soft_keyboard.0 != Some(entity) || told_soft_keyboard.1 != edited {
        let chars = |place| edited.text[..edited.byte(place)].chars().count();
        soft_keyboard::show_text(&edited.text, chars(edited.anchor), chars(edited.caret));
        *told_soft_keyboard = (Some(entity), edited.clone());
    }
    if keep(edited, &mut text_box, &mut cursor, now) {
        edited_boxes.write(TextEdited(entity));
    }
    if text_box.takes_keys {
        let enter = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter);
        let done = (enter && !rules.multiline) || keys.just_pressed(KeyCode::Escape) || gave_up;
        if done {
            focus.0 = None;
        }
        keys.reset_all();
    }
}

#[cfg(test)]
mod tests;
