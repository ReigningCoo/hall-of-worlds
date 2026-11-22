//! What the screen's presses do: a cell chooses its character (or "+" goes on to
//! making one), Play plays the chosen one, Delete asks first, Cancel goes back
//! to the startup menu, Change User logs in again, and Link Discord links it.

use bevy::prelude::*;

use super::cells::Cell;
use super::chosen::Chosen;
use super::discord::LinkFirst;
use super::requester::Answer;
use super::screen::Button;
use crate::common::{CharacterChoice, LoginChoice};
use crate::widgets::ButtonPressed;

pub(super) fn press_buttons(
    mut pressed: MessageReader<ButtonPressed>,
    cells: Query<&Cell>,
    buttons: Query<&Button>,
    answers: Query<&Answer>,
    (mut chosen, link_first): (ResMut<Chosen>, Res<LinkFirst>),
    mut choices: MessageWriter<CharacterChoice>,
    mut login: MessageWriter<LoginChoice>,
) {
    for press in pressed.read() {
        let entity = press.0;
        if chosen.deleting {
            // Only the requester's answers while it asks.
            match answers.get(entity) {
                Ok(Answer::Delete) => {
                    if let Some(id) = chosen.id {
                        choices.write(CharacterChoice::Delete(id));
                    }
                    chosen.deleting = false;
                }
                Ok(Answer::Keep) => chosen.deleting = false,
                Err(_) => {}
            }
            continue;
        }
        if let Ok(cell) = cells.get(entity) {
            match *cell {
                Cell::Character(id) if chosen.id != Some(id) => chosen.id = Some(id),
                Cell::Character(_) => {}
                // The server would refuse it until Discord is linked.
                Cell::New if link_first.0 => {}
                Cell::New => {
                    choices.write(CharacterChoice::New);
                }
            }
            continue;
        }
        match (buttons.get(entity), chosen.id) {
            (Ok(Button::Play), Some(id)) if !link_first.0 => {
                choices.write(CharacterChoice::Play(id));
            }
            (Ok(Button::Delete), Some(_)) => chosen.deleting = true,
            (Ok(Button::Cancel), _) => {
                choices.write(CharacterChoice::Cancel);
            }
            (Ok(Button::ChangeUser), _) => {
                login.write(LoginChoice::ChangeUser);
            }
            (Ok(Button::LinkDiscord), _) => {
                login.write(LoginChoice::LinkDiscord);
            }
            _ => {}
        }
    }
}
