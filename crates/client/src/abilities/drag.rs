//! Moving abilities about by mouse or finger (owner's design): dragged from the
//! Abilities page onto a slot puts it there; from one slot onto another swaps them;
//! from a slot onto the page takes it off the bar; anywhere else nothing changes.
//! An item dragged from a grid onto a slot puts its kind there (the item itself
//! goes back where it was; a potion puts that potion there, brewing decision
//! 22), and a kind of item or potion dragged from a slot onto any of the
//! Character panel takes it off (owner, 2026-11-15). A click or tap on a
//! slot uses it; a right-click or a finger held still on one turns its
//! automatic use on or off.
//!
//! The dragging itself (following the pointer or finger, being drawn, being let
//! go) is `dom`'s, as for items (`HeldObject`): `drag_abilities` picks up before
//! it, and `drop_abilities` says what a drop would do, and does it, after.

use bevy::prelude::*;
use shared::ability::{BarChange, BarThing, known};

use super::bar::count_of;
use super::item_look::thing_look;
use super::own_bar::OwnBar;
use super::page::usable;
use super::spots::{Spot, Spots};
use super::use_ability::UseSlot;
use crate::character::CharacterSheet;
use crate::common::BenchHere;
use crate::common::{TouchGesture, UiPointer, in_interface};
use crate::connection::ToServer;
use crate::dom::{AbilityThing, Dropped, Hand, HeldObject, HeldThing, HeldWhat};
use crate::inventory::Carried;

/// How far the mouse moves with its button down before a press is a drag.
const DRAG_PIXELS: f32 = 6.0;

/// Where a dragged ability came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum From {
    Page,
    Slot(u8),
}

/// Where the ability being dragged came from, and a mouse press that may become
/// a drag.
#[derive(Resource, Default)]
pub(super) struct Drag {
    from: Option<From>,
    pressed: Option<(BarThing, From, Vec2)>,
    /// Where the finger last stood a still press on, so it acts once.
    held_still: Option<Vec2>,
}

/// The slot something dragged is over, and whether it could go there.
#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub(super) struct Aim(pub Option<(u8, bool)>);

/// What can be picked up at a spot.
fn pick(spot: Option<Spot>, bar: &OwnBar) -> Option<(BarThing, From)> {
    match spot? {
        Spot::Listed(ability) => Some((ability.into(), From::Page)),
        Spot::Slot(slot) => bar.0.slots[slot as usize].map(|held| (held.thing, From::Slot(slot))),
        Spot::Page | Spot::Panel => None,
    }
}

/// What dropping `dragged` on `spot` asks for.
pub(super) fn drop_change(thing: BarThing, from: From, spot: Option<Spot>) -> Option<BarChange> {
    match (from, spot?, thing) {
        (From::Page, Spot::Slot(slot), BarThing::Ability(ability)) => {
            Some(BarChange::Place { slot, ability })
        }
        (From::Slot(a), Spot::Slot(b), _) if a != b => Some(BarChange::Swap { a, b }),
        (From::Slot(slot), Spot::Page | Spot::Listed(_), _) => Some(BarChange::Clear { slot }),
        (From::Slot(slot), Spot::Panel, BarThing::Item(_) | BarThing::Potion(_)) => {
            Some(BarChange::Clear { slot })
        }
        _ => None,
    }
}

/// Mouse and finger presses on abilities, before widgets and the map see them:
/// using a slot, turning its automatic use on or off, and picking an ability up
/// once it is dragged.
#[allow(clippy::too_many_arguments)]
pub(super) fn drag_abilities(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    pointer: Res<UiPointer>,
    gesture: Res<TouchGesture>,
    ui_scale: Res<UiScale>,
    spots: Spots,
    mut bar: ResMut<OwnBar>,
    (carried, sheet, here): (Res<Carried>, Res<CharacterSheet>, Res<BenchHere>),
    assets: Res<AssetServer>,
    (mut drag, mut object): (ResMut<Drag>, ResMut<HeldObject>),
    mut to_server: MessageWriter<ToServer>,
    mut used: MessageWriter<UseSlot>,
) {
    let kind = carried.0.creature;
    let known = known(kind, &sheet.0);
    let spot = pointer.0.and_then(|at| spots.at(at));
    // Something already held (an item, say) isn't for the bar.
    let free = object.thing().is_none();
    if mouse.just_pressed(MouseButton::Right)
        && free
        && let Some(Spot::Slot(slot)) = spot
    {
        mouse.clear_just_pressed(MouseButton::Right);
        bar.change(BarChange::ToggleAuto { slot }, &known, &mut to_server);
    }
    if mouse.just_pressed(MouseButton::Left)
        && free
        && let Some((thing, from)) = pick(spot, &bar)
    {
        mouse.clear_just_pressed(MouseButton::Left);
        drag.pressed = pointer.0.map(|at| (thing, from, at));
    }
    let mut pick_up = |thing: BarThing, from: From, by: Hand, drag: &mut Drag| {
        let count = count_of(thing, &carried.0);
        let usable = match thing {
            BarThing::Ability(ability) => usable(ability, &carried, &sheet, &here),
            BarThing::Potion(_) => count > 0,
            BarThing::Item(_) => true,
        };
        let thing = AbilityThing {
            thing,
            look: thing_look(thing, count, kind, usable, &assets),
        };
        let what = HeldWhat::Ability(thing);
        // (No place and time picked up: letting the button go drops it.)
        object.pick_up(HeldThing { what, by }, None);
        drag.from = Some(from);
    };
    if let (Some((thing, from, start)), Some(at)) = (drag.pressed, pointer.0)
        && mouse.pressed(MouseButton::Left)
        && at.distance(start) > DRAG_PIXELS
    {
        drag.pressed = None;
        pick_up(thing, from, Hand::Mouse, &mut drag);
        return;
    }
    // A finger: dragging from an ability picks it up; held still on a slot, it
    // turns automatic use on or off, once.
    if gesture.first_finger.is_none() {
        drag.held_still = None;
    }
    if free && gesture.drag != Vec2::ZERO {
        let start = in_interface(gesture.drag_start, &ui_scale);
        if let Some((thing, from)) = pick(spots.at(start), &bar) {
            pick_up(thing, from, Hand::Finger, &mut drag);
            return;
        }
    }
    if let Some(still) = gesture.still_at.map(|at| in_interface(at, &ui_scale))
        && free
        && drag.held_still != Some(still)
        && let Some(Spot::Slot(slot)) = spots.at(still)
    {
        drag.held_still = Some(still);
        bar.change(BarChange::ToggleAuto { slot }, &known, &mut to_server);
    }
    if free && mouse.just_released(MouseButton::Left) {
        if let Some((_, From::Slot(slot), _)) = drag.pressed {
            used.write(UseSlot(slot));
        }
        drag.pressed = None;
    }
}

/// What letting go of the ability (or kind of item) dragged would do, and
/// doing it when `dom` says it was let go; and of an item from a grid, whose
/// kind would go on a slot.
pub(super) fn drop_abilities(
    object: Res<HeldObject>,
    mut dropped: MessageReader<Dropped>,
    spots: Spots,
    mut bar: ResMut<OwnBar>,
    (carried, sheet): (Res<Carried>, Res<CharacterSheet>),
    (mut drag, mut aim): (ResMut<Drag>, ResMut<Aim>),
    mut to_server: MessageWriter<ToServer>,
) {
    let known = known(carried.0.creature, &sheet.0);
    let over = object.over.front.and_then(|front| spots.of(front));
    let change = object
        .thing()
        .and_then(|held| held_change(held, drag.from, over));
    let aimed = change.and_then(|change| match over {
        Some(Spot::Slot(slot)) => Some((slot, bar.0.clone().apply(change, &known).is_ok())),
        _ => None,
    });
    aim.set_if_neq(Aim(aimed));
    for drop in dropped.read() {
        let from = drag.from.take();
        let over = drop.over.front.and_then(|front| spots.of(front));
        if let Some(change) = held_change(&drop.thing, from, over) {
            bar.change(change, &known, &mut to_server);
        }
    }
    if object.thing().and_then(HeldThing::ability).is_none() {
        // Let go, or put back with Esc.
        drag.from = None;
    }
}

/// What letting go of this over `spot` would ask of the bar: an ability or
/// kind of item dragged from `from`, or an item from a grid, whose kind would
/// go in a slot.
fn held_change(held: &HeldThing, from: Option<From>, spot: Option<Spot>) -> Option<BarChange> {
    if let Some(item) = held.item() {
        let top = item.item.top();
        let Spot::Slot(slot) = spot? else {
            return None;
        };
        return Some(match top.potion_key() {
            Some(potion) => BarChange::PlacePotion { slot, potion },
            None => BarChange::PlaceItem {
                slot,
                item: top.kind,
            },
        });
    }
    drop_change(held.ability()?.thing, from?, spot)
}
