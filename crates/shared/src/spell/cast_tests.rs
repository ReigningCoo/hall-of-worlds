use super::*;
use crate::vitals::Points;

#[test]
fn a_design_always_has_the_same_key_and_others_differ() {
    let heal = heal_spell();
    assert_eq!(SpellKey::of(&heal), SpellKey::of(&heal.clone()));
    let twice = Spell::new(vec![Step::effects(&[(Effect::Heal, 2)])]);
    assert_ne!(SpellKey::of(&heal), SpellKey::of(&twice));
    let book = Spellbook::default();
    assert_eq!(book.by_key(SpellKey::of(&heal)).unwrap().spell, heal);
    assert!(book.by_key(SpellKey::of(&twice)).is_none());
}

#[test]
fn heals_keep_the_old_amount_each() {
    let one = crate::magic::heal_amount(None);
    assert_eq!(one, Points::whole(12));
    assert_eq!(heal_restores(3, None), Points::whole(36));
}

#[test]
fn spells_leave_the_caster_and_friends_one_health() {
    let health = Points::whole(10);
    let harm = Points::whole(30);
    assert_eq!(spared_harm(harm, health, Side::Caster), Points::whole(9));
    assert_eq!(spared_harm(harm, health, Side::Friend), Points::whole(9));
    assert_eq!(spared_harm(harm, health, Side::Enemy), harm);
    assert_eq!(spared_harm(harm, Points::whole(1), Side::Friend), Points(0));
}
