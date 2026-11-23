//! Void, Fire and Water (magic plan PR 11): fire harms what it hits, Void
//! turns Heal into harm that never takes the caster below 1 Health, Water
//! wets and fire then does half and dries, and Water puts out a campfire.

use shared::campfire::Campfire;
use shared::creature::CreatureKind;
use shared::dungeon::Cell;
use shared::material::Material;
use shared::protocol::{ClientMessage, Landed, ServerMessage, SpellbookAsk};
use shared::skill::{SkillId, practice_for};
use shared::spell::{
    Effect, Outcome, Part, Spell, SpellKey, Step, WET, WET_FIRE, Where, harm_before_resisting,
    resistance, resisted, spell_power,
};
use shared::vitals::{Points, Vital};

use super::tests::{beside, mob_health, received, run};
use crate::connections::tests::TestClient;

/// Stores `spell` in the player's book and casts it at `target`, letting
/// ticks go by: everything the player was told.
pub(super) fn cast(
    test: &mut TestClient,
    spell: &Spell,
    target: Option<shared::id::Id>,
) -> Vec<ServerMessage> {
    let skills = test.client.sheet.skills.clone();
    let book = &mut test.client.sheet.spellbook;
    let _ = book.store(spell.clone(), None, Some(&skills));
    test.client.sheet.vitals.fill();
    let mut told = test.send(&[ClientMessage::Spellbook(SpellbookAsk::Cast {
        spell: SpellKey::of(spell),
        target,
    })]);
    for _ in 0..80 {
        run(test, 1, false);
        told.extend(received(test));
    }
    told
}

/// Touch unlocked (Magic at 1%).
pub(super) fn unlock_touch(test: &mut TestClient) {
    let magic = SkillId::find("magic").unwrap();
    test.client.sheet.skills.practise(magic, practice_for(1));
}

/// A player in a castle with a mob beside them, Touch unlocked.
pub(super) fn magician() -> TestClient {
    let (mut test, _) = beside(CreatureKind::Dwarf);
    unlock_touch(&mut test);
    test
}

pub(super) fn on_self(effects: &[(Effect, u8)]) -> Spell {
    Spell::new(vec![Step::effects(effects)])
}

pub(super) fn health(test: &TestClient) -> Points {
    let tick = test.world.action_clock.ticks();
    let limits = test.client.limits(tick);
    test.client.sheet.vitals.at(Vital::Health, limits, tick)
}

#[test]
fn touch_and_fire_harms_the_mob_beside() {
    let (mut test, mob) = beside(CreatureKind::Dwarf);
    unlock_touch(&mut test);
    let full = mob_health(&test, mob).unwrap();
    let spell = Spell::new(vec![
        Step::at(Where::Touch, 1),
        Step::effects(&[(Effect::Fire, 1)]),
    ]);
    let told = cast(&mut test, &spell, Some(mob));
    let hit = told.iter().any(|message| {
        matches!(message, ServerMessage::AbilityLanded {
            target: Some(target),
            landed: Landed::Hit { harm, .. },
            ..
        } if *target == mob && *harm > Points::ZERO)
    });
    assert!(hit, "told of the harm");
    assert!(mob_health(&test, mob).unwrap() < full);
}

#[test]
fn void_heal_harms_the_caster_but_never_below_one() {
    let mut test = magician();
    let told = cast(
        &mut test,
        &on_self(&[(Effect::Void, 1), (Effect::Heal, 2)]),
        None,
    );
    assert!(harm_to_self(&told) > Points::ZERO, "harmed");
    let after = health(&test);
    assert!(after >= Points::whole(1), "never below 1 Health");
}

/// The harm the caster's spell did them, as they were told.
fn harm_to_self(told: &[ServerMessage]) -> Points {
    (told.iter())
        .find_map(|message| match message {
            ServerMessage::AbilityLanded {
                target: None,
                landed: Landed::Hit { harm, .. },
                ..
            } => Some(*harm),
            _ => None,
        })
        .unwrap_or(Points::ZERO)
}

/// What one Fire on themselves would harm the caster now, dry.
fn fire_on_self(test: &TestClient) -> u64 {
    let fire = Outcome::Part(Part::plain(Effect::Fire));
    let (attributes, skills) = (&test.client.bonused.attributes, &test.client.sheet.skills);
    let before = harm_before_resisting(fire, 1, Some(skills));
    let power = spell_power(fire, attributes, Some(skills));
    u64::from(resisted(before, power, resistance(attributes, Some(skills))).milli())
}

#[test]
fn water_wets_and_fire_on_the_wet_does_half_and_dries() {
    let mut test = magician();
    let dry = fire_on_self(&test);
    let told = cast(&mut test, &on_self(&[(Effect::Fire, 1)]), None);
    assert_eq!(u64::from(harm_to_self(&told).milli()), dry);
    let _ = cast(&mut test, &on_self(&[(Effect::Water, 1)]), None);
    let tick = test.world.action_clock.ticks();
    assert!(test.client.sheet.under_spell(WET, tick), "wet");
    let dry = fire_on_self(&test);
    let wet = harm_to_self(&cast(&mut test, &on_self(&[(Effect::Fire, 1)]), None));
    assert_eq!(
        u64::from(wet.milli()),
        dry * WET_FIRE / 1000,
        "half while wet"
    );
    let tick = test.world.action_clock.ticks();
    assert!(!test.client.sheet.under_spell(WET, tick), "dried");
}

#[test]
fn water_puts_out_a_campfire_underfoot() {
    let mut test = magician();
    let visit = test.client.visit.as_ref().unwrap();
    let key = visit.plant_key(visit.position().0, visit.position().1);
    let now = crate::sites::campfires::fire_now(&test.world);
    crate::sites::campfires::keep_campfire(
        &mut test.world,
        key,
        Some(Campfire::lit(now, Material::Oak)),
    );
    let _ = cast(&mut test, &on_self(&[(Effect::Water, 1)]), None);
    assert!(crate::sites::campfires::saved_campfire(&mut test.world, key).is_none());
}

#[test]
fn frozen_water_is_ice_until_it_thaws() {
    let mut test = magician();
    let visit = test.client.visit.as_mut().unwrap();
    let (x, y) = visit.position();
    let water = (x + 1, y);
    visit.set_cell_for_test(water, Cell::Water);
    let key = visit.plant_key(water.0, water.1);
    let tick = test.world.action_clock.ticks();
    test.world.spell_marks.freeze(key, Cell::Water, tick + 10);
    let cell = |test: &TestClient| test.client.visit.as_ref().unwrap().cells().cell(x + 1, y);
    let alone = std::slice::from_mut(&mut test.client);
    crate::sites::sync_spell_marks(alone, &mut test.world);
    assert_eq!(cell(&test), Cell::Ice);
    test.world.action_clock.advance(10);
    let alone = std::slice::from_mut(&mut test.client);
    crate::sites::sync_spell_marks(alone, &mut test.world);
    assert_eq!(cell(&test), Cell::Water, "thawed");
}

/// Air, Earth, Poison, Curse, Illusion and Warding (magic plan PR 12).
mod lasting;
