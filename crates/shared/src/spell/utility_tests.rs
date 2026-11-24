use super::*;
use crate::bonus::{Source, Totals};
use crate::clearance::{Footing, check_segment, check_segment_by};
use crate::creature::CreatureKind;
use crate::dungeon::{Cell, Level};
use crate::item::Inventory;
use crate::movement::Gait;
use crate::sheet::{Attribute, Sheet};
use crate::skill::{SkillId, Skills};
use crate::spot::Spot;
use crate::vitals::Points;

fn outcome_of(effects: Vec<(Effect, u8)>) -> Outcome {
    let step = EffectStep { effects };
    outcomes(&step)[0].0
}

fn reversed(effect: Effect) -> Acts {
    acts(outcome_of(vec![(Effect::Void, 1), (effect, 1)]))
}

fn plain(effect: Effect) -> Acts {
    acts(outcome_of(vec![(effect, 1)]))
}

fn human() -> (Sheet, Inventory) {
    let kind = CreatureKind::Human;
    let carrying = Inventory {
        creature: kind,
        ..Inventory::default()
    };
    (Sheet::new(kind), carrying)
}

#[test]
fn each_last_effect_acts_as_the_plan_says() {
    assert!(plain(Effect::Identify).identifies && reversed(Effect::Identify).conceals);
    assert!(plain(Effect::Identify).on_place(), "it finds secret doors");
    assert!(plain(Effect::Mana).gives_mana && reversed(Effect::Mana).drains_mana);
    assert_eq!(plain(Effect::Teleport).lasts, Some(TELEPORTING));
    assert_eq!(reversed(Effect::Teleport).lasts, Some(ANCHORED));
    assert_eq!(plain(Effect::Haste).lasts, Some(HASTED));
    assert_eq!(reversed(Effect::Haste).lasts, Some(SLOWED));
    assert_eq!(plain(Effect::Calm).lasts, Some(CALMED));
    assert_eq!(reversed(Effect::Calm).lasts, Some(FRIGHTENED));
    assert_eq!(plain(Effect::Blessing).lasts, Some(BLESSED));
    assert_eq!(reversed(Effect::Blessing).lasts, Some(WEAKENED));
    assert_eq!(plain(Effect::Levitate).lasts, Some(LEVITATING));
    assert_eq!(reversed(Effect::Levitate).ends, Some(Ends::Levitation));
    let light = plain(Effect::Light);
    assert_eq!((light.lasts, light.ends), (Some(LIT), Some(Ends::Darkness)));
    assert!(light.glows && light.on_place());
    let dark = reversed(Effect::Light);
    assert_eq!((dark.lasts, dark.ends), (Some(DARKENED), Some(Ends::Light)));
    assert!(!dark.on_place());
}

#[test]
fn every_lasting_part_lasts_and_haste_and_blessing_change_numbers() {
    for part in [
        HASTED,
        SLOWED,
        BLESSED,
        WEAKENED,
        LIT,
        DARKENED,
        CALMED,
        FRIGHTENED,
        LEVITATING,
        TELEPORTING,
        ANCHORED,
    ] {
        let effect = lasting(part, 2, None, 0).expect("it lasts");
        assert_eq!(effect.source, Source::Spell(part));
        assert!(effect.end.is_some());
        assert_eq!(seconds_of(part, 2), utility_seconds(part) * 2);
    }
    let (mut sheet, carrying) = human();
    let speed = sheet.speed(&carrying);
    sheet.effects.add(lasting(HASTED, 1, None, 0).unwrap());
    assert_eq!(sheet.speed(&carrying), speed * 125 / 100);
    let blessed = lasting(BLESSED, 1, None, 0).unwrap();
    let base = sheet.attributes;
    let up = Totals::of(&blessed.bonuses).attributes(&base);
    assert!(up.get(Attribute::Strength) > base.get(Attribute::Strength));
    let weak = lasting(WEAKENED, 1, None, 0).unwrap();
    let down = Totals::of(&weak.bonuses).attributes(&base);
    assert!(down.get(Attribute::Will) < base.get(Attribute::Will));
}

#[test]
fn light_and_darkness_change_the_light_carried() {
    let (mut sheet, carrying) = human();
    assert_eq!(sheet.light_reach(&carrying), 0);
    sheet.effects.add(lasting(LIT, 1, None, 0).unwrap());
    assert_eq!(sheet.light_reach(&carrying), LIGHT_CELLS as u32);
    let mut skilled = Skills::default();
    skilled.set_percent(Effect::Light.skill(), 100);
    let bright = utility_bonuses(LIT, 1, Some(&skilled));
    assert_eq!(Totals::of(&bright).light(), 2 * LIGHT_CELLS as u32);
    sheet.effects.add(lasting(DARKENED, 1, None, 0).unwrap());
    assert_eq!(sheet.light_reach(&carrying), 0);
}

#[test]
fn mana_given_never_comes_to_more_than_the_spell_cost() {
    let cost = Points::whole(10);
    assert_eq!(mana_given(cost, 1, None), Points::whole(5));
    assert_eq!(mana_given(cost, 2, None), Points(2500));
    let mut skilled = Skills::default();
    skilled.set_percent(Effect::Mana.skill(), 100);
    assert_eq!(mana_given(cost, 1, Some(&skilled)), cost);
    assert_eq!(
        mana_drained(2, None),
        Points(mana_drained(1, None).milli() * 2)
    );
    assert!(SkillId::find("magic/arcane/mana").is_some());
}

/// Floor at x 0 and 4, a hole at 1, water at 2 and 3.
fn gap() -> Level {
    let mut level = Level::new(0, 0, 5, 1);
    for (x, cell) in [
        Cell::Floor,
        Cell::Hole,
        Cell::Water,
        Cell::Water,
        Cell::Floor,
    ]
    .into_iter()
    .enumerate()
    {
        level.set(x as i64, 0, cell);
    }
    level
}

#[test]
fn levitating_floats_over_holes_and_water_and_may_jump_out_after() {
    let level = gap();
    let size = CreatureKind::Human.def().size;
    let (from, to) = (Spot::middle_of((0, 0)), Spot::middle_of((4, 0)));
    assert!(check_segment(&level, size, from, to).is_none());
    assert!(check_segment_by(&level, size, Footing::Floats, from, to).is_some());
    let (mut sheet, _) = human();
    assert_eq!(Gait::of_sheet(&sheet).footing(), Footing::Walks);
    sheet.effects.add(lasting(LEVITATING, 1, None, 0).unwrap());
    assert_eq!(Gait::of_sheet(&sheet).footing(), Footing::Floats);
    // Left over the hole when it ends: a jump out is still a move.
    let over_hole = Spot::middle_of((1, 0));
    assert!(check_segment(&level, size, over_hole, from).is_some());
}

#[test]
fn a_teleport_goes_to_a_clear_cell_in_range_only_while_not_held() {
    let mut level = Level::new(0, 0, 20, 3);
    for x in 0..20 {
        for y in 0..3 {
            level.set(x, y, Cell::Floor);
        }
    }
    level.set(5, 0, Cell::Wall);
    let size = CreatureKind::Human.def().size;
    let at = Spot::middle_of((0, 1));
    assert!(may_teleport_to(&level, size, at, Spot::middle_of((8, 1))));
    assert!(
        !may_teleport_to(&level, size, at, Spot::middle_of((15, 1))),
        "too far"
    );
    assert!(
        !may_teleport_to(&level, size, at, Spot::middle_of((5, 0))),
        "a wall"
    );
    let (mut sheet, _) = human();
    assert!(!sheet.may_teleport());
    sheet.effects.add(lasting(TELEPORTING, 1, None, 0).unwrap());
    assert!(sheet.may_teleport());
    sheet.effects.add(lasting(ANCHORED, 1, None, 0).unwrap());
    assert!(!sheet.may_teleport());
}
