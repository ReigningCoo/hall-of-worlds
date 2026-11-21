//! Building the process by mouse or finger: a tool dragged down from the top
//! row goes in as a step on the empty place it is let go on, or at the end of
//! the top row on a click or tap; a step dragged to an empty place moves
//! there, and dragged off the grid is taken out; a click or tap on a step
//! joins both rows (or splits a join). An item dragged from a grid onto a
//! drop space goes in if it may (the item itself goes back where it was, as
//! on the action bar); a click or tap on a filled space empties it, and on
//! the water faced adds 100 ml. Nothing changes while it runs.
//!
//! What is dragged is `dom`'s (`HeldObject`), drawn as a kind of item is on
//! the action bar: `press_bench` picks up before it, `drop_on_bench` says what
//! a drop would do, and does it, after.

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use shared::ability::{Ability, BarThing};
use shared::cooking::Tool;
use shared::id::Id;
use shared::item::Item;

use super::bench::Bench;
use super::boxes::tool_item;
use super::grid::{GridCanvas, PlaceNode, SpaceNode, StepNode, ToolNode};
use super::kitchen::Kitchens;
use super::spaces::{Place, SpaceIcons, in_place, more_faced, put, takes};
use crate::common::{TouchGesture, UiPointer, in_interface};
use crate::dom::{AbilityThing, Dropped, Finder, Hand, HeldObject, HeldThing, HeldWhat};
use crate::inventory::Carried;
use crate::widgets::{AbilityLook, GRID_COLOUR, logical_rect};

/// How far the mouse moves with its button down before a press is a drag.
const DRAG_PIXELS: f32 = 6.0;

/// What of the panel is being dragged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Dragging {
    /// A tool from the top row.
    New(Tool),
    /// The step at this index.
    Step(usize),
}

/// What is at a point of the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spot {
    Tool(Tool),
    Step(usize),
    Place(u8, u8),
    Space(Place),
}

/// A mouse press on the panel that may become a drag.
#[derive(Resource, Default)]
pub(super) struct BenchDrag {
    pressed: Option<(Spot, Vec2)>,
}

/// What a drag would do, for the grid to show: what is dragged, the empty
/// place it is over with whether it may go there, and the drop space an item
/// is over with whether it may go there.
#[derive(Resource, Default, Clone, Copy, PartialEq, Debug)]
pub(super) struct BenchAim {
    pub(super) dragging: Option<Dragging>,
    pub(super) place: Option<((u8, u8), bool)>,
    pub(super) space: Option<(Place, bool)>,
}

/// Finds what of the panel is at a point, or a node is in.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct BenchSpots<'w, 's> {
    finder: Finder<'w, 's>,
    parents: Query<'w, 's, &'static ChildOf>,
    tools: Query<'w, 's, &'static ToolNode>,
    steps: Query<'w, 's, &'static StepNode>,
    places: Query<'w, 's, &'static PlaceNode>,
    spaces: Query<'w, 's, &'static SpaceNode>,
}

impl BenchSpots<'_, '_> {
    fn at(&self, at: Vec2) -> Option<Spot> {
        self.of(self.finder.target_at(at)?)
    }

    fn of(&self, front: Entity) -> Option<Spot> {
        let up = std::iter::once(front).chain(self.parents.iter_ancestors(front));
        up.into_iter().find_map(|entity| {
            if let Ok(node) = self.tools.get(entity) {
                Some(Spot::Tool(node.0))
            } else if let Ok(node) = self.steps.get(entity) {
                Some(Spot::Step(node.0))
            } else if let Ok(node) = self.places.get(entity) {
                Some(Spot::Place(node.0, node.1))
            } else {
                self.spaces.get(entity).ok().map(|node| Spot::Space(node.0))
            }
        })
    }
}

/// Presses on the panel, before widgets see them: clicks and taps add a
/// step, join or split one, empty a space or add water faced; dragging a
/// tool or a step picks it up.
#[allow(clippy::too_many_arguments)]
pub(super) fn press_bench(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    (pointer, gesture, ui_scale): (Res<UiPointer>, Res<TouchGesture>, Res<UiScale>),
    spots: BenchSpots,
    (icons, carried): (Res<SpaceIcons>, Res<Carried>),
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
        let (dragging, tool) = match spot {
            Spot::Tool(tool) => (Dragging::New(tool), tool),
            Spot::Step(at) => (Dragging::Step(at), bench.steps[at].tool),
            Spot::Place(..) | Spot::Space(_) => return false,
        };
        let kind = tool_item(tool);
        let look = AbilityLook {
            icon: if kind.is_none() {
                icons.tool(tool)
            } else {
                Handle::default()
            },
            name: tool.name().to_owned(),
            passive: false,
            usable: true,
            colour: GRID_COLOUR,
            item: kind.map(|kind| Item::new(Id::stand_in(0), kind)),
        };
        let thing = AbilityThing {
            thing: kind.map_or(BarThing::Ability(Ability::Cook), BarThing::Item),
            look,
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
    if free && mouse.just_released(MouseButton::Left) {
        match drag.pressed.take() {
            Some((Spot::Tool(tool), _)) => {
                bench.add(tool);
            }
            Some((Spot::Step(at), _)) => {
                bench.toggle_join(at);
            }
            Some((Spot::Space(Place::Faced(step)), _)) => more_faced(&mut bench, step),
            Some((Spot::Space(place), _)) if in_place(&bench, place).is_some() => {
                put(&mut bench, &carried.0, place, None);
            }
            _ => {}
        }
    }
}

/// Where a drag would go, and making the change when it is let go.
#[allow(clippy::too_many_arguments)]
pub(super) fn drop_on_bench(
    object: Res<HeldObject>,
    mut dropped: MessageReader<Dropped>,
    spots: BenchSpots,
    grid: Query<(&ComputedNode, &UiGlobalTransform), With<GridCanvas>>,
    (carried, kitchens): (Res<Carried>, Res<Kitchens>),
    mut bench: ResMut<Bench>,
    mut aim: ResMut<BenchAim>,
) {
    let mut now = BenchAim {
        dragging: aim.dragging,
        ..default()
    };
    let height = |bench: &Bench, dragging: Dragging| match dragging {
        Dragging::New(_) => (1, None),
        Dragging::Step(at) => (bench.steps.get(at).map_or(1, |s| s.height), Some(at)),
    };
    if let Some(held) = object.thing() {
        let over = object.over.front.and_then(|f| spots.of(f));
        if held.ability().is_some()
            && let Some(dragging) = aim.dragging
            && let Some(Spot::Place(column, row)) = over
        {
            let (tall, except) = height(&bench, dragging);
            let row = if tall > 1 { 0 } else { row };
            let ok = bench.free(column, row, tall, except);
            now.place = Some(((column, row), ok));
        }
        if let Some(item) = held.item()
            && let Some(Spot::Space(place)) = over
        {
            let ok = !bench.is_running() && takes(&carried.0, &kitchens.0, place, item.id);
            now.space = Some((place, ok));
        }
    }
    for drop in dropped.read() {
        let over = drop.over.front.and_then(|front| spots.of(front));
        if let Some(item) = drop.thing.item()
            && let Some(Spot::Space(place)) = over
            && !bench.is_running()
            && takes(&carried.0, &kitchens.0, place, item.id)
        {
            put(&mut bench, &carried.0, place, Some(item.id));
        }
        if drop.thing.ability().is_none() {
            continue;
        }
        let Some(dragging) = now.dragging.take() else {
            continue;
        };
        let on_grid = drop.at.is_some_and(|at| {
            grid.iter().any(|(node, transform)| {
                let (corner, size) = logical_rect(node, transform);
                let inside =
                    |v: f32, from: f32, length: f32| v >= from - 8.0 && v <= from + length + 8.0;
                inside(at.x, corner.x, size.x) && inside(at.y, corner.y, size.y)
            })
        });
        match (dragging, over) {
            (Dragging::New(tool), Some(Spot::Place(column, row))) => {
                bench.place(tool, column, row);
            }
            (Dragging::Step(from), Some(Spot::Place(column, row))) => {
                bench.move_to(from, column, row);
            }
            (Dragging::Step(from), _) if !on_grid => bench.remove(from),
            _ => {}
        }
        now.place = None;
    }
    if object.thing().is_none() {
        // Let go, or put back with Esc.
        now = BenchAim::default();
    }
    aim.set_if_neq(now);
}
