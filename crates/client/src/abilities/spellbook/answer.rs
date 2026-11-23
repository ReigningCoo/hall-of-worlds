//! What the player does on the Spells tab: the star marks a favourite or
//! not; Rename opens its requester, and Rename there asks the server for the
//! name typed (left empty, the automatic one); Remove asks first, and Remove
//! in its requester asks the server. Keep, Cancel or a press outside leaves
//! it. Each is asked at once (`SpellbookAsk`), and the list follows the
//! sheet the server sends back.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, SpellbookAsk};

use super::SpellbookTab;
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};

pub(in crate::abilities) fn answer_spellbook(
    mut events: MessageReader<UiEvent>,
    mut tab: ResMut<SpellbookTab>,
    mut to_server: MessageWriter<ToServer>,
) {
    let mut ask = |ask: SpellbookAsk| {
        to_server.write(ToServer(ClientMessage::Spellbook(ask)));
    };
    for event in events.read() {
        let ("spellbook", name) = event.id.parts() else {
            continue;
        };
        let row =
            (event.id.index()).and_then(|at| tab.listed.get(at).map(|(kept, _)| kept.clone()));
        match (name, &event.value, row) {
            ("favourite", UiValue::Pressed, Some(kept)) => ask(SpellbookAsk::Favourite {
                spell: kept.spell.clone(),
                favourite: !kept.favourite,
            }),
            ("rename", UiValue::Pressed, Some(kept)) => tab.renaming = Some(kept.spell.clone()),
            ("new_name", UiValue::Text(text), _) => tab.draft.clone_from(text),
            ("really_rename", UiValue::Pressed, _) => {
                if let Some(spell) = tab.renaming.take() {
                    let name = Some(tab.draft.clone());
                    ask(SpellbookAsk::Rename { spell, name });
                }
            }
            ("cancel_rename", UiValue::Pressed, _) | ("renaming", UiValue::Dismissed, _) => {
                tab.renaming = None;
            }
            ("remove", UiValue::Pressed, Some(kept)) => tab.asking = Some(kept.spell.clone()),
            ("really_remove", UiValue::Pressed, _) => {
                if let Some(spell) = tab.asking.take() {
                    ask(SpellbookAsk::Remove { spell });
                }
            }
            ("keep", UiValue::Pressed, _) | ("asking", UiValue::Dismissed, _) => {
                tab.asking = None;
            }
            _ => {}
        }
    }
}
