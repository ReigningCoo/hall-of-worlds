//! Building the spell by mouse or finger, one where or effect at a time (no
//! counts: one used twice is added twice). A click or tap on a piece along
//! the top adds it to the last step if it takes it (the same where again, an
//! effect to effects), else as a new last step; a filter goes into the first
//! step, above its where (only ever there, and only with a where to filter).
//! A piece dragged down goes where it is let go: into the step under it if
//! it takes it, else between two steps; a filter anywhere on the row into the
//! first step. A step, or one where or effect in a step, dragged moves the
//! same way, and anything dragged off the row is taken out, a filter too.
//!
//! What is dragged is `dom`'s (`HeldObject`), drawn as an ability is:
//! `press_design` picks up before it, `drop_on_design` says what a drop
//! would do, and does it, after.

use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::spell::Step;

use super::draft::{Draft, Goes};
use super::piece::{FilterKind, Piece};
use super::places::Places;
use super::spots::{DesignSpots, Spot};
use super::units::{Unit, units};
use crate::common::{TouchGesture, UiPointer, in_interface};
use crate::dom::{AbilityThing, Dropped, Hand, HeldObject, HeldThing, HeldWhat};

/// How far the mouse moves with its button down before a press is a drag.
const DRAG_PIXELS: f32 = 6.0;

/// What of the panel is being dragged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Dragging {
    /// A piece from along the top.
    Piece(Piece),
    /// The step at this place.
    Step(usize),
    /// One where or effect in a step: the step and its place there.
    Entry(usize, usize),
    /// A filter in the first step.
    Filter(FilterKind),
}

/// A mouse press on the panel that may become a drag.
#[derive(Resource, Default)]
pub(super) struct DesignDrag {
    pressed: Option<(Spot, Vec2)>,
}

/// What a drag would do, for the rows to show: what is dragged, the place in
/// the row it would go, or the effect step it would join.
#[derive(Resource, Default, Clone, Copy, PartialEq, Debug)]
pub(super) struct DesignAim {
    pub(super) dragging: Option<Dragging>,
    pub(super) insert: Option<usize>,
    pub(super) into: Option<usize>,
}

/// Presses on the panel, before widgets see them: clicks and taps add pieces;
/// dragging a piece, a step, or a where, effect or filter in a step picks it
/// up.
pub(super) fn press_design(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    (pointer, gesture, ui_scale): (Res<UiPointer>, Res<TouchGesture>, Res<UiScale>),
    spots: DesignSpots,
    mut draft: ResMut<Draft>,
    (mut drag, mut aim, mut object): (ResMut<DesignDrag>, ResMut<DesignAim>, ResMut<HeldObject>),
    assets: Res<AssetServer>,
) {
    if !draft.is_open() || draft.naming {
        drag.pressed = None;
        return;
    }
    let spot = pointer.0.and_then(|at| spots.at(at));
    let free = object.thing().is_none();
    if mouse.just_pressed(MouseButton::Left) && free && spot.is_some() {
        mouse.clear_just_pressed(MouseButton::Left);
        drag.pressed = spot.zip(pointer.0);
    }
    let mut pick_up = |spot: Spot, by: Hand, aim: &mut DesignAim| {
        let Some((dragging, piece)) = dragged(spot, &draft.steps) else {
            return false;
        };
        let thing = AbilityThing {
            thing: BarThing::Ability(Ability::Magic),
            look: piece.look(true, &assets),
        };
        let what = HeldWhat::Ability(thing);
        object.pick_up(HeldThing { what, by }, None);
        aim.dragging = Some(dragging);
        true
    };
    if let (Some((spot, start)), Some(at)) = (drag.pressed, pointer.0)
        && mouse.pressed(MouseButton::Left)
        && at.distance(start) > DRAG_PIXELS
    {
        drag.pressed = None;
        pick_up(spot, Hand::Mouse, &mut aim);
        return;
    }
    if free && gesture.drag != Vec2::ZERO {
        let start = in_interface(gesture.drag_start, &ui_scale);
        if let Some(spot) = spots.at(start)
            && pick_up(spot, Hand::Finger, &mut aim)
        {
            return;
        }
    }
    if free
        && mouse.just_released(MouseButton::Left)
        && let Some((spot, _)) = drag.pressed.take()
    {
        click(&mut draft, spot);
    }
}

/// What pressing on a spot picks up, and the piece it is drawn as.
fn dragged(spot: Spot, steps: &[Step]) -> Option<(Dragging, Piece)> {
    match spot {
        Spot::Piece(piece) => Some((Dragging::Piece(piece), piece)),
        Spot::Step(at) => {
            let piece = match steps.get(at)? {
                Step::Where(step) => Piece::Where(step.place),
                Step::Effects(step) => Piece::Effect(step.effects.first()?.0),
            };
            Some((Dragging::Step(at), piece))
        }
        Spot::Entry(at, n) => {
            let all = units(steps.get(at)?);
            let piece = all.get(n)?.piece();
            // Alone in its step, it moves the step (with its filters).
            match all.len() {
                1 => Some((Dragging::Step(at), piece)),
                _ => Some((Dragging::Entry(at, n), piece)),
            }
        }
        Spot::Filter(filter) => Some((Dragging::Filter(filter), Piece::Filter(filter))),
        Spot::End => None,
    }
}

/// A click or tap let go where it was pressed.
fn click(draft: &mut Draft, spot: Spot) {
    match spot {
        Spot::Piece(Piece::Where(place)) => draft.append(Unit::Where(place)),
        Spot::Piece(Piece::Effect(effect)) => draft.append(Unit::Effect(effect)),
        // Always the first step's, above its where.
        Spot::Piece(Piece::Filter(filter)) => draft.set_filter(filter, true),
        Spot::Step(_) | Spot::Entry(..) | Spot::Filter(_) | Spot::End => {}
    }
}

/// Where a drag would go, and making the change when it is let go.
pub(super) fn drop_on_design(
    object: Res<HeldObject>,
    mut dropped: MessageReader<Dropped>,
    spots: DesignSpots,
    places: Places,
    mut draft: ResMut<Draft>,
    mut aim: ResMut<DesignAim>,
) {
    let mut now = DesignAim {
        dragging: aim.dragging,
        ..default()
    };
    if let Some(dragging) = aim.dragging
        && object.thing().is_some()
    {
        let over = object.over.front.and_then(|front| spots.of(front));
        let gap = object.at.and_then(|at| places.insert_at(at));
        (now.insert, now.into) = target(dragging, over, gap, &draft);
    }
    for drop in dropped.read() {
        if drop.thing.ability().is_none() {
            continue;
        }
        let Some(dragging) = now.dragging.take() else {
            continue;
        };
        let over = drop.over.front.and_then(|front| spots.of(front));
        let gap = drop.at.and_then(|at| places.insert_at(at));
        let (insert, into) = target(dragging, over, gap, &draft);
        let draft = &mut *draft;
        match (dragging, insert, into) {
            (Dragging::Piece(Piece::Filter(filter)), _, Some(_)) => {
                draft.set_filter(filter, true);
            }
            (Dragging::Filter(filter), None, None) => draft.set_filter(filter, false),
            (Dragging::Piece(piece), _, Some(step)) => {
                if let Some(unit) = Unit::of(piece) {
                    draft.add_to(step, unit);
                }
            }
            (Dragging::Piece(piece), Some(at), _) => {
                if let Some(unit) = Unit::of(piece) {
                    draft.add_step(at, unit);
                }
            }
            (Dragging::Step(from), Some(to), _) => draft.shift(from, to),
            (Dragging::Step(from), None, _) => draft.remove(from),
            (Dragging::Entry(step, n), _, Some(to)) => draft.move_unit((step, n), Goes::Step(to)),
            (Dragging::Entry(step, n), Some(to), _) => draft.move_unit((step, n), Goes::Gap(to)),
            (Dragging::Entry(step, n), None, None) => {
                draft.remove_unit(step, n);
            }
            _ => {}
        }
        now.insert = None;
        now.into = None;
    }
    if object.thing().is_none() {
        // Let go, or put back with Esc.
        now = DesignAim::default();
    }
    aim.set_if_neq(now);
}

/// Where what is dragged would go when let go over `over`, `gap` being the
/// place in the row under the pointer: a place in the row, or a step that
/// takes it to join.
fn target(
    dragging: Dragging,
    over: Option<Spot>,
    gap: Option<usize>,
    draft: &Draft,
) -> (Option<usize>, Option<usize>) {
    let over_step = over.and_then(Spot::step);
    let unit = match dragging {
        // Anywhere on the row: they only ever go in the first step.
        Dragging::Piece(Piece::Filter(_)) => {
            let first_where = matches!(draft.steps.first(), Some(Step::Where(_)));
            let on_row = gap.is_some() || over_step.is_some();
            return (None, (on_row && first_where).then_some(0));
        }
        // Off the row it comes off; on it, it stays.
        Dragging::Filter(_) => return (gap.or(over_step), None),
        Dragging::Step(_) => return (gap, None),
        Dragging::Piece(piece) => Unit::of(piece),
        Dragging::Entry(at, n) => draft
            .steps
            .get(at)
            .and_then(|step| units(step).get(n).copied()),
    };
    match (unit, over_step) {
        (Some(unit), Some(at)) if draft.takes(at, unit) => (None, Some(at)),
        _ => (gap, None),
    }
}
