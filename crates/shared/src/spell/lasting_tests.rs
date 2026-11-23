use super::*;
use crate::bonus::{Source, Stat, Totals};
use crate::combat::Guard;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::spot::Spot;

fn outcome_of(effects: Vec<(Effect, u8)>) -> Outcome {
    let step = EffectStep { effects };
    outcomes(&step)[0].0
}

#[test]
fn each_new_effect_acts_as_the_plan_says() {
    let air = acts(outcome_of(vec![(Effect::Air, 1)]));
    assert_eq!(air.knocks, Some(Knock::Away));
    let vacuum = acts(outcome_of(vec![(Effect::Void, 1), (Effect::Air, 1)]));
    assert_eq!(vacuum.knocks, Some(Knock::In));
    let earth = acts(outcome_of(vec![(Effect::Earth, 1)]));
    assert_eq!(earth.lasts, Some(PARALYSED));
    let stoneskin = acts(outcome_of(vec![(Effect::Void, 1), (Effect::Earth, 1)]));
    assert_eq!(stoneskin.lasts, Some(STONESKIN));
    let cure = acts(outcome_of(vec![(Effect::Void, 1), (Effect::Poison, 1)]));
    assert_eq!(cure.ends, Some(Ends::Poison));
    let dispel = acts(outcome_of(vec![(Effect::Void, 1), (Effect::Warding, 1)]));
    assert_eq!(dispel.ends, Some(Ends::Spells));
    // None of them does anything to the place itself.
    for acted in [air, vacuum, earth, stoneskin, cure, dispel] {
        assert!(!acted.on_place() && !acted.harms);
    }
}

#[test]
fn lasting_effects_change_blows_and_resistance() {
    let now = 100;
    let stoneskin = lasting(STONESKIN, 1, None, now).unwrap();
    let cursed = lasting(CURSED, 2, None, now).unwrap();
    let illusion = lasting(ILLUSION, 1, None, now).unwrap();
    let warded = lasting(WARDED, 1, None, now).unwrap();
    let totals = Totals::of(stoneskin.bonuses.iter().chain(&illusion.bonuses));
    let guard = Guard::of(&totals);
    assert_eq!(guard.blows_taken, 850);
    assert_eq!(guard.defence, 1250);
    assert_eq!(guard.blows_done, 1000);
    let cursed = Guard::of(&Totals::of(&cursed.bonuses));
    assert_eq!(cursed.blows_done, 700);
    // A cursed blow on stoneskin: 0.7 × 0.85.
    assert_eq!(cursed.blow(guard, 1000), 595);
    let warded = Totals::of(&warded.bonuses);
    assert_eq!(warded.per_mille(Stat::Resistance), 1250);
    // Two counts last twice as long.
    assert_eq!(
        lasting(CURSED, 2, None, now).unwrap().end,
        Some(now + 2 * (stoneskin.end.unwrap() - now))
    );
    assert!(lasting(POISONED, 1, None, now).is_none());
}

#[test]
fn curses_and_stoneskin_never_take_off_more_than_their_most() {
    let cursed = lasting(CURSED, 50, None, 0).unwrap();
    let guard = Guard::of(&Totals::of(&cursed.bonuses));
    assert_eq!(guard.blows_done, 1000 - 10 * MOST_LESS_PERCENT as u64);
}

#[test]
fn poison_drains_health_for_its_time() {
    let per_second = poison_per_second(None);
    // Heal's line at no skill, 12 Health, over 10 s.
    assert_eq!(per_second.milli(), 1200);
    let effect = poison(per_second, 2, 0);
    assert_eq!(effect.source, Source::Spell(POISONED));
    assert_eq!(
        Totals::of(&effect.bonuses).draining(crate::vitals::Vital::Health),
        1200
    );
    assert_eq!(seconds_of(POISONED, 2), 2 * POISON_SECONDS);
}

#[test]
fn removing_effects_end_only_theirs() {
    assert!(Ends::Poison.ends(Source::Poison));
    assert!(Ends::Poison.ends(Source::Spell(POISONED)));
    assert!(!Ends::Poison.ends(Source::Spell(CURSED)));
    assert!(Ends::Curse.ends(Source::Spell(CURSED)));
    assert!(Ends::Illusion.ends(Source::Spell(ILLUSION)));
    assert!(Ends::Spells.ends(Source::Spell(WARDED)));
    assert!(Ends::Spells.ends(Source::Spell(WET)));
    assert!(!Ends::Spells.ends(Source::WellFed));
}

#[test]
fn air_knocks_as_far_as_walls_let() {
    assert_eq!(knock_units(1, None), KNOCK_UNITS);
    assert_eq!(knock_units(9, None), MOST_KNOCK);
    // Floor from x 1 to 4 between walls, a wall at 5.
    let mut level = Level::new(0, 0, 8, 3);
    for x in 1..5 {
        level.set(x, 1, Cell::Floor);
    }
    let size = crate::creature::Size::Medium;
    let at = Spot::middle_of((1, 1));
    // Two cells east is clear.
    let to = knocked_to(&level, size, at, Heading::EAST, 2 * KNOCK_UNITS);
    assert_eq!(to, Spot::middle_of((3, 1)));
    // Five would go through the wall: it stops short of it.
    let to = knocked_to(&level, size, at, Heading::EAST, MOST_KNOCK);
    assert!(to.cell().0 < 5 && to.cell().0 > 1);
}
