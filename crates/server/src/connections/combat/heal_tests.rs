//! Casting spells (magic plan PR 5): the Heal spell on yourself, a Touch
//! → Heal on a mob, fizzling, Mana and steps a second apart.

use shared::ability::Ability;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::{ClientMessage, DevVitals, Landed, ServerMessage};
use shared::skill::{SkillId, practice_for};
use shared::spell::{Effect, Filters, Spell, SpellKey, Step, Where, heal_spell, spell_cost};
use shared::vitals::{Points, Vital};

use super::tests::{beside, mob_health, received, run};
use crate::connections::tests::TestClient;

fn cast(spell: &Spell, target: Option<Id>) -> ClientMessage {
    ClientMessage::Spellbook(shared::protocol::SpellbookAsk::Cast {
        spell: SpellKey::of(spell),
        target,
    })
}

fn vital(test: &TestClient, vital: Vital) -> Points {
    let tick = test.world.action_clock.ticks();
    let limits = test.client.limits(tick);
    test.client.sheet.vitals.at(vital, limits, tick)
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

/// Hurts the player to 2 Health, so Health doesn't refill much meanwhile.
fn hurt(test: &mut TestClient) {
    let tick = test.world.action_clock.ticks();
    let most = test.client.limits(tick).health;
    test.send(&[ClientMessage::DevVitals(DevVitals::Hurt(most - 2))]);
}

/// Touch unlocked, and this spell in the book.
fn learn(test: &mut TestClient, spell: &Spell) {
    let magic = SkillId::find("magic").unwrap();
    let skills = &mut test.client.sheet.skills;
    skills.practise(magic, practice_for(1));
    let skills = Some(&test.client.sheet.skills).cloned();
    (test.client.sheet.spellbook)
        .store(spell.clone(), None, skills.as_ref())
        .unwrap();
}

fn touch_heal(filters: Filters) -> Spell {
    Spell::new(vec![
        Step::filtered(Where::Touch, 1, filters),
        Step::effects(&[(Effect::Heal, 1)]),
    ])
}

#[test]
fn the_heal_spell_turns_mana_into_twelve_health_at_first() {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    assert!(started(&test.send(&[cast(&heal_spell(), None)])));
    hurt(&mut test);
    let mana = vital(&test, Vital::Mana);
    run(&mut test, 30, false);
    assert_eq!(vital(&test, Vital::Health), Points::whole(14));
    assert!(vital(&test, Vital::Mana) < mana);
    assert!(received(&mut test).iter().any(|message| matches!(
        message,
        ServerMessage::AbilityLanded {
            target: None,
            landed: Landed::Healed { amount, .. },
            ..
        } if *amount == Points::whole(12)
    )));
    let healing = shared::magic::heal_skill();
    assert!(test.client.sheet.skills.practice(healing) > 0);
}

#[test]
fn healing_at_full_health_still_spends_the_mana_but_teaches_nothing() {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    let mana = vital(&test, Vital::Mana);
    assert!(started(&test.send(&[cast(&heal_spell(), None)])));
    run(&mut test, 30, false);
    let spent = mana.milli() - vital(&test, Vital::Mana).milli();
    let cost = spell_cost(&heal_spell(), None).milli();
    // Mana refills meanwhile, so a little less than all of it shows.
    assert!(spent > cost - 1000, "{spent}");
    let healing = shared::magic::heal_skill();
    assert_eq!(test.client.sheet.skills.practice(healing), 0);
}

#[test]
fn without_the_mana_it_isnt_begun() {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    let tick = test.world.action_clock.ticks();
    let most = test.client.limits(tick).mana;
    test.send(&[ClientMessage::DevVitals(DevVitals::SpendMana(most - 1))]);
    assert!(!started(&test.send(&[cast(&heal_spell(), None)])));
}

#[test]
fn a_spell_not_in_the_book_isnt_begun() {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    let spell = touch_heal(Filters::NONE);
    assert!(!started(&test.send(&[cast(&spell, None)])));
}

#[test]
fn touch_then_heal_heals_a_hurt_mob_and_practises_touch() {
    let (mut test, mob) = beside(CreatureKind::Dwarf);
    test.send(&[ClientMessage::UseAbility {
        ability: Ability::Attack,
        target: Some(mob),
    }]);
    run(&mut test, 30, false);
    let hurt = mob_health(&test, mob).unwrap();
    let spell = touch_heal(Filters::NONE);
    learn(&mut test, &spell);
    assert!(started(&test.send(&[cast(&spell, Some(mob))])));
    // The wind-up, the Touch, then the Heal a second later.
    run(&mut test, 60, false);
    assert!(mob_health(&test, mob).unwrap() > hurt);
    assert!(received(&mut test).iter().any(|message| matches!(
        message,
        ServerMessage::AbilityLanded {
            target: Some(target),
            landed: Landed::Healed { .. },
            ..
        } if *target == mob
    )));
    let touch = Where::Touch.skill();
    assert!(test.client.sheet.skills.practice(touch) > 0);
}

#[test]
fn a_first_step_hitting_nobody_fizzles_and_costs_nothing() {
    let (mut test, mob) = beside(CreatureKind::Dwarf);
    // Friend with nobody's friend about: nobody to aim at.
    let friends = Filters {
        friend: true,
        ..Filters::NONE
    };
    let spell = touch_heal(friends);
    learn(&mut test, &spell);
    assert!(started(&test.send(&[cast(&spell, Some(mob))])));
    let mana = vital(&test, Vital::Mana);
    run(&mut test, 60, false);
    assert!(vital(&test, Vital::Mana) >= mana);
    assert!(received(&mut test).iter().any(|message| matches!(
        message,
        ServerMessage::AbilityLanded {
            landed: Landed::Fizzled,
            ..
        }
    )));
}

#[test]
fn steps_resolve_a_second_apart() {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    // Healing at 64%: each Heal costs 1 Mana, so two steps cost 1 × 1.
    let healing = shared::magic::heal_skill();
    test.client.sheet.skills.practise(healing, practice_for(64));
    let twice = Spell::new(vec![
        Step::effects(&[(Effect::Heal, 1)]),
        Step::effects(&[(Effect::Heal, 1)]),
    ]);
    learn(&mut test, &twice);
    assert!(started(&test.send(&[cast(&twice, None)])));
    let healed = |test: &mut TestClient| {
        (received(test).iter())
            .filter(|message| {
                matches!(
                    message,
                    ServerMessage::AbilityLanded {
                        landed: Landed::Healed { .. },
                        ..
                    }
                )
            })
            .count()
    };
    // The wind-up is half a second (10 ticks at normal speed).
    run(&mut test, 12, false);
    assert_eq!(healed(&mut test), 1);
    run(&mut test, 12, false);
    assert_eq!(healed(&mut test), 0);
    run(&mut test, 10, false);
    assert_eq!(healed(&mut test), 1);
}
