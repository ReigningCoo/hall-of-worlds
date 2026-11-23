//! Void, Fire and Water (magic plan PR 11): what each outcome does, harm and
//! resisting, the wet and the chill, frozen water and clouds of steam.

use super::*;
use crate::creature::CreatureKind;
use crate::dungeon::{Cell, Level, line_is_clear};
use crate::item::Inventory;
use crate::rng::Seed;
use crate::sheet::{Attributes, Sheet};
use crate::vitals::Points;

fn outcomes_of(effects: &[(Effect, u8)]) -> Vec<(Outcome, u32)> {
    outcomes(&EffectStep {
        effects: effects.to_vec(),
    })
}

#[test]
fn void_turns_fire_cold_and_water_ice() {
    let cold = Outcome::Part(Part::reversed(Effect::Fire));
    let ice = Outcome::Part(Part::reversed(Effect::Water));
    assert_eq!(
        outcomes_of(&[(Effect::Void, 1), (Effect::Fire, 1)]),
        [(cold, 1)]
    );
    assert_eq!(
        outcomes_of(&[(Effect::Void, 1), (Effect::Water, 2)]),
        [(ice, 2)]
    );
    // Fire before Void stays fire.
    let fire = Outcome::Part(Part::plain(Effect::Fire));
    assert_eq!(
        outcomes_of(&[(Effect::Fire, 1), (Effect::Void, 1), (Effect::Water, 1)]),
        [(fire, 1), (ice, 1)]
    );
}

#[test]
fn each_outcome_acts_as_the_plan_says() {
    let fire = acts(Outcome::Part(Part::plain(Effect::Fire)));
    assert!(fire.harms && fire.lights && fire.dries && !fire.douses);
    let cold = acts(Outcome::Part(Part::reversed(Effect::Fire)));
    assert!(cold.harms && !cold.lights && !cold.on_place());
    let water = acts(Outcome::Part(Part::plain(Effect::Water)));
    assert!(water.wets && water.douses && !water.harms);
    let ice = acts(Outcome::Part(Part::reversed(Effect::Water)));
    assert!(ice.chills && ice.freezes && !ice.harms);
    let steam = acts(Outcome::Mixture(Mixture::Steam));
    assert!(steam.harms && steam.douses && steam.clouds);
    let harm = acts(Outcome::Part(Part::reversed(Effect::Heal)));
    assert!(harm.harms && !harm.on_place());
    assert!(!acts(Outcome::Part(Part::plain(Effect::Heal))).harms);
}

#[test]
fn harm_follows_heals_line_and_steam_does_half_again() {
    let fire = Outcome::Part(Part::plain(Effect::Fire));
    let steam = Outcome::Mixture(Mixture::Steam);
    let water = Outcome::Part(Part::plain(Effect::Water));
    assert_eq!(harm_before_resisting(fire, 1, None), Points::whole(12));
    assert_eq!(harm_before_resisting(fire, 2, None), Points::whole(24));
    assert_eq!(harm_before_resisting(steam, 1, None), Points::whole(18));
    assert_eq!(harm_before_resisting(water, 1, None), Points::ZERO);
}

#[test]
fn equals_resist_half_and_aura_resists_more() {
    let fire = Outcome::Part(Part::plain(Effect::Fire));
    let median = Attributes::all(100);
    let power = spell_power(fire, &median, None);
    let resist = resistance(&median, None);
    assert_eq!(
        power, resist,
        "an untrained median caster against a median target"
    );
    assert_eq!(resisted(Points::whole(12), power, resist), Points::whole(6));
    let mut strong = median;
    strong.aura *= 3;
    assert!(resistance(&strong, None) > resist);
}

#[test]
fn a_chill_takes_hold_by_the_ratio_contest() {
    let mut rng = Seed::new(5).rng();
    let held = (0..1000)
        .filter(|_| takes_hold(1000, 3000, &mut rng))
        .count();
    // A quarter, give or take.
    assert!((200..300).contains(&held), "{held}");
}

#[test]
fn the_wet_and_the_chilled_last_and_the_chill_slows() {
    let kind = CreatureKind::Human;
    let carrying = Inventory {
        creature: kind,
        ..Inventory::default()
    };
    let mut sheet = Sheet::new(kind);
    let speed = sheet.speed(&carrying);
    sheet.effects.add(wet(1, 0));
    assert!(sheet.under_spell(WET, 0));
    assert_eq!(sheet.speed(&carrying), speed, "wet doesn't slow");
    sheet.effects.add(chill(1, 0));
    assert_eq!(sheet.speed(&carrying), speed * CHILL_SPEED_PERCENT / 100);
    let ends = CHILL_SECONDS * 20;
    assert!(!sheet.under_spell(CHILLED, ends));
    assert!(sheet.end_spell(WET));
    assert!(!sheet.under_spell(WET, 0));
}

/// A row of floor from x 0 to 6 with water from 2 to 4.
fn pond() -> Level {
    let mut level = Level::new(0, 0, 7, 1);
    for x in 0..7 {
        level.set(
            x,
            0,
            if (2..=4).contains(&x) {
                Cell::Water
            } else {
                Cell::Floor
            },
        );
    }
    level
}

#[test]
fn ice_freezes_water_round_where_it_lands_and_is_walked_on() {
    let level = pond();
    assert_eq!(frozen_cells(&level, (1, 0), 1), [(2, 0)]);
    assert_eq!(frozen_cells(&level, (3, 0), 1), [(2, 0), (3, 0), (4, 0)]);
    assert!(Cell::Ice.is_walkable() && Cell::Ice.is_open());
}

#[test]
fn a_cloud_of_steam_hides_what_is_behind_it() {
    let mut level = pond();
    assert!(line_is_clear(&level, (0, 0), (6, 0)));
    level.set_cloud(3, 0, true);
    assert!(!line_is_clear(&level, (0, 0), (6, 0)), "behind it");
    assert!(line_is_clear(&level, (0, 0), (3, 0)), "the cloud itself");
    assert_eq!(cloud_cells(&level, (3, 0), 1), [(2, 0), (3, 0), (4, 0)]);
}

#[test]
fn a_spell_acting_on_its_place_lands_without_a_creature() {
    let touch = Step::at(Where::Touch, 1);
    let water = Spell::new(vec![touch.clone(), Step::effects(&[(Effect::Water, 1)])]);
    let heal = Spell::new(vec![touch, Step::effects(&[(Effect::Heal, 1)])]);
    assert!(acts_on_place(&water));
    assert!(!acts_on_place(&heal));
}
