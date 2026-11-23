use super::super::{ActionBar, BarChange, BarRefusal, known_by_kind};
use super::*;
use crate::creature::CreatureKind;
use crate::spell::Spellbook;

fn place(slot: u8, item: ItemKind) -> BarChange {
    BarChange::PlaceItem { slot, item }
}

#[test]
fn food_goes_on_the_bar_once_and_moves() {
    let mut bar = ActionBar::default();
    let kind = CreatureKind::Human;
    assert_eq!(
        bar.apply(
            place(4, ItemKind::Bread),
            &known_by_kind(kind),
            &Spellbook::default()
        ),
        Ok(())
    );
    assert_eq!(bar.slots[4], Some(BarSlot::new(ItemKind::Bread)));
    bar.apply(
        place(6, ItemKind::Bread),
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slots[4], None);
    assert_eq!(bar.slot_of(ItemKind::Bread), Some(6));
    // Over an ability, it takes its place.
    bar.apply(
        place(0, ItemKind::Bread),
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slots[0], Some(BarSlot::new(ItemKind::Bread)));
    assert_eq!(bar.slot_of(BarThing::heal()), None);
}

#[test]
fn any_food_may_be_placed_whoever_eats_it() {
    let mut bar = ActionBar::default();
    // Not a dwarf, and not one who should eat it raw: placed all the same.
    for item in [ItemKind::DwarvenBread, ItemKind::RawMeat] {
        assert!(usable_from_bar(item));
        assert_eq!(
            bar.apply(
                place(5, item),
                &known_by_kind(CreatureKind::Human),
                &Spellbook::default()
            ),
            Ok(())
        );
    }
}

#[test]
fn what_cant_be_used_stays_off_the_bar() {
    let mut bar = ActionBar::default();
    let before = bar.clone();
    let not_food = *ItemKind::ALL
        .iter()
        .find(|&&kind| !is_food(kind))
        .expect("something isn't food");
    assert_eq!(
        bar.apply(
            place(3, not_food),
            &known_by_kind(CreatureKind::Human),
            &Spellbook::default()
        ),
        Err(BarRefusal::NotUsable)
    );
    assert_eq!(
        bar.apply(
            place(10, ItemKind::Bread),
            &known_by_kind(CreatureKind::Human),
            &Spellbook::default()
        ),
        Err(BarRefusal::NoSlot)
    );
    assert_eq!(bar, before);
}

#[test]
fn items_are_never_automatic() {
    let mut bar = ActionBar::default();
    let kind = CreatureKind::Human;
    bar.apply(
        place(2, ItemKind::Bread),
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(
        bar.apply(
            BarChange::ToggleAuto { slot: 2 },
            &known_by_kind(kind),
            &Spellbook::default()
        ),
        Err(BarRefusal::NotAutomatic)
    );
    assert!(!bar.slots[2].unwrap().auto);
}

#[test]
fn swapping_and_clearing_work_on_items_too() {
    let mut bar = ActionBar::default();
    let kind = CreatureKind::Human;
    bar.apply(
        place(2, ItemKind::Bread),
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    bar.apply(
        BarChange::Swap { a: 2, b: 0 },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slot_of(ItemKind::Bread), Some(0));
    assert_eq!(bar.slot_of(BarThing::heal()), Some(2));
    bar.apply(
        BarChange::Clear { slot: 0 },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slot_of(ItemKind::Bread), None);
}

const OLD_SAVE: &str = "(slots: (Some((ability: Heal, auto: false)), \
    Some((ability: Attack, auto: true)), None, None, None, None, None, None, \
    None, None), given: 1)";

#[test]
fn a_bar_saved_before_items_still_loads_its_heal_ability_as_the_heal_spell() {
    let old: ActionBar = ron::from_str(OLD_SAVE).unwrap();
    assert_eq!(old.slots[0], Some(BarSlot::new(BarThing::heal())));
    let attack = BarSlot {
        thing: Ability::Attack.into(),
        auto: true,
    };
    assert_eq!(old.slots[1], Some(attack));
}

#[test]
fn saves_keep_the_old_shape_for_abilities_and_round_trip() {
    let mut bar: ActionBar = ron::from_str(OLD_SAVE).unwrap();
    bar.apply(
        place(3, ItemKind::RawMeat),
        &known_by_kind(CreatureKind::Human),
        &Spellbook::default(),
    )
    .unwrap();
    let saved = ron::to_string(&bar).unwrap();
    assert!(saved.contains("(ability:Attack,auto:true)"), "{saved}");
    assert!(saved.contains("(item:RawMeat)"), "{saved}");
    assert_eq!(ron::from_str::<ActionBar>(&saved).unwrap(), bar);
    // And in messages.
    let sent = postcard::to_allocvec(&bar).unwrap();
    assert_eq!(postcard::from_bytes::<ActionBar>(&sent).unwrap(), bar);
}

#[test]
fn a_particular_potion_goes_on_the_bar_and_is_saved() {
    use crate::herb::Stat;
    use crate::potion::{Contents, Dose, PotionKey};
    let mut bar = ActionBar::default();
    let known = known_by_kind(CreatureKind::Human);
    let held = Contents::new([Dose {
        stat: Stat::Mana,
        step: 5,
    }]);
    let potion = PotionKey::of(&held);
    let change = |slot| BarChange::PlacePotion { slot, potion };
    assert_eq!(bar.apply(change(3), &known, &Spellbook::default()), Ok(()));
    bar.apply(change(5), &known, &Spellbook::default()).unwrap();
    assert_eq!(bar.slot_of(potion), Some(5));
    assert_eq!(bar.slots[3], None);
    let empty = PotionKey::of(&Contents::EMPTY);
    let refused = BarChange::PlacePotion {
        slot: 1,
        potion: empty,
    };
    assert_eq!(
        bar.apply(refused, &known, &Spellbook::default()),
        Err(BarRefusal::NotUsable)
    );
    assert!(!BarThing::Potion(potion).can_be_automatic(&Spellbook::default()));
    let saved = ron::to_string(&bar).unwrap();
    assert!(saved.contains("(potion:"), "{saved}");
    assert_eq!(ron::from_str::<ActionBar>(&saved).unwrap(), bar);
    let sent = postcard::to_allocvec(&bar).unwrap();
    assert_eq!(postcard::from_bytes::<ActionBar>(&sent).unwrap(), bar);
}

#[test]
fn spells_go_on_the_bar_only_from_the_book_and_round_trip_in_saves() {
    use crate::spell::{SpellKey, heal_spell};
    let mut bar = ActionBar::default();
    let known = known_by_kind(CreatureKind::Human);
    let heal = SpellKey::of(&heal_spell());
    let other = SpellKey(heal.0 ^ 1);
    let place = |slot, spell| BarChange::PlaceSpell { slot, spell };
    let empty = Spellbook::empty();
    assert_eq!(
        bar.apply(place(5, heal), &known, &empty),
        Err(BarRefusal::Unknown)
    );
    let book = Spellbook::default();
    assert_eq!(
        bar.apply(place(5, other), &known, &book),
        Err(BarRefusal::Unknown)
    );
    // In the book, it moves from slot 0 to slot 5.
    bar.apply(place(5, heal), &known, &book).unwrap();
    assert_eq!(bar.slot_of(heal), Some(5));
    bar.apply(BarChange::ToggleAuto { slot: 5 }, &known, &book)
        .unwrap();
    let saved = ron::to_string(&bar).unwrap();
    assert!(saved.contains("spell:"), "{saved}");
    let loaded: ActionBar = ron::from_str(&saved).unwrap();
    assert_eq!(loaded, bar);
}
