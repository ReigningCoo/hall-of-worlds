//! What the player does on the Spells tab: Design a Spell opens the spell
//! design panel (`spell_design::OpenSpellDesign`; nothing if it is open
//! already, as that would close it); the star marks a favourite or
//! not; Rename opens its requester, and Rename there asks the server for the
//! name typed (left empty, the automatic one); Remove asks first, and Remove
//! in its requester asks the server and puts the spell on the spell design
//! panel to be changed (`spell_design::EditSpell`). Keep, Cancel or a press
//! outside leaves it. Each is asked at once (`SpellbookAsk`), and the list follows the
//! sheet the server sends back.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::protocol::{ClientMessage, SpellbookAsk};

use super::SpellbookTab;
use crate::common::OpenPanels;
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};
use crate::spell_design::{EditSpell, OpenSpellDesign};

pub(in crate::abilities) fn answer_spellbook(
    mut events: MessageReader<UiEvent>,
    mut tab: ResMut<SpellbookTab>,
    mut to_server: MessageWriter<ToServer>,
    (mut edit, mut design): (MessageWriter<EditSpell>, MessageWriter<OpenSpellDesign>),
    panels: Res<OpenPanels>,
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
            ("design", UiValue::Pressed, _) => {
                if !panels.is_open(Ability::Magic) {
                    design.write(OpenSpellDesign);
                }
            }
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
                    let kept = tab.listed.iter().find(|(kept, _)| kept.spell == spell);
                    if let Some((kept, _)) = kept {
                        edit.write(EditSpell(kept.clone()));
                    }
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
