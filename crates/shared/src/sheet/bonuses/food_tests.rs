//! Eating on a sheet: poison runs alongside healing and never kills.

use super::*;
use crate::creature::CreatureKind;
use crate::vitals::{Points, REFILL_TICKS, Vital};

const SECOND: u64 = crate::vitals::TICKS;

fn sheet() -> Sheet {
    let mut sheet = Sheet::new(CreatureKind::Human);
    sheet.attributes = Attributes::all(100);
    sheet
}

fn situation() -> Situation<'static> {
    Situation {
        worn: &[],
        playtime: 0,
        refill: REFILL_TICKS,
    }
}

fn health(sheet: &Sheet, now: u64) -> Points {
    let limits = sheet.limits(situation(), now);
    sheet.vitals.at(Vital::Health, limits, now)
}

#[test]
fn raw_meat_poisons_a_human_down_to_one_health_and_no_lower() {
    let mut sheet = sheet();
    assert!(sheet.eat(ItemKind::RawMeat, CreatureKind::Human, situation(), 0));
    // 20 Health less 1 a second, with a little refilled.
    let at = health(&sheet, 5 * SECOND);
    assert!(at < Points::whole(16) && at > Points::whole(15), "{at:?}");
    assert_eq!(health(&sheet, 59 * SECOND), Points::whole(1));
    // Moving and being hurt don't end it.
    assert!(!sheet.cancel_effects(Cancel::Moving, situation(), 10 * SECOND));
    assert!(!sheet.cancel_effects(Cancel::Hurt, situation(), 10 * SECOND));
    // Once it has run out, refilling goes on from 1.
    assert!(sheet.forget_ended(situation(), 70 * SECOND));
    assert!(health(&sheet, 70 * SECOND) > Points::whole(1));
}

#[test]
fn dwarven_bread_is_refused_to_a_human() {
    let mut sheet = sheet();
    let kind = CreatureKind::Human;
    assert!(!sheet.eat(ItemKind::DwarvenBread, kind, situation(), 0));
    assert!(sheet.effects.is_empty());
}

#[test]
fn poison_and_healing_run_together() {
    let mut sheet = sheet();
    let human = CreatureKind::Human;
    sheet.eat(ItemKind::RawMeat, human, situation(), 0);
    sheet.eat(ItemKind::Bread, human, situation(), SECOND);
    assert_eq!(sheet.effects.0.len(), 2);
    // Eating more raw meat starts the poison again rather than adding to it.
    sheet.eat(ItemKind::RawMeat, human, situation(), 2 * SECOND);
    assert_eq!(sheet.effects.0.len(), 2);
    let limits = sheet.limits(situation(), 3 * SECOND);
    assert_eq!(limits.restore.drain_health, 1000);
    assert_eq!(limits.restore.health, 1000);
}
