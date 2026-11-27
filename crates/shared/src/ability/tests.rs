use super::*;
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::sheet::Sheet;
use crate::spell::Spellbook;

/// Whether a plain creature of this kind, carrying `carried`, can use it.
fn can_use_carrying(ability: Ability, kind: CreatureKind, carried: &[ItemKind]) -> bool {
    let mut inventory = Inventory {
        creature: kind,
        ..Inventory::default()
    };
    for (index, &item) in carried.iter().enumerate() {
        let item = Item::new(Id::for_test(index as u128 + 1), item);
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .expect("room");
    }
    ability.can_use(&inventory, &Sheet::new(kind))
}

/// Whether a plain creature of this kind, carrying nothing, can use it.
fn usable(ability: Ability, kind: CreatureKind) -> bool {
    can_use_carrying(ability, kind, &[])
}

/// A kind that knows Kick and Trample besides Attack.
fn kicker() -> CreatureKind {
    *CreatureKind::ALL
        .iter()
        .find(|kind| {
            let abilities = kind.def().abilities;
            abilities.contains(&Ability::Kick) && abilities.contains(&Ability::Trample)
        })
        .expect("some kind kicks and tramples")
}

fn slot(ability: Ability, auto: bool) -> Option<BarSlot> {
    Some(BarSlot {
        thing: ability.into(),
        auto,
    })
}

#[test]
fn every_creature_knows_attack_first() {
    for kind in CreatureKind::ALL {
        assert_eq!(
            known_by_kind(kind).first(),
            Some(&Ability::Attack),
            "{kind:?}"
        );
    }
}

#[test]
fn every_ability_is_listed_once() {
    for ability in Ability::ALL {
        assert_eq!(Ability::ALL.iter().filter(|a| **a == ability).count(), 1);
    }
}

/// A bar with only Attack, in the first slot, as from before Heal.
fn attack_only() -> ActionBar {
    let mut slots = [None; BAR_SLOTS];
    slots[0] = slot(Ability::Attack, false);
    ActionBar {
        slots,
        ..ActionBar::default()
    }
}

#[test]
fn the_heal_spell_may_be_automatic_and_a_spell_not_in_the_book_may_not() {
    assert!(BarThing::heal().can_be_automatic(&Spellbook::default()));
    assert!(!BarThing::heal().can_be_automatic(&Spellbook::empty()));
}

#[test]
fn a_new_bar_has_the_heal_spell_then_attack_then_magic() {
    let bar = ActionBar::default();
    assert_eq!(bar.slots[0], Some(BarSlot::new(BarThing::heal())));
    assert_eq!(bar.slots[1], slot(Ability::Attack, false));
    assert_eq!(bar.slots[2], slot(Ability::Magic, false));
    assert!(bar.slots[3..].iter().all(Option::is_none));
}

#[test]
fn an_old_bar_is_given_magic_once_and_taken_off_it_stays_off() {
    let mut bar = attack_only();
    bar.given = 1;
    bar.catch_up();
    assert_eq!(bar.slot_of(Ability::Magic), Some(1));
    bar.apply(
        BarChange::Clear { slot: 1 },
        &known_by_kind(CreatureKind::Human),
        &Spellbook::default(),
    )
    .unwrap();
    bar.catch_up();
    assert_eq!(bar.slot_of(Ability::Magic), None);
}

#[test]
fn an_old_bar_is_given_the_heal_spell_once_in_the_first_empty_slot() {
    let mut bar = attack_only();
    bar.given = 0;
    bar.catch_up();
    assert_eq!(bar.slots[1], Some(BarSlot::new(BarThing::heal())));
    assert_eq!(bar.slot_of(Ability::Magic), Some(2), "then Magic");
    // Taken off, it stays off.
    bar.apply(
        BarChange::Clear { slot: 1 },
        &known_by_kind(CreatureKind::Human),
        &Spellbook::default(),
    )
    .unwrap();
    bar.catch_up();
    assert_eq!(bar.slot_of(BarThing::heal()), None);
    // A bar saved without `given` reads as 0.
    let old: ActionBar =
        ron::from_str("(slots: (None, None, None, None, None, None, None, None, None, None))")
            .unwrap();
    assert_eq!(old.given, 0);
}

#[test]
fn every_ability_has_a_description() {
    for ability in Ability::ALL {
        assert!(ability.description().ends_with('.'), "{ability:?}");
    }
}

#[test]
fn passive_and_unknown_abilities_stay_off_the_bar() {
    let mut bar = attack_only();
    let place = |ability| BarChange::Place { slot: 3, ability };
    assert_eq!(
        bar.apply(
            place(Ability::NightVision),
            &known_by_kind(CreatureKind::Human),
            &Spellbook::default()
        ),
        Err(BarRefusal::Passive)
    );
    assert_eq!(
        bar.apply(
            place(Ability::Trample),
            &known_by_kind(CreatureKind::Human),
            &Spellbook::default()
        ),
        Err(BarRefusal::Unknown)
    );
    assert_eq!(
        bar.apply(
            place(Ability::Kick),
            &known_by_kind(kicker()),
            &Spellbook::default()
        ),
        Ok(())
    );
    assert_eq!(bar.slots[3], slot(Ability::Kick, false));
}

#[test]
fn placing_an_ability_already_on_the_bar_moves_it_with_its_auto() {
    let mut bar = attack_only();
    let kind = CreatureKind::Human;
    bar.apply(
        BarChange::ToggleAuto { slot: 0 },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    bar.apply(
        BarChange::Place {
            slot: 4,
            ability: Ability::Attack,
        },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slots[0], None);
    assert_eq!(bar.slots[4], slot(Ability::Attack, true));
}

#[test]
fn swapping_keeps_each_auto_and_clearing_forgets_it() {
    let mut bar = attack_only();
    let kind = kicker();
    bar.apply(
        BarChange::Place {
            slot: 1,
            ability: Ability::Kick,
        },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    bar.apply(
        BarChange::ToggleAuto { slot: 0 },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    bar.apply(
        BarChange::Swap { a: 0, b: 1 },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slots[0], slot(Ability::Kick, false));
    assert_eq!(bar.slots[1], slot(Ability::Attack, true));
    bar.apply(
        BarChange::Clear { slot: 1 },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    bar.apply(
        BarChange::Place {
            slot: 1,
            ability: Ability::Attack,
        },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(bar.slots[1], slot(Ability::Attack, false));
}

#[test]
fn only_some_abilities_can_be_made_automatic() {
    let mut bar = attack_only();
    let kind = kicker();
    bar.apply(
        BarChange::Place {
            slot: 1,
            ability: Ability::Kick,
        },
        &known_by_kind(kind),
        &Spellbook::default(),
    )
    .unwrap();
    assert_eq!(
        bar.apply(
            BarChange::ToggleAuto { slot: 1 },
            &known_by_kind(kind),
            &Spellbook::default()
        ),
        Err(BarRefusal::NotAutomatic)
    );
    assert!(Ability::Attack.can_be_automatic());
    assert!(
        bar.apply(
            BarChange::ToggleAuto { slot: 0 },
            &known_by_kind(kind),
            &Spellbook::default()
        )
        .is_ok()
    );
}

#[test]
fn bad_slots_and_empty_autos_are_refused_without_change() {
    let mut bar = attack_only();
    let before = bar.clone();
    let kind = CreatureKind::Human;
    assert_eq!(
        bar.apply(
            BarChange::Swap { a: 0, b: 10 },
            &known_by_kind(kind),
            &Spellbook::default()
        ),
        Err(BarRefusal::NoSlot)
    );
    assert_eq!(
        bar.apply(
            BarChange::ToggleAuto { slot: 5 },
            &known_by_kind(kind),
            &Spellbook::default()
        ),
        Err(BarRefusal::Empty)
    );
    assert_eq!(bar, before);
}

#[test]
fn only_known_active_abilities_can_be_used() {
    assert!(usable(Ability::Attack, CreatureKind::Human));
    // Anyone with legs kicks (owner, 2026-11-09); trampling is a kind's own.
    assert!(usable(Ability::Kick, CreatureKind::Human));
    assert!(!usable(Ability::Trample, CreatureKind::Human));
    assert!(usable(Ability::Trample, kicker()));
    assert!(!usable(Ability::NightVision, CreatureKind::Human));
}

#[test]
fn alchemy_gives_everyone_its_abilities_for_now() {
    let sheet = Sheet::new(CreatureKind::Human);
    for ability in [Ability::PressOil, Ability::Alchemy] {
        assert_eq!(ability.skill(), Some(skill::ALCHEMY));
        assert!(ability.is_known(CreatureKind::Human, &sheet), "{ability:?}");
        assert!(known(CreatureKind::Wolf, &sheet).contains(&ability));
        assert!(!known_by_kind(CreatureKind::Human).contains(&ability));
    }
    assert!(crate::skill::SkillId::find(skill::ALCHEMY).is_some());
}

#[test]
fn abilities_with_a_tool_need_it_carried() {
    let human = CreatureKind::Human;
    let tools = [(Ability::PressOil, ItemKind::OilPress)];
    for (ability, tool) in tools {
        assert_eq!(ability.tool(), Some(tool));
        assert!(!usable(ability, human), "{ability:?} without its tool");
        assert!(can_use_carrying(ability, human, &[tool]), "{ability:?}");
        // Inside a bag counts too.
        let mut inventory = Inventory::default();
        let bag = Item::new(Id::for_test(1), ItemKind::Backpack);
        let mut ids = crate::id::stand_ins();
        inventory.give(bag, StackLimits::NONE, &mut ids).unwrap();
        let inside = crate::item::Holder::Inside(Id::for_test(1));
        let tool = Item::new(Id::for_test(2), tool);
        inventory
            .add(tool, inside, 0, 0, crate::item::Rotation::None)
            .unwrap();
        assert!(ability.has_tool(&inventory), "{ability:?} in a bag");
    }
    assert!(Ability::Attack.tool().is_none());
}

#[test]
fn learned_abilities_are_known_and_saved() {
    let mut sheet = Sheet::new(CreatureKind::Human);
    assert!(!Ability::Trample.is_known(CreatureKind::Human, &sheet));
    assert!(sheet.abilities.learn(Ability::Trample));
    assert!(!sheet.abilities.learn(Ability::Trample), "learned once");
    assert!(Ability::Trample.is_known(CreatureKind::Human, &sheet));
    assert!(known(CreatureKind::Human, &sheet).contains(&Ability::Trample));
    let text = ron::to_string(&sheet.abilities).unwrap();
    assert_eq!(text, "[Trample]");
    let back: Learned = ron::from_str(&text).unwrap();
    assert_eq!(back, sheet.abilities);
}

#[test]
fn every_ability_leans_on_something() {
    for ability in Ability::ALL {
        assert!(!ability.leans().is_empty(), "{ability:?}");
    }
    use crate::sheet::{Attribute, Row};
    assert_eq!(Attribute::Perception.row(), Row::Mind);
    for row in Row::ALL {
        for attribute in row.attributes() {
            assert_eq!(attribute.row(), row);
        }
    }
}

#[test]
fn everyone_knows_sneak_a_toggle_for_the_bar() {
    assert!(Ability::Sneak.is_toggle());
    assert!(Ability::Sneak.is_active(), "it goes on the bar");
    assert!(!Ability::Sneak.can_be_automatic());
    for kind in CreatureKind::ALL {
        assert!(known_by_kind(kind).contains(&Ability::Sneak), "{kind:?}");
    }
}

#[test]
fn alchemy_needs_an_apparatus_carried() {
    let human = CreatureKind::Human;
    assert!(!usable(Ability::Alchemy, human));
    for apparatus in [ItemKind::MortarAndPestle, ItemKind::Retort] {
        assert!(can_use_carrying(Ability::Alchemy, human, &[apparatus]));
    }
    assert!(!can_use_carrying(
        Ability::Alchemy,
        human,
        &[ItemKind::Cauldron]
    ));
}

#[test]
fn alchemy_at_a_bench_needs_one_of_its_tools() {
    use crate::crafting_bench::{BenchTools, Craft};
    let human = CreatureKind::Human;
    let carried = Inventory {
        creature: human,
        ..Inventory::default()
    };
    let sheet = Sheet::new(human);
    let mut bench = BenchTools::new(Craft::Alchemy);
    assert!(!Ability::Alchemy.can_use_at(&carried, &sheet, None));
    assert!(!Ability::Alchemy.can_use_at(&carried, &sheet, Some(&bench)));
    let retort = Item::new(Id::for_test(9), ItemKind::Retort);
    bench.put(retort).expect("taken");
    assert!(Ability::Alchemy.can_use_at(&carried, &sheet, Some(&bench)));
    assert!(!Ability::PressOil.can_use_at(&carried, &sheet, Some(&bench)));
}

#[test]
fn old_grind_and_brew_slots_load_as_one_alchemy() {
    let text = "(slots: (Some((ability: Grind, auto: false)), Some((ability: Brew, auto: false)), \
        None, None, None, None, None, None, None, None), given: 2)";
    let mut bar: ActionBar = ron::from_str(text).unwrap();
    bar.catch_up();
    assert_eq!(bar.slot_of(Ability::Alchemy), Some(0));
    assert!(bar.slots[1].is_none(), "the second is cleared");
}
