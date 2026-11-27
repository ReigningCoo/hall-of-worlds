//! Casting a design as it is (the spell design panel's Cast): kept in the book
//! or not, by the same checks as a spell in the book.

use shared::ability::Ability;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::{ClientMessage, ServerMessage, SpellbookAsk};
use shared::spell::{Effect, Spell, Step, Where, heal_spell};

use super::tests::beside;

fn cast_design(spell: &Spell, target: Option<Id>) -> ClientMessage {
    ClientMessage::Spellbook(SpellbookAsk::CastDesign {
        spell: spell.clone(),
        target,
    })
}

fn started(answers: &[ServerMessage]) -> bool {
    answers.iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityStarted {
                who: None,
                ability: Ability::Magic,
                ..
            }
        )
    })
}

#[test]
fn a_design_not_in_the_book_is_cast() {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    let book = &mut test.client.sheet.spellbook;
    book.remove(&heal_spell()).unwrap();
    assert!(
        book.by_key(shared::spell::SpellKey::of(&heal_spell()))
            .is_none()
    );
    assert!(started(&test.send(&[cast_design(&heal_spell(), None)])));
}

#[test]
fn a_design_the_skills_dont_allow_is_refused() {
    // With no Magic, no where is unlocked yet.
    let (mut test, mob) = beside(CreatureKind::Dwarf);
    let touch = Spell::new(vec![
        Step::at(Where::Touch, 1),
        Step::effects(&[(Effect::Heal, 1)]),
    ]);
    assert!(shared::spell::check(&touch, Some(&test.client.sheet.skills)).is_err());
    assert!(!started(&test.send(&[cast_design(&touch, Some(mob))])));
}
