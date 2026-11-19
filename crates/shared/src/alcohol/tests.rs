use super::*;
use crate::bonus::{Source, Stat};
use crate::creature::CreatureKind;
use crate::fermentation::{Base, Drink, Sweetness};
use crate::id::Id;
use crate::item::{Inventory, Quality};
use crate::movement::{FULL, Step};
use crate::potion::Liquid;
use crate::sheet::{Sheet, Situation};
use crate::spot::Spot;
use crate::vitals::REFILL_TICKS;

fn ale(strength: u32) -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Grain("barley".into()),
        strength,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}

const AVERAGE: Drinker = Drinker {
    grams: AVERAGE_GRAMS,
    tolerance: 0,
};

#[test]
fn a_serving_of_ale_holds_a_unit_and_a_quarter() {
    assert_eq!(units_in(&ale(50), 250), 1250);
    assert_eq!(
        units_in(&Liquid::Water(crate::potion::Water::Plain), 250),
        0
    );
}

#[test]
fn thresholds_grow_with_mass_and_tolerance() {
    assert_eq!(AVERAGE.threshold(TIPSY_MILLI), TIPSY_MILLI);
    assert_eq!(
        Drinker::new(140_000, 0).threshold(TIPSY_MILLI),
        2 * TIPSY_MILLI
    );
    assert_eq!(
        Drinker::new(70_000, FULL).threshold(TIPSY_MILLI),
        2 * TIPSY_MILLI
    );
    assert_eq!(Drinker::new(70_000, 5 * FULL).tolerance, FULL);
}

#[test]
fn a_unit_wears_off_in_ten_minutes_at_seventy_kilograms() {
    let mut alcohol = Alcohol::default();
    alcohol.drink(1000, 100, AVERAGE);
    assert_eq!(alcohol.milli_at(100 + UNIT_TICKS / 2, AVERAGE), 500);
    assert_eq!(alcohol.milli_at(100 + UNIT_TICKS, AVERAGE), 0);
    assert_eq!(alcohol.sober_at(AVERAGE), 100 + UNIT_TICKS);
    // A body twice as heavy wears it off twice as fast.
    assert_eq!(
        alcohol.sober_at(Drinker::new(140_000, 0)),
        100 + UNIT_TICKS / 2
    );
}

#[test]
fn stages_go_by_units_then_a_hangover() {
    let mut alcohol = Alcohol::default();
    alcohol.drink(8000, 0, AVERAGE);
    assert_eq!(alcohol.stage(0, AVERAGE), Some(Stage::Drunk));
    let tipsy = alcohol.next_change(0, AVERAGE).unwrap();
    assert_eq!(alcohol.stage(tipsy - 1, AVERAGE), Some(Stage::Drunk));
    assert_eq!(alcohol.stage(tipsy, AVERAGE), Some(Stage::Tipsy));
    let hungover = alcohol.next_change(tipsy, AVERAGE).unwrap();
    assert_eq!(alcohol.stage(hungover, AVERAGE), Some(Stage::Hangover));
    let sober = alcohol.next_change(hungover, AVERAGE).unwrap();
    assert_eq!(sober, alcohol.sober_at(AVERAGE) + HANGOVER_TICKS);
    assert_eq!(alcohol.stage(sober, AVERAGE), None);
    assert_eq!(alcohol.next_change(sober, AVERAGE), None);
}

#[test]
fn only_being_drunk_brings_a_hangover() {
    let mut alcohol = Alcohol::default();
    alcohol.drink(4000, 0, AVERAGE);
    let sober = alcohol.next_change(0, AVERAGE).unwrap();
    assert_eq!(alcohol.stage(sober, AVERAGE), None);
}

#[test]
fn becoming_very_drunk_collapses_for_a_minute() {
    let mut alcohol = Alcohol::default();
    alcohol.drink(10_000, 0, AVERAGE);
    assert_eq!(alcohol.stage(0, AVERAGE), Some(Stage::Drunk));
    alcohol.drink(3000, 20, AVERAGE);
    assert_eq!(alcohol.stage(20, AVERAGE), Some(Stage::Collapsed));
    assert_eq!(alcohol.next_change(20, AVERAGE), Some(20 + COLLAPSE_TICKS));
    assert_eq!(
        alcohol.stage(20 + COLLAPSE_TICKS, AVERAGE),
        Some(Stage::VeryDrunk)
    );
    // Drinking more while very drunk doesn't collapse it again.
    alcohol.drink(1000, 30 + COLLAPSE_TICKS, AVERAGE);
    assert_eq!(
        alcohol.stage(30 + COLLAPSE_TICKS, AVERAGE),
        Some(Stage::VeryDrunk)
    );
}

#[test]
fn the_stage_holds_until_its_next_change() {
    let mut alcohol = Alcohol::default();
    alcohol.drink(25_000, 0, AVERAGE);
    let mut now = 0;
    let mut seen = Vec::new();
    while let Some(next) = alcohol.next_change(now, AVERAGE) {
        let stage = alcohol.stage(now, AVERAGE);
        seen.push(stage);
        for tick in [now + 1, (now + next) / 2, next - 1] {
            assert_eq!(alcohol.stage(tick, AVERAGE), stage);
        }
        assert_ne!(alcohol.stage(next, AVERAGE), stage);
        now = next;
    }
    use Stage::*;
    let expected = [Collapsed, Poisoned, VeryDrunk, Drunk, Tipsy, Hangover];
    assert_eq!(seen, expected.map(Some));
}

#[test]
fn poisoning_drains_health_and_drink_slows() {
    let drains = Stage::Poisoned.bonuses();
    assert!(
        drains
            .iter()
            .any(|bonus| bonus.stat == Stat::Drain(crate::vitals::Vital::Health))
    );
    assert!(Stage::Tipsy.bonuses().is_empty());
    assert!(Stage::Drunk.bonuses().len() == 3);
}

fn situation(worn: &[crate::item::Item]) -> Situation<'_> {
    Situation {
        worn,
        playtime: 0,
        refill: REFILL_TICKS,
    }
}

#[test]
fn drinking_lays_on_the_stage_and_slows() {
    let kind = CreatureKind::Human;
    let carrying = Inventory {
        creature: kind,
        ..Inventory::default()
    };
    let mut sheet = Sheet::new(kind);
    let worn = Vec::new();
    let speed = sheet.speed(&carrying);
    sheet.drink_alcohol(4000, kind, situation(&worn), 0);
    assert_eq!(sheet.alcohol_stage(), Some(Stage::Tipsy));
    assert_eq!(sheet.speed(&carrying), speed * 67 / 100);
    let end = sheet
        .effects
        .0
        .iter()
        .find(|effect| matches!(effect.source, Source::Alcohol(_)))
        .and_then(|effect| effect.end)
        .unwrap();
    assert!(!sheet.keep_alcohol(kind, situation(&worn), end - 1));
    assert!(sheet.keep_alcohol(kind, situation(&worn), end));
    assert_eq!(sheet.alcohol_stage(), None);
    assert_eq!(sheet.speed(&carrying), speed);
}

#[test]
fn stumbles_are_seeded_and_odd() {
    let id = Id::for_test(7);
    let count = |stage| {
        (0..10_000)
            .filter(|&number| stumble(id, 1, number, stage).is_some())
            .count()
    };
    assert_eq!(count(Some(Stage::Tipsy)), 0);
    assert_eq!(count(None), 0);
    let drunk = count(Some(Stage::Drunk));
    assert!((800..1200).contains(&drunk), "{drunk}");
    let very = count(Some(Stage::VeryDrunk));
    assert!((1700..2300).contains(&very), "{very}");
    for number in 0..100 {
        let stage = Some(Stage::Drunk);
        assert_eq!(stumble(id, 1, number, stage), stumble(id, 1, number, stage));
    }
}

#[test]
fn veering_turns_steps_and_moves() {
    let east = Step::new(1, 0).unwrap();
    let veer = |clockwise, eighths| Veer { clockwise, eighths };
    assert_eq!(veer(true, 1).step(east), Step::new(1, 1).unwrap());
    assert_eq!(veer(false, 2).step(east), Step::new(0, -1).unwrap());
    let from = Spot::new(0, 0);
    let to = Spot::new(240, 0);
    assert_eq!(veer(true, 2).turned(from, to), Spot::new(0, 240));
    assert_eq!(veer(true, 1).turned(from, to), Spot::new(169, 169));
    assert_eq!(veer(false, 1).turned(from, to), Spot::new(169, -169));
}
