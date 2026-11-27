use shared::creature::CreatureKind;
use shared::sheet::Sheet;
use shared::skill::{SkillId, practice_for};
use shared::spell::{Effect, Spell, Step, Where, heal_spell};
use shared::vitals::Points;

use super::*;

fn touch_heal() -> Spell {
    Spell::new(vec![
        Step::at(Where::Touch, 1),
        Step::effects(&[(Effect::Heal, 1)]),
    ])
}

#[test]
fn nothing_told_affords_only_what_costs_none() {
    let vitals = OwnVitals(None);
    assert!(affords(Points(0), &vitals, 0.0));
    assert!(!affords(Points::whole(1), &vitals, 0.0));
    assert!(!can_cast_spell(
        &heal_spell(),
        &Sheet::default(),
        (&vitals, 0.0),
        false
    ));
}

#[test]
fn a_where_the_skills_dont_unlock_is_refused() {
    let sheet = Sheet::new(CreatureKind::Dwarf);
    let vitals = OwnVitals(None);
    // Refused by the design whatever the Mana.
    assert!(shared::spell::check(&touch_heal(), Some(&sheet.skills)).is_err());
    assert!(!can_cast_spell(
        &touch_heal(),
        &sheet,
        (&vitals, 0.0),
        false
    ));
}

#[test]
fn the_way_inn_refuses_a_where_but_not_a_spell_on_the_caster() {
    let mut sheet = Sheet::new(CreatureKind::Dwarf);
    let magic = SkillId::find("magic").unwrap();
    sheet.skills.practise(magic, practice_for(1));
    // A whole Mana pool, told now.
    let vitals = told(1000);
    assert!(can_cast_spell(&touch_heal(), &sheet, (&vitals, 0.0), false));
    assert!(!can_cast_spell(&touch_heal(), &sheet, (&vitals, 0.0), true));
    assert!(can_cast_spell(&heal_spell(), &sheet, (&vitals, 0.0), true));
}

fn told(mana: u32) -> OwnVitals {
    use shared::vitals::VitalNow;
    let full = |most: u32| VitalNow {
        value: Points::whole(most),
        most,
        refill: shared::vitals::REFILL_TICKS as u32,
        ..Default::default()
    };
    OwnVitals(Some(super::super::ToldVitals {
        health: full(100),
        mana: full(mana),
        at: 0.0,
    }))
}
