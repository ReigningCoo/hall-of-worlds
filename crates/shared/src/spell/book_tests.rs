use super::*;
use crate::skill::Skills;

/// Missile → Fire.
fn fire_missile() -> Spell {
    Spell::new(vec![
        Step::at(Where::Missile, 1),
        Step::effects(&[(Effect::Fire, 1)]),
    ])
}

fn spell_of(effect: Effect) -> Spell {
    Spell::new(vec![Step::effects(&[(effect, 1)])])
}

fn designs(book: &Spellbook) -> Vec<Spell> {
    book.iter().map(|kept| kept.spell.clone()).collect()
}

#[test]
fn a_new_book_holds_heal() {
    let book = Spellbook::default();
    assert_eq!(designs(&book), vec![heal_spell()]);
    assert_eq!(book.get(&heal_spell()).unwrap().name, None);
}

#[test]
fn an_older_sheet_loads_with_heal() {
    let book: Spellbook = ron::from_str("(spells: [])").unwrap();
    assert!(book.is_empty());
    #[derive(serde::Deserialize)]
    struct Holder {
        #[serde(default)]
        book: Spellbook,
    }
    let holder: Holder = ron::from_str("()").unwrap();
    assert!(holder.book.knows(&heal_spell()));
}

#[test]
fn stored_spells_go_first_and_never_twice() {
    let mut book = Spellbook::default();
    let skills = Skills::default();
    book.store(spell_of(Effect::Fire), Some("Burn"), Some(&skills))
        .unwrap();
    assert_eq!(designs(&book), vec![spell_of(Effect::Fire), heal_spell()]);
    assert_eq!(
        book.get(&spell_of(Effect::Fire)).unwrap().name.as_deref(),
        Some("Burn")
    );
    assert_eq!(
        book.store(heal_spell(), None, Some(&skills)),
        Err(BookRefusal::Known)
    );
}

#[test]
fn designs_are_checked() {
    let mut book = Spellbook::empty();
    let refused = book.store(fire_missile(), None, Some(&Skills::default()));
    assert_eq!(
        refused,
        Err(BookRefusal::Design(DesignError::Locked(Where::Missile)))
    );
    assert_eq!(
        book.store(Spell::default(), None, None),
        Err(BookRefusal::Design(DesignError::Empty))
    );
    assert!(book.is_empty());
}

#[test]
fn a_full_book_refuses_more() {
    let mut book = Spellbook::empty();
    for count in 1..=MOST_SPELLS as u8 {
        let spell = Spell::new(vec![Step::effects(&[(Effect::Heal, count)])]);
        book.store(spell, None, None).unwrap();
    }
    let one_more = spell_of(Effect::Fire);
    assert_eq!(book.store(one_more, None, None), Err(BookRefusal::Full));
}

#[test]
fn favourites_are_listed_first() {
    let mut book = Spellbook::default();
    book.store(spell_of(Effect::Fire), None, None).unwrap();
    book.store(spell_of(Effect::Water), None, None).unwrap();
    book.set_favourite(&heal_spell(), true).unwrap();
    let order = vec![
        heal_spell(),
        spell_of(Effect::Water),
        spell_of(Effect::Fire),
    ];
    assert_eq!(designs(&book), order);
    book.set_favourite(&heal_spell(), false).unwrap();
    assert_eq!(designs(&book)[2], heal_spell());
    let missing = book.set_favourite(&fire_missile(), true);
    assert_eq!(missing, Err(BookRefusal::NotInBook));
}

#[test]
fn renaming_and_removing() {
    let mut book = Spellbook::default();
    book.rename(&heal_spell(), Some("  Mend  ")).unwrap();
    assert_eq!(
        book.get(&heal_spell()).unwrap().name.as_deref(),
        Some("Mend")
    );
    book.rename(&heal_spell(), Some("   ")).unwrap();
    assert_eq!(book.get(&heal_spell()).unwrap().name, None);
    let long = "x".repeat(LONGEST_NAME + 1);
    assert_eq!(
        book.rename(&heal_spell(), Some(&long)),
        Err(BookRefusal::BadName)
    );
    assert_eq!(
        book.rename(&heal_spell(), Some("a\nb")),
        Err(BookRefusal::BadName)
    );
    let removed = book.remove(&heal_spell()).unwrap();
    assert_eq!(removed.spell, heal_spell());
    assert!(book.is_empty());
    assert_eq!(book.remove(&heal_spell()), Err(BookRefusal::NotInBook));
}
