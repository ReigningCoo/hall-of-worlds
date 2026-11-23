//! Under the steps: the spell's name (its own kept from the book, and the
//! automatic one), what it costs, what is wrong or why it can't be saved, and
//! Save (asking for a name) and Clear. Every figure is `shared::spell`'s,
//! as the server checks it (`Spellbook::store` on the client's own copy).

use bevy::prelude::*;
use shared::lang::Lang;
use shared::protocol::{ClientMessage, SpellbookAsk};
use shared::sheet::Sheet;
use shared::spell::{BookRefusal, auto_name, spell_cost, tidy_name, unlocked_wheres};

use super::draft::Draft;
use super::markup::WORDS;
use super::words::{cost_words, next_words, problem_words};
use crate::character::CharacterSheet;
use crate::common::Wording;
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};

/// Whether the design on the panel could be stored now, else why not.
pub(super) fn can_store(draft: &Draft, sheet: &Sheet) -> Result<(), String> {
    if draft.filters_stranded() {
        return Err("Filters need a where in the first step to filter.".to_owned());
    }
    let mut book = sheet.spellbook.clone();
    match book.store(draft.spell(), None, Some(&sheet.skills)) {
        Ok(()) => Ok(()),
        Err(BookRefusal::Design(error)) => Err(problem_words(error)),
        Err(BookRefusal::Known) => Err("This spell is in your spellbook.".to_owned()),
        Err(BookRefusal::Full) => Err("Your spellbook is full: remove a spell first.".to_owned()),
        Err(BookRefusal::NotInBook | BookRefusal::BadName) => Err(String::new()),
    }
}

/// Save, the requester's buttons and name, and Clear.
pub(super) fn answer_panel(
    mut events: MessageReader<UiEvent>,
    mut draft: ResMut<Draft>,
    sheet: Res<CharacterSheet>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("spell_design", name) = event.id.parts() else {
            continue;
        };
        match (name, &event.value) {
            ("save", UiValue::Pressed) if can_store(&draft, &sheet.0).is_ok() => {
                draft.typed = draft.name.clone().unwrap_or_default();
                draft.naming = true;
            }
            ("typed", UiValue::Text(text)) => draft.typed.clone_from(text),
            ("really_save", UiValue::Pressed) => {
                let Ok(name) = tidy_name(Some(&draft.typed)) else {
                    continue;
                };
                if can_store(&draft, &sheet.0).is_ok() {
                    let spell = draft.spell();
                    let ask = SpellbookAsk::Store {
                        spell,
                        name: name.clone(),
                    };
                    to_server.write(ToServer(ClientMessage::Spellbook(ask)));
                    draft.name = name;
                }
                draft.naming = false;
            }
            ("cancel_save", UiValue::Pressed) | ("naming", UiValue::Dismissed) => {
                draft.naming = false;
            }
            ("clear", UiValue::Pressed) => draft.clear(),
            _ => {}
        }
    }
}

/// Says what the design costs and whether it can be saved, whenever it, the
/// skills, the book or the words change; and the requester on opening.
pub(super) fn show_side(
    draft: Res<Draft>,
    (sheet, wording): (Res<CharacterSheet>, Res<Wording>),
    mut dom: Dom,
    mut shown: Local<Option<(Entity, bool)>>,
) {
    let Some(document) = draft.document else {
        *shown = None;
        return;
    };
    let fresh = shown.is_none_or(|(drawn, _)| drawn != document);
    if !(fresh || draft.is_changed() || sheet.is_changed() || wording.is_changed()) {
        return;
    }
    let id = |name: &str| format!("spell_design/{name}");
    let skills = &sheet.0.skills;
    dom.set_text(&id("words"), WORDS);
    let unlocked = unlocked_wheres(Some(skills));
    dom.set_text(&id("next"), next_words(unlocked, skills));
    let spell = draft.spell();
    let lang: &Lang = &wording.0;
    let named = match (&draft.name, spell.steps.is_empty()) {
        (_, true) => String::new(),
        (Some(own), false) => format!("{own} ({})", auto_name(&spell, lang)),
        (None, false) => auto_name(&spell, lang),
    };
    dom.set_text(&id("name"), named);
    let total = spell_cost(&spell, Some(skills)).shown();
    dom.set_text(&id("cost"), cost_words(&spell, skills, total));
    let ready = can_store(&draft, &sheet.0);
    let problem = match &ready {
        Ok(()) => "Ready to save.".to_owned(),
        Err(why) => why.clone(),
    };
    dom.set_text(&id("problem"), problem);
    dom.set_flag(&id("save"), "disabled", ready.is_err());
    dom.set_flag(&id("clear"), "disabled", draft.steps.is_empty());
    let opening = draft.naming && (fresh || shown.is_none_or(|(_, was)| !was));
    if opening {
        dom.set(&id("typed"), "value", &draft.typed);
        dom.set(&id("typed"), "placeholder", &auto_name(&spell, lang));
    }
    dom.set_flag(&id("naming"), "hidden", !draft.naming);
    *shown = Some((document, draft.naming));
}
