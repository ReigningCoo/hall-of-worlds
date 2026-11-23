//! The spellbook through a connected client (magic plan PR 4): a new
//! character's holds Heal; designs stored only when unlocked; renamed,
//! favourited and removed; every ask answered with the sheet.

use shared::protocol::{ClientMessage, ServerMessage, SpellbookAsk};
use shared::skill::SkillId;
use shared::spell::{Effect, Spell, Step, Where, heal_spell};

use super::tests::TestClient;

/// Missile → Fire.
fn fire_missile() -> Spell {
    Spell::new(vec![
        Step::at(Where::Missile, 1),
        Step::effects(&[(Effect::Fire, 1)]),
    ])
}

fn ask(ask: SpellbookAsk) -> ClientMessage {
    ClientMessage::Spellbook(ask)
}

fn told_sheet(answers: &[ServerMessage]) -> bool {
    answers
        .iter()
        .any(|answer| matches!(answer, ServerMessage::Sheet(_)))
}

#[test]
fn a_new_character_knows_heal() {
    let test = TestClient::new();
    assert!(test.client.sheet.spellbook.knows(&heal_spell()));
}

#[test]
fn a_locked_design_is_refused_until_magic_unlocks_it() {
    let mut test = TestClient::new();
    let store = SpellbookAsk::Store {
        spell: fire_missile(),
        name: Some("Firebolt".to_owned()),
    };
    let answers = test.send(&[ask(store.clone())]);
    assert!(told_sheet(&answers), "answered either way");
    assert!(!test.client.sheet.spellbook.knows(&fire_missile()));
    let magic = SkillId::find("magic").unwrap();
    test.client.sheet.skills.set_percent(magic, 50);
    test.send(&[ask(store)]);
    let kept = test.client.sheet.spellbook.get(&fire_missile()).unwrap();
    assert_eq!(kept.name.as_deref(), Some("Firebolt"));
}

#[test]
fn renaming_favouring_and_removing() {
    let mut test = TestClient::new();
    let spell = heal_spell();
    test.send(&[
        ask(SpellbookAsk::Rename {
            spell: spell.clone(),
            name: Some("Mend".to_owned()),
        }),
        ask(SpellbookAsk::Favourite {
            spell: spell.clone(),
            favourite: true,
        }),
    ]);
    let kept = test.client.sheet.spellbook.get(&spell).unwrap();
    assert_eq!(kept.name.as_deref(), Some("Mend"));
    assert!(kept.favourite);
    test.send(&[ask(SpellbookAsk::Remove { spell })]);
    assert!(test.client.sheet.spellbook.is_empty());
}
