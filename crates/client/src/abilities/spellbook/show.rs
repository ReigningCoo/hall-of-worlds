//! Showing the Spells tab: its document put in the page when the page is
//! made, the list again whenever the book or what a spell costs changes,
//! and the requesters while a spell is renamed or its removal asked about.

use bevy::prelude::*;
use shared::lang::Lang;
use shared::spell::{BookSpell, SpellKey, auto_name, spell_cost};

use super::markup::SPELLBOOK;
use super::{FORM, SpellbookTab, spell_id};
use crate::character::CharacterSheet;
use crate::common::{SpellsPage, Wording};
use crate::dom::{Dom, document_in};

/// Puts the tab's document in each new Spells page.
pub(in crate::abilities) fn spawn_spellbook(
    pages: Query<Entity, Added<SpellsPage>>,
    mut tab: ResMut<SpellbookTab>,
    mut commands: Commands,
) {
    for page in &pages {
        tab.document = Some(commands.spawn(document_in(&SPELLBOOK, page)).id());
    }
}

/// The list, drawn again when the book, the Mana spells cost or the words
/// change, or into a new document; and the requesters.
pub(in crate::abilities) fn show_spellbook(
    mut tab: ResMut<SpellbookTab>,
    (sheet, wording): (Res<CharacterSheet>, Res<Wording>),
    mut dom: Dom,
) {
    let Some(document) = tab.document else {
        return;
    };
    let skills = Some(&sheet.0.skills);
    let book: Vec<(BookSpell, u32)> = (sheet.0.spellbook.iter())
        .map(|kept| (kept.clone(), spell_cost(&kept.spell, skills).shown()))
        .collect();
    let fresh = tab.drawn_in != Some(document);
    if fresh || tab.listed != book || wording.is_changed() {
        let list = format!("{FORM}/spells");
        dom.empty(&list);
        dom.set_flag(&format!("{FORM}/empty"), "hidden", !book.is_empty());
        dom.set_flag(&format!("{FORM}/scroll"), "hidden", book.is_empty());
        for (at, (kept, cost)) in book.iter().enumerate() {
            dom.copy_template(&format!("{FORM}/spell"), &list, at);
            show_row(&mut dom, at, (kept, *cost), &wording.0);
        }
        tab.listed = book;
        tab.drawn_in = Some(document);
    }
    let now = (tab.renaming.clone(), tab.asking.clone());
    if fresh || tab.shown != now {
        show_renaming(&mut dom, &mut tab, &wording.0);
        show_asking(&mut dom, tab.asking.as_ref(), &sheet, &wording.0);
        tab.shown = now;
    }
}

/// One row: the spell's picture, its name, what it costs (and the automatic
/// name, if it has its own), and whether it is a favourite.
fn show_row(dom: &mut Dom, at: usize, (kept, cost): (&BookSpell, u32), lang: &Lang) {
    let id = |name: &str| format!("{FORM}/{name}.{at}");
    let key = SpellKey::of(&kept.spell);
    dom.set(&id("picture"), "for", &spell_id(key));
    let auto = auto_name(&kept.spell, lang);
    let (name, about) = match &kept.name {
        Some(own) => (own.clone(), format!("{auto}, {cost} Mana")),
        None => (auto, format!("{cost} Mana")),
    };
    dom.set_text(&id("name"), name);
    dom.set_text(&id("about"), about);
    dom.set_flag(&id("favourite"), "selected", kept.favourite);
    let tip = if kept.favourite {
        "A favourite: click to unmark"
    } else {
        "Mark as a favourite"
    };
    dom.set(&id("favourite"), "title", tip);
}

/// The requester for a new name, up while a spell is renamed, its box filled
/// with the spell's own name on opening, the automatic one in grey.
fn show_renaming(dom: &mut Dom, tab: &mut SpellbookTab, lang: &Lang) {
    dom.set_flag(
        &format!("{FORM}/renaming"),
        "hidden",
        tab.renaming.is_none(),
    );
    let Some(spell) = &tab.renaming else {
        return;
    };
    let own = (tab.listed.iter())
        .find(|(kept, _)| kept.spell == *spell)
        .and_then(|(kept, _)| kept.name.clone())
        .unwrap_or_default();
    let field = format!("{FORM}/new_name");
    dom.set(&field, "value", &own);
    dom.set(&field, "placeholder", &auto_name(spell, lang));
    tab.draft = own;
}

/// The requester asking before a spell is removed, up while one is asked
/// about.
fn show_asking(
    dom: &mut Dom,
    asking: Option<&shared::spell::Spell>,
    sheet: &CharacterSheet,
    lang: &Lang,
) {
    dom.set_flag(&format!("{FORM}/asking"), "hidden", asking.is_none());
    let Some(spell) = asking else {
        return;
    };
    let name = (sheet.0.spellbook.get(spell))
        .and_then(|kept| kept.name.clone())
        .unwrap_or_else(|| auto_name(spell, lang));
    let question = format!(
        "Take {name} out of your spellbook? Its steps go back to the spell design panel, to change and save again."
    );
    dom.set_text(&format!("{FORM}/question"), question);
}
