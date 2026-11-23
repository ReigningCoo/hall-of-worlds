//! Wheres merged in one step (magic plan PR 10), on the test room (see
//! `hits_tests.rs`): the caster at the middle, facing east.

use serde::Serialize;

use super::*;
use crate::combat::Body;
use crate::creature::Size;
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::lang::{Lang, Language};
use crate::skill::{SkillId, Skills};
use crate::spot::Spot;

const ENEMY: Filters = Filters {
    enemy: true,
    ..Filters::NONE
};

fn at(x: i64, y: i64, side: Side) -> Creature {
    Creature {
        body: Body {
            at: Spot::new(x, y),
            height: 0,
            size: Size::Medium,
        },
        side,
    }
}

fn with_caster(others: &[Creature]) -> Vec<Creature> {
    let mut all = vec![at(0, 0, Side::Caster)];
    all.extend_from_slice(others);
    all
}

fn part(place: Where, count: u8) -> WherePart {
    WherePart { place, count }
}

fn merged(parts: &[WherePart], filters: Filters) -> WhereStep {
    match Step::merged(parts, filters) {
        Some(Step::Where(step)) => step,
        _ => panic!("a where step"),
    }
}

fn hit_ones(hits: &[Hit]) -> Vec<usize> {
    let mut ones: Vec<usize> = hits.iter().filter_map(|hit| hit.creature).collect();
    ones.sort_unstable();
    ones.dedup();
    ones
}

fn hits_of(step: &WhereStep, toward: Toward, all: &[Creature]) -> Vec<usize> {
    let level = Level::test_room();
    let origin = Origin::from_creature(all, 0, Heading::EAST);
    hit_ones(&step_hits(&level, step, &origin, toward, all))
}

#[test]
fn missile_and_surround_hit_the_target_and_the_ring_round_it() {
    let all = with_caster(&[at(960, 0, Side::Enemy), at(1200, 0, Side::Enemy)]);
    let step = merged(
        &[part(Where::Missile, 1), part(Where::Surround, 1)],
        Filters::NONE,
    );
    assert_eq!(hits_of(&step, Toward::Creature(1), &all), [1, 2]);
    // Separate, the ring alone would be what the step after lands on.
    let level = Level::test_room();
    let origin = Origin::from_creature(&all, 0, Heading::EAST);
    let first = where_hits(&level, &step, &origin, Toward::Creature(1), &all);
    assert_eq!(hit_ones(&first), [1]);
    let ring = part_hits(&level, &step, 1, &first[0].origin(), &all);
    assert_eq!(hit_ones(&ring), [2]);
}

#[test]
fn touch_and_missile_hit_what_is_touched_and_what_the_missile_meets() {
    let all = with_caster(&[at(240, 0, Side::Enemy), at(960, 0, Side::Enemy)]);
    let step = merged(
        &[part(Where::Touch, 1), part(Where::Missile, 1)],
        Filters::NONE,
    );
    assert_eq!(hits_of(&step, Toward::Ahead, &all), [1, 2]);
}

#[test]
fn surround_and_missile_send_missiles_out_from_the_ring() {
    // Nobody round the caster: eight missiles fly out from the ring's spots.
    let all = with_caster(&[at(960, 0, Side::Enemy)]);
    let step = merged(
        &[part(Where::Surround, 1), part(Where::Missile, 1)],
        Filters::NONE,
    );
    assert_eq!(hits_of(&step, Toward::Ahead, &all), [1]);
}

#[test]
fn merged_wheres_keep_friend_and_enemy_but_only_the_first_is_aimed() {
    let all = with_caster(&[at(960, 0, Side::Enemy), at(1200, 0, Side::Friend)]);
    let step = merged(&[part(Where::Missile, 1), part(Where::Surround, 1)], ENEMY);
    assert_eq!(hits_of(&step, Toward::Creature(1), &all), [1]);
    // Nothing for the first to aim at: nothing at all.
    let lonely = with_caster(&[at(1200, 0, Side::Friend)]);
    assert!(hits_of(&step, Toward::Ahead, &lonely).is_empty());
    // No where at that place, or the first: nothing.
    let level = Level::test_room();
    let origin = Origin::from_creature(&all, 0, Heading::EAST);
    assert!(part_hits(&level, &step, 0, &origin, &all).is_empty());
    assert!(part_hits(&level, &step, 2, &origin, &all).is_empty());
    assert!(
        launch_part(&level, &step, 1, &origin, &all).is_none(),
        "not a missile"
    );
}

#[test]
fn merged_wheres_multiply_into_the_steps_cost() {
    let step = merged(
        &[part(Where::Missile, 1), part(Where::Surround, 1)],
        Filters::NONE,
    );
    assert_eq!(where_step_cost(&step, None), 6000);
    let step = merged(&[part(Where::Missile, 2), part(Where::Touch, 1)], ENEMY);
    assert_eq!(where_step_cost(&step, None), 5000);
    // Form takes off each where's own share, never below a point each.
    let mut skills = Skills::default();
    skills.set_percent(SkillId::find("magic/form/missile").unwrap(), 100);
    let step = merged(
        &[part(Where::Missile, 1), part(Where::Surround, 1)],
        Filters::NONE,
    );
    assert_eq!(where_step_cost(&step, Some(&skills)), 3000);
    let spell = Spell::new(vec![Step::Where(step), Step::effects(&[(Effect::Fire, 1)])]);
    assert_eq!(spell_cost(&spell, None).0, 2 * 3 * 5 * 1000);
}

#[test]
fn merged_wheres_are_checked() {
    let mut skills = Skills::default();
    skills.set_percent(SkillId::find("magic").unwrap(), 2);
    let fire = Step::effects(&[(Effect::Fire, 1)]);
    let design = |parts: &[WherePart]| {
        Spell::new(vec![
            Step::merged(parts, Filters::NONE).unwrap(),
            fire.clone(),
        ])
    };
    let two = design(&[part(Where::Touch, 1), part(Where::Missile, 1)]);
    assert_eq!(check(&two, Some(&skills)), Ok(()));
    let locked = design(&[part(Where::Missile, 1), part(Where::Surround, 1)]);
    assert_eq!(
        check(&locked, Some(&skills)),
        Err(DesignError::Locked(Where::Surround))
    );
    let zero = design(&[part(Where::Touch, 1), part(Where::Missile, 0)]);
    assert_eq!(check(&zero, Some(&skills)), Err(DesignError::ZeroCount));
    let many = design(&[part(Where::Touch, 1); MAX_WHERES + 1]);
    assert_eq!(check(&many, Some(&skills)), Err(DesignError::TooManyWheres));
}

#[test]
fn a_merged_step_is_named_by_its_last_where() {
    let lang = Lang::new(Language::English);
    let spell = Spell::new(vec![
        Step::merged(
            &[part(Where::Missile, 1), part(Where::Surround, 1)],
            Filters::NONE,
        )
        .unwrap(),
        Step::effects(&[(Effect::Fire, 1)]),
    ]);
    assert_eq!(auto_name(&spell, &lang), "Ball of Fire");
}

#[test]
fn old_where_steps_load_and_keep_their_keys() {
    let old = "(place: Missile, count: 2, filters: (caster: false, friend: false, enemy: true))";
    let step: WhereStep = ron::from_str(old).unwrap();
    assert_eq!(Step::Where(step), Step::filtered(Where::Missile, 2, ENEMY));

    /// `Step` as it was written before merging.
    #[derive(Serialize)]
    enum OldStep {
        Where {
            place: Where,
            count: u8,
            filters: Filters,
        },
        Effects(EffectStep),
    }
    let old_key = |steps: &[OldStep]| {
        let bytes = postcard::to_allocvec(steps).unwrap();
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in bytes {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        SpellKey(hash)
    };
    let heal = EffectStep {
        effects: vec![(Effect::Heal, 1)],
    };
    let was = [
        OldStep::Where {
            place: Where::Missile,
            count: 1,
            filters: Filters::NONE,
        },
        OldStep::Effects(heal.clone()),
    ];
    assert_eq!(SpellKey::of(&healing_missile()), old_key(&was));
    assert_eq!(
        SpellKey::of(&heal_spell()),
        old_key(&[OldStep::Effects(heal)])
    );
    // A merged step has a key of its own.
    let merged = Spell::new(vec![
        Step::merged(
            &[part(Where::Missile, 1), part(Where::Surround, 1)],
            Filters::NONE,
        )
        .unwrap(),
        Step::effects(&[(Effect::Heal, 1)]),
    ]);
    assert_ne!(SpellKey::of(&merged), SpellKey::of(&healing_missile()));
}
