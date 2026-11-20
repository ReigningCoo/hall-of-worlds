//! Effects laid on for later: hidden until known, started at their ticks.

use super::*;
use crate::bonus::Stat;
use crate::creature::CreatureKind;
use crate::fungus::{Eater, FungusTag, eating};
use crate::sheet::Attributes;
use crate::vitals::{REFILL_TICKS, Vital};

fn situation() -> Situation<'static> {
    Situation {
        worn: &[],
        playtime: 0,
        refill: REFILL_TICKS,
    }
}

fn death_cap(sheet: &mut Sheet, known: bool) {
    let def = Fungi::built_in().get("death_cap").unwrap();
    let eater = Eater {
        most_health: 20,
        easy: false,
    };
    let effects = eating(def, 0, eater);
    sheet.lay_on(effects, known, situation(), 0);
}

fn sheet() -> Sheet {
    Sheet {
        attributes: Attributes::all(100),
        ..Sheet::new(CreatureKind::Human)
    }
}

#[test]
fn a_poison_not_recognised_is_hidden_until_it_starts() {
    let mut sheet = sheet();
    death_cap(&mut sheet, false);
    assert!(sheet.effects.is_empty(), "nothing yet");
    assert_eq!(sheet.later.0.len(), 2, "sickness and the worst");
    assert!(sheet.as_sent().later.is_empty(), "hidden");
    // Nothing drains before it starts.
    let start = sheet.later.next_start(0).unwrap();
    assert_eq!(sheet.limits(situation(), start - 1).restore.drain_health, 0);
    assert_eq!(sheet.limits(situation(), 0).restore.until, Some(start));
    // Its player isn't told when restoring would change, or they'd know.
    assert_eq!(sheet.told_limits(situation(), 0).restore.until, None);
    assert!(sheet.forget_ended(situation(), start));
    let tag = FungusTag::of("death_cap");
    assert_eq!(sheet.effects.0[0].source, Source::FungusPoison(tag));
    assert!(sheet.limits(situation(), start).restore.drain_health > 0);
    // Suffering it teaches it: the worst to come is known and sent now.
    assert!(sheet.fungi.knows_poison("death_cap"));
    assert_eq!(sheet.as_sent().later.0.len(), 1);
    assert!(!sheet.limits(situation(), start).restore.deadly);
}

#[test]
fn a_recognised_poison_is_sent_before_it_starts() {
    let mut sheet = sheet();
    death_cap(&mut sheet, true);
    assert_eq!(sheet.as_sent().later.0.len(), 2);
    let start = sheet.later.next_start(0);
    assert_eq!(sheet.told_limits(situation(), 0).restore.until, start);
}

#[test]
fn the_worst_is_deadly_after_the_easing() {
    let mut sheet = sheet();
    death_cap(&mut sheet, false);
    let worst = (sheet.later.0.iter())
        .map(|p| p.effect.start)
        .max()
        .unwrap();
    sheet.forget_ended(situation(), worst);
    let limits = sheet.limits(situation(), worst);
    assert!(limits.restore.deadly);
    let deadly = Stat::DeadlyDrain(Vital::Health);
    let bonuses: Vec<_> = sheet.effects.bonuses(worst).collect();
    assert!(bonuses.iter().any(|b| b.stat == deadly));
    assert!(sheet.later.is_empty());
}

#[test]
fn a_confusing_poison_stumbles() {
    let mut sheet = sheet();
    assert_eq!(sheet.stumble_one_in(), None);
    let def = Fungi::built_in().get("fly_agaric").unwrap();
    let eater = Eater {
        most_health: 20,
        easy: false,
    };
    sheet.lay_on(eating(def, 0, eater), false, situation(), 0);
    assert_eq!(
        sheet.stumble_one_in(),
        Some(crate::alcohol::CONFUSED_ONE_IN)
    );
}
