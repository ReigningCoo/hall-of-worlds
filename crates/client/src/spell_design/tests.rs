use shared::spell::{Effect, Filters, Step, Where, WherePart};

use super::draft::{Draft, Goes};
use super::markup::SPELL_DESIGN;
use super::piece::FilterKind;
use super::right_click::remove;
use super::spots::Spot;
use super::units::{MOST_REPEATS, Unit};
use crate::common::{MAGIC_ICON, spell_icon};

#[test]
fn the_panel_is_good_markup() {
    SPELL_DESIGN.assert_good();
}

#[test]
fn clicks_build_a_double_healing_missile() {
    let mut draft = Draft::default();
    draft.append(Unit::Where(Where::Missile));
    draft.append(Unit::Where(Where::Missile));
    draft.append(Unit::Effect(Effect::Heal));
    draft.append(Unit::Effect(Effect::Heal));
    assert_eq!(
        draft.steps,
        vec![
            Step::at(Where::Missile, 2),
            Step::effects(&[(Effect::Heal, 2)])
        ]
    );
}

#[test]
fn a_different_where_starts_a_new_step() {
    let mut draft = Draft::default();
    draft.append(Unit::Where(Where::Touch));
    draft.append(Unit::Where(Where::Missile));
    assert_eq!(draft.steps.len(), 2);
    assert!(!draft.takes(1, Unit::Effect(Effect::Fire)), "never both");
}

#[test]
fn effects_keep_their_order() {
    let mut draft = Draft::default();
    for effect in [Effect::Void, Effect::Fire, Effect::Fire, Effect::Water] {
        draft.append(Unit::Effect(effect));
    }
    let expected = [(Effect::Void, 1), (Effect::Fire, 2), (Effect::Water, 1)];
    assert_eq!(draft.steps, vec![Step::effects(&expected)]);
    // Taking out the Void.
    draft.remove_unit(0, 0);
    let expected = [(Effect::Fire, 2), (Effect::Water, 1)];
    assert_eq!(draft.steps, vec![Step::effects(&expected)]);
}

#[test]
fn filters_go_on_the_first_where_only() {
    let mut draft = Draft::default();
    draft.set_filter(FilterKind::Enemy, true);
    assert!(!draft.has_filter(FilterKind::Enemy), "no where to filter");
    draft.append(Unit::Effect(Effect::Fire));
    draft.set_filter(FilterKind::Enemy, true);
    assert!(!draft.has_filter(FilterKind::Enemy), "nor in effects");
    draft.add_step(0, Unit::Where(Where::Touch));
    draft.set_filter(FilterKind::Enemy, true);
    let filters = Filters {
        enemy: true,
        ..Filters::NONE
    };
    assert_eq!(
        draft.spell().steps[0],
        Step::filtered(Where::Touch, 1, filters)
    );
    // They stay with the first step: moved after Fire, the Touch loses them,
    // and they show as wrong until a where is first again.
    draft.shift(0, 2);
    assert_eq!(draft.spell().steps[1], Step::at(Where::Touch, 1));
    assert!(draft.filters_stranded());
    draft.shift(1, 0);
    assert!(!draft.filters_stranded());
    // Taking out every step forgets them.
    draft.remove(0);
    draft.remove(0);
    assert!(!draft.has_filter(FilterKind::Enemy));
}

#[test]
fn repeats_stop_at_the_most() {
    let mut draft = Draft::default();
    for _ in 0..MOST_REPEATS + 3 {
        draft.append(Unit::Where(Where::Touch));
    }
    assert_eq!(draft.steps[0], Step::at(Where::Touch, MOST_REPEATS as u8));
    assert_eq!(
        draft.steps[1],
        Step::at(Where::Touch, 3),
        "the rest start a new step"
    );
}

#[test]
fn steps_move_and_come_out() {
    let mut draft = Draft::default();
    draft.append(Unit::Effect(Effect::Heal));
    draft.append(Unit::Where(Where::Missile));
    draft.shift(1, 0);
    assert_eq!(draft.steps[0], Step::at(Where::Missile, 1));
    draft.remove(0);
    assert_eq!(draft.steps, vec![Step::effects(&[(Effect::Heal, 1)])]);
}

#[test]
fn wheres_and_effects_move_between_steps() {
    let mut draft = Draft::default();
    draft.append(Unit::Effect(Effect::Fire));
    draft.append(Unit::Where(Where::Touch));
    draft.append(Unit::Effect(Effect::Water));
    // Fire, alone in the first step, joins Water: its step goes.
    draft.move_unit((0, 0), Goes::Step(2));
    assert_eq!(
        draft.steps,
        vec![
            Step::at(Where::Touch, 1),
            Step::effects(&[(Effect::Water, 1), (Effect::Fire, 1)])
        ]
    );
    // A where won't go into effects: it stays in its own step.
    draft.append(Unit::Where(Where::Touch));
    draft.add_to(0, Unit::Where(Where::Touch));
    draft.move_unit((0, 1), Goes::Step(1));
    assert_eq!(
        draft.steps[1],
        Step::effects(&[(Effect::Water, 1), (Effect::Fire, 1)])
    );
    // And out again, into a new step at the start.
    draft.move_unit((1, 1), Goes::Gap(0));
    assert_eq!(draft.steps[0], Step::effects(&[(Effect::Fire, 1)]));
}

#[test]
fn no_more_than_ten_steps() {
    let mut draft = Draft::default();
    for at in 0..12 {
        let place = if at % 2 == 0 {
            Where::Touch
        } else {
            Where::Missile
        };
        draft.append(Unit::Where(place));
    }
    assert_eq!(draft.steps.len(), shared::spell::MAX_STEPS);
}

#[test]
fn a_where_dragged_onto_a_where_step_merges_after_the_rest() {
    let mut draft = Draft::default();
    draft.append(Unit::Where(Where::Missile));
    draft.append(Unit::Where(Where::Surround));
    draft.append(Unit::Effect(Effect::Fire));
    // Clicks keep separate steps; Surround dragged onto Missile merges.
    assert_eq!(draft.steps.len(), 3);
    draft.move_unit((1, 0), Goes::Step(0));
    let parts = [
        WherePart {
            place: Where::Missile,
            count: 1,
        },
        WherePart {
            place: Where::Surround,
            count: 1,
        },
    ];
    let merged = Step::merged(&parts, Filters::NONE).unwrap();
    assert_eq!(
        draft.steps,
        vec![merged, Step::effects(&[(Effect::Fire, 1)])]
    );
    // A click of the last where again stacks it; another kind starts a step.
    draft.add_step(1, Unit::Where(Where::Touch));
    draft.move_unit((1, 0), Goes::Step(0));
    draft.add_to(0, Unit::Where(Where::Touch));
    let Step::Where(step) = &draft.steps[0] else {
        panic!("a where step");
    };
    assert_eq!(step.last().count, 2);
    assert_eq!(step.len(), 3);
    // Taken out again one at a time, the rest close up.
    assert_eq!(draft.remove_unit(0, 1), Some(Unit::Where(Where::Surround)));
    let Step::Where(step) = &draft.steps[0] else {
        panic!("a where step");
    };
    assert_eq!((step.len(), step.last().place), (2, Where::Touch));
}

#[test]
fn a_right_click_takes_out_a_where_effect_or_filter_but_not_a_piece() {
    let mut draft = Draft::default();
    draft.append(Unit::Where(Where::Missile));
    draft.append(Unit::Effect(Effect::Heal));
    draft.append(Unit::Effect(Effect::Fire));
    draft.set_filter(FilterKind::Enemy, true);
    assert!(!remove(
        &mut draft,
        Spot::Piece(super::piece::Piece::Where(Where::Beam))
    ));
    assert!(remove(&mut draft, Spot::Entry(1, 0)));
    assert_eq!(draft.steps[1], Step::effects(&[(Effect::Fire, 1)]));
    assert!(remove(&mut draft, Spot::Filter(FilterKind::Enemy)));
    assert!(!draft.has_filter(FilterKind::Enemy));
    // The last in its step takes the step with it.
    assert!(remove(&mut draft, Spot::Entry(1, 0)));
    assert_eq!(draft.steps.len(), 1);
}

#[test]
fn a_design_healing_first_shows_heals_icon_else_magics() {
    let mut draft = Draft::default();
    draft.append(Unit::Effect(Effect::Heal));
    assert_ne!(spell_icon(&draft.spell()), MAGIC_ICON);
    draft.clear();
    draft.append(Unit::Effect(Effect::Fire));
    assert_eq!(spell_icon(&draft.spell()), MAGIC_ICON);
}
