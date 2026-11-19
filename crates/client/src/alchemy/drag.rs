//! Building the process by mouse or finger: an apparatus dragged down from the
//! top row goes in as a step where it is let go (between two, or at the end),
//! or at the end on a click or tap; a step dragged along the row moves there,
//! and dragged off the row is taken out. An item dragged from a grid onto a
//! drop space goes in if it may (the item itself goes back where it was, as on
//! the action bar); a click or tap on a filled space empties it. Nothing
//! changes while it runs.
//!
//! What is dragged is `dom`'s (`HeldObject`), drawn as a kind of item is on
//! the action bar: `press_bench` picks up before it, `drop_on_bench` says what
//! a drop would do, and does it, after.

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use shared::ability::BarThing;
use shared::alchemy::Apparatus;
use shared::id::Id;
use shared::item::Item;

use super::bench::Bench;
use super::rows::{ApparatusNode, EndNode, ProcessRow, SpaceNode, StepNode};
use super::spaces::{Place, in_place, put, takes};
use crate::common::{TouchGesture, UiPointer, in_interface};
use crate::dom::{AbilityThing, Dropped, Finder, Hand, HeldObject, HeldThing, HeldWhat};
use crate::inventory::Carried;
use crate::widgets::{AbilityLook, GRID_COLOUR, logical_rect};

/// How far the mouse moves with its button down before a press is a drag.
const DRAG_PIXELS: f32 = 6.0;

/// What of the panel is being dragged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Dragging {
    /// An apparatus from the top row.
    New(Apparatus),
    /// The step at this place.
    Step(usize),
}

/// What is at a point of the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spot {
    Apparatus(Apparatus),
    Step(usize),
    Space(Place),
}

/// A mouse press on the panel that may become a drag.
#[derive(Resource, Default)]
pub(super) struct BenchDrag {
    pressed: Option<(Spot, Vec2)>,
}

/// What a drag would do, for the rows to show: what is dragged, the place in
/// the row it would go, and the drop space an item is over with whether it
/// may go there.
#[derive(Resource, Default, Clone, Copy, PartialEq, Debug)]
pub(super) struct BenchAim {
    pub(super) dragging: Option<Dragging>,
    pub(super) insert: Option<usize>,
    pub(super) space: Option<(Place, bool)>,
}

/// Finds what of the panel is at a point, or a node is in.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct BenchSpots<'w, 's> {
    finder: Finder<'w, 's>,
    parents: Query<'w, 's, &'static ChildOf>,
    kit: Query<'w, 's, &'static ApparatusNode>,
    steps: Query<'w, 's, &'static StepNode>,
    spaces: Query<'w, 's, &'static SpaceNode>,
}

impl BenchSpots<'_, '_> {
    fn at(&self, at: Vec2) -> Option<Spot> {
        self.of(self.finder.target_at(at)?)
    }

    fn of(&self, front: Entity) -> Option<Spot> {
        let up = std::iter::once(front).chain(self.parents.iter_ancestors(front));
        up.into_iter().find_map(|entity| {
            if let Ok(node) = self.kit.get(entity) {
                Some(Spot::Apparatus(node.0))
            } else if let Ok(node) = self.steps.get(entity) {
                Some(Spot::Step(node.0))
            } else {
                self.spaces.get(entity).ok().map(|node| Spot::Space(node.0))
            }
        })
    }
}

/// Presses on the panel, before widgets see them: clicks and taps add a step
/// or empty a space; dragging an apparatus or a step picks it up.
#[allow(clippy::too_many_arguments)]
pub(super) fn press_bench(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    (pointer, gesture, ui_scale): (Res<UiPointer>, Res<TouchGesture>, Res<UiScale>),
    spots: BenchSpots,
    mut bench: ResMut<Bench>,
    (mut drag, mut aim, mut object): (ResMut<BenchDrag>, ResMut<BenchAim>, ResMut<HeldObject>),
) {
    if !bench.is_open() {
        drag.pressed = None;
        return;
    }
    let spot = pointer.0.and_then(|at| spots.at(at));
    let free = object.thing().is_none();
    if mouse.just_pressed(MouseButton::Left) && free && spot.is_some() {
        mouse.clear_just_pressed(MouseButton::Left);
        drag.pressed = spot.zip(pointer.0);
    }
    if bench.is_running() {
        // Locked: presses on it do nothing.
        drag.pressed = None;
        return;
    }
    let mut pick_up = |spot: Spot, by: Hand, aim: &mut BenchAim| {
        let (dragging, apparatus) = match spot {
            Spot::Apparatus(apparatus) => (Dragging::New(apparatus), apparatus),
            Spot::Step(at) => (Dragging::Step(at), bench.steps[at].apparatus),
            Spot::Space(_) => return false,
        };
        let Some(kind) = apparatus.item() else {
            return false;
        };
        let look = AbilityLook {
            icon: Handle::default(),
            name: apparatus.name().to_owned(),
            passive: false,
            usable: true,
            colour: GRID_COLOUR,
            item: Some(Item::new(Id::stand_in(0), kind)),
        };
        let thing = AbilityThing {
            thing: BarThing::Item(kind),
            look,
        };
        object.pick_up(
            HeldThing {
                what: HeldWhat::Ability(thing),
                by,
            },
            None,
        );
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
    if free && mouse.just_released(MouseButton::Left) {
        match drag.pressed.take() {
            Some((Spot::Apparatus(apparatus), _)) => {
                let end = bench.steps.len();
                bench.insert(end, apparatus);
            }
            Some((Spot::Space(place), _)) if in_place(&bench, place).is_some() => {
                put(&mut bench, place, None);
            }
            _ => {}
        }
    }
}

/// Where a drag would go, and making the change when it is let go.
pub(super) fn drop_on_bench(
    object: Res<HeldObject>,
    mut dropped: MessageReader<Dropped>,
    spots: BenchSpots,
    places: Places,
    carried: Res<Carried>,
    mut bench: ResMut<Bench>,
    mut aim: ResMut<BenchAim>,
) {
    let mut now = BenchAim {
        dragging: aim.dragging,
        ..default()
    };
    if let Some(held) = object.thing() {
        if held.ability().is_some() && aim.dragging.is_some() {
            now.insert = object.at.and_then(|at| places.insert_at(at));
        }
        if let Some(item) = held.item()
            && let Some(Spot::Space(place)) = object.over.front.and_then(|f| spots.of(f))
        {
            let ok = !bench.is_running() && takes(&bench, &carried.0, place, item.id);
            now.space = Some((place, ok));
        }
    }
    for drop in dropped.read() {
        let over = drop.over.front.and_then(|front| spots.of(front));
        if let Some(item) = drop.thing.item()
            && let Some(Spot::Space(place)) = over
            && !bench.is_running()
            && takes(&bench, &carried.0, place, item.id)
        {
            put(&mut bench, place, Some(item.id));
        }
        if drop.thing.ability().is_none() {
            continue;
        }
        let Some(dragging) = now.dragging.take() else {
            continue;
        };
        let insert = drop.at.and_then(|at| places.insert_at(at));
        match (dragging, insert) {
            (Dragging::New(apparatus), Some(at)) => bench.insert(at, apparatus),
            (Dragging::Step(from), Some(to)) => bench.shift(from, to),
            (Dragging::Step(from), None) => bench.remove(from),
            (Dragging::New(_), None) => {}
        }
        now.insert = None;
    }
    if object.thing().is_none() {
        // Let go, or put back with Esc.
        now = BenchAim::default();
    }
    aim.set_if_neq(now);
}

/// Where the process row and its steps are.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct Places<'w, 's> {
    row: Query<'w, 's, (&'static ComputedNode, &'static UiGlobalTransform), With<ProcessRow>>,
    steps: Query<
        'w,
        's,
        (
            &'static StepNode,
            &'static ComputedNode,
            &'static UiGlobalTransform,
        ),
    >,
    end: Query<'w, 's, (&'static ComputedNode, &'static UiGlobalTransform), With<EndNode>>,
}

impl Places<'_, '_> {
    /// The place in the row a step let go at `at` (interface pixels) goes:
    /// before the first step whose middle is right of it; none off the row.
    fn insert_at(&self, at: Vec2) -> Option<usize> {
        let (node, transform) = self.row.iter().next()?;
        let (corner, size) = logical_rect(node, transform);
        let (end_corner, end_size) = (self.end.iter().next())
            .map_or((corner, size), |(node, transform)| {
                logical_rect(node, transform)
            });
        // The row's height is its boxes'; its width runs to the end box.
        let right = (end_corner.x + end_size.x).max(corner.x + size.x);
        let inside = at.x >= corner.x - 8.0
            && at.x <= right + 8.0
            && at.y >= corner.y - 8.0
            && at.y <= corner.y + size.y + 8.0;
        if !inside {
            return None;
        }
        let before = self.steps.iter().filter(|(_, node, transform)| {
            let (corner, size) = logical_rect(node, transform);
            corner.x + size.x / 2.0 < at.x
        });
        Some(before.count())
    }
}
