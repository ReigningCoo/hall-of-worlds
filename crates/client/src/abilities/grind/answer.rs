//! What is done in the mixing panel: ticking herbs (one of each kind), choosing
//! the oil and doses, picking a mix made before, and Grind (asking the server)
//! or Cancel.

use bevy::prelude::*;
use shared::protocol::ClientMessage;

use super::Mixing;
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};

pub(in crate::abilities) fn answer_mixing(
    mut events: MessageReader<UiEvent>,
    mut mixing: ResMut<Mixing>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("grind", name) = event.id.parts() else {
            continue;
        };
        let at = event.id.index();
        match (&event.value, name, at) {
            (UiValue::Flag(on), "herb", Some(at)) => tick(&mut mixing, at, *on),
            (UiValue::Choice { index, .. }, "oil", _) => mixing.oil = *index,
            (UiValue::Choice { index, .. }, "doses", _) => mixing.doses = *index as u32 + 1,
            (UiValue::Pressed, "again", Some(at)) => again(&mut mixing, at),
            (UiValue::Pressed, "grind", _) => {
                let order = mixing.order();
                to_server.write(ToServer(ClientMessage::Grind(order)));
                mixing.open = false;
            }
            (UiValue::Pressed, "cancel", _) | (UiValue::Dismissed, _, _) => mixing.open = false,
            _ => continue,
        }
        mixing.dirty = true;
    }
}

/// Ticks or unticks the herb in row `at`: ticking one takes the place of any
/// other stack of the same herb, as only one of each counts.
fn tick(mixing: &mut Mixing, at: usize, on: bool) {
    let Some(stack) = mixing.herbs.get(at).cloned() else {
        return;
    };
    let herbs = mixing.herbs.clone();
    mixing.chosen.retain(|id| {
        let same = herbs.iter().find(|other| other.stack == *id);
        *id != stack.stack && same.is_none_or(|other| other.herb != stack.herb)
    });
    if on {
        mixing.chosen.push(stack.stack);
    }
}

/// Chooses the mix made before in row `at`: the best stack of each of its herbs
/// (the rows list them best first) and a bottle of its oil.
fn again(mixing: &mut Mixing, at: usize) {
    let Some(recipe) = mixing.recipes.get(at).cloned() else {
        return;
    };
    mixing.chosen = (recipe.herbs.iter())
        .filter_map(|key| mixing.herbs.iter().find(|stack| &stack.herb == key))
        .map(|stack| stack.stack)
        .collect();
    if let Some(oil) = mixing.oils.iter().position(|oil| oil.oil == recipe.oil) {
        mixing.oil = oil;
    }
}
