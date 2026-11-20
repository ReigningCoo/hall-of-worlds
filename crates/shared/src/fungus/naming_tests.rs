//! Knowing fungi: names shown, lookalikes, the picking check, poisons known.

use super::*;

#[test]
fn shown_is_never_more_than_is_known() {
    let fungi = Fungi::built_in();
    let death_cap = fungi.get("death_cap").unwrap();
    let mut known = FungusKnowledge::default();
    assert_eq!(
        known.shown(death_cap, fungi),
        Shown::Unknown { lichen: false }
    );
    let lichen = fungi.iter().find(|def| def.group == Group::Lichen).unwrap();
    assert_eq!(known.shown(lichen, fungi), Shown::Unknown { lichen: true });
    // The first known lookalike, in the table's order.
    known.learn_name("giant_puffball");
    known.learn_name("straw_mushroom");
    let straw = fungi.get("straw_mushroom").unwrap();
    assert_eq!(known.shown(death_cap, fungi), Shown::LooksLike(straw));
    known.learn_name("death_cap");
    assert_eq!(known.shown(death_cap, fungi), Shown::Known(death_cap));
}

#[test]
fn knowing_the_name_knows_the_poison() {
    let mut known = FungusKnowledge::default();
    assert!(!known.knows_poison("death_cap"));
    known.learn_name("death_cap");
    assert!(known.knows_poison("death_cap"));
    let mut suffered = FungusKnowledge::default();
    suffered.learn_poison("fly_agaric");
    assert!(suffered.knows_poison("fly_agaric"));
    assert!(!suffered.knows_name("fly_agaric"));
}

#[test]
fn herb_effects_are_learned_one_at_a_time() {
    let mut known = FungusKnowledge::default();
    assert!(known.learn_effect("common_ink_cap", 2));
    assert!(!known.learn_effect("common_ink_cap", 2));
    assert!(!known.learn_effect("common_ink_cap", 4));
    assert!(known.knows_effect("common_ink_cap", 2));
    assert!(!known.knows_effect("common_ink_cap", 0));
    assert!(known.learn_all("common_ink_cap"));
    assert!((0..4).all(|n| known.knows_effect("common_ink_cap", n)));
    known.forget_all();
    assert_eq!(known, FungusKnowledge::default());
}

#[test]
fn picking_teaches_names_more_often_with_skill() {
    assert_eq!(name_chance(0, 0), NAME_BASE);
    let full = 100 * crate::skill::VALUE_SCALE;
    assert_eq!(name_chance(full, full), NAME_MOST);
    assert!(name_chance(full / 2, 0) > name_chance(0, full / 2));
    assert!(names_on_picking(0, 0, NAME_BASE - 1));
    assert!(!names_on_picking(0, 0, NAME_BASE));
}

#[test]
fn older_knowledge_loads_with_nothing_named() {
    let old: FungusKnowledge = ron::from_str(r#"(poisons: ["death_cap"])"#).unwrap();
    assert!(old.knows_poison("death_cap"));
    assert!(old.names.is_empty());
}
