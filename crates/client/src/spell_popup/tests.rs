use shared::spell::{SpellKey, Spellbook, heal_spell};

use super::popup::SPELL_POPUP;
use super::subject::{Subject, spell_of};
use crate::spell_design::PreviewSpell;

#[test]
fn the_markup_is_good() {
    SPELL_POPUP.assert_good();
}

#[test]
fn spells_are_found_in_the_book_or_the_preview() {
    let book = Spellbook::default();
    let heal = heal_spell();
    let key = SpellKey::of(&heal);
    let none = PreviewSpell::default();
    let found = spell_of(&Subject::Book(key, "1".to_owned()), &book, &none);
    assert_eq!(found, Some((heal.clone(), None)));
    let gone = Spellbook::empty();
    assert_eq!(
        spell_of(&Subject::Book(key, String::new()), &gone, &none),
        None
    );
    assert_eq!(spell_of(&Subject::Preview, &book, &none), None);
    let preview = PreviewSpell(Some((heal.clone(), Some("Mend".to_owned()))));
    assert_eq!(
        spell_of(&Subject::Preview, &book, &preview),
        Some((heal, Some("Mend".to_owned())))
    );
}
