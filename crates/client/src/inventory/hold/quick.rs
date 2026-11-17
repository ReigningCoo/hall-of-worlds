//! Quick moves (owner, 2026-11-17): a right-click (by touch, a double tap) on
//! an item not held sends it to "the other side", if it can go. Where that is,
//! without Bevy: the doing is `hold_items::quick`, the same as letting go there.

use shared::id::Id;
use shared::item::{Holder, Inventory, Item, MoveRefused, Rotation, StackLimits};

use super::{Held, Target};

/// What was right-clicked.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::inventory) enum QuickFrom {
    /// Something carried, by its id.
    Carried(Id),
    /// Something on the ground (`common::GROUND`'s stock).
    Ground(Item),
}

/// Why it can't go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::inventory) enum QuickRefused {
    /// The inventory's rules say no.
    Move(MoveRefused),
    /// There is no ground here to drop it on (the world map).
    NoGround,
}

impl QuickRefused {
    /// The red line's words.
    pub(in crate::inventory) fn words(self) -> String {
        match self {
            QuickRefused::Move(why) => super::super::bag_slots::explain_move(why),
            QuickRefused::NoGround => "There is no ground here to put it on.".into(),
        }
    }
}

/// Where a quick move of this would go, as a hold and a drop there would do it.
/// `None` when nothing happens at all (something worn or in the bin).
///
/// - From the ground: into the own grid, on the first stack it all fits on, or
///   else the first place it fits (as `Inventory::give`).
/// - From the own grid (or a bag tied to the belt, part of it): into the bag
///   panel opened or raised last (`open_bags`, the last is the latest) that it
///   may go in, the same way; with no bag panel open, onto the ground, as on the
///   Drop icon.
/// - From any other bag: into the own grid, the same way.
pub(in crate::inventory) fn quick_target(
    inventory: &Inventory,
    from: &QuickFrom,
    open_bags: &[Id],
    limits: StackLimits,
    has_ground: bool,
) -> Option<Result<(Held, Target), QuickRefused>> {
    let id = match from {
        QuickFrom::Ground(item) => return Some(into_new(inventory, item, limits)),
        QuickFrom::Carried(id) => *id,
    };
    if inventory.in_bin(id) {
        return None;
    }
    let (holder, placed) = inventory.placed(id)?;
    let item = &placed.item;
    let in_own_grid = match holder {
        Holder::Inventory => true,
        Holder::Inside(bag) => inventory.is_tied(bag),
    };
    if !in_own_grid {
        return Some(into_carried(inventory, item, Holder::Inventory, limits));
    }
    // Not into itself, or anything in it.
    let bag = open_bags.iter().rev().copied().find(|&bag| {
        let into_itself = inventory.check_move(id, Holder::Inside(bag), 0, 0, Rotation::None)
            == Err(MoveRefused::IntoItself);
        !into_itself && !inventory.in_bin(bag) && inventory.find(bag).is_some()
    });
    match bag {
        Some(bag) => Some(into_carried(inventory, item, Holder::Inside(bag), limits)),
        None if has_ground => Some(Ok((held(id), Target::Drop { allowed: true }))),
        None => Some(Err(QuickRefused::NoGround)),
    }
}

/// Holding it by its first cell, unturned.
fn held(id: Id) -> Held {
    Held {
        id,
        rotation: Rotation::None,
        grip: (0, 0),
    }
}

/// A carried item into this grid: its first stack it all fits on, or its first
/// free place.
fn into_carried(
    inventory: &Inventory,
    item: &Item,
    holder: Holder,
    limits: StackLimits,
) -> Result<(Held, Target), QuickRefused> {
    let id = item.id;
    if let Some(onto) = inventory
        .first_stack_for(item, holder, limits)
        .filter(|&onto| onto != id)
    {
        let aim = Target::Stack {
            onto,
            allowed: true,
        };
        return Ok((held(id), aim));
    }
    let Some((x, y, rotation)) = inventory.first_place_for(id, holder, Rotation::None) else {
        let why = inventory.check_move(id, holder, 0, 0, Rotation::None).err();
        return Err(QuickRefused::Move(why.unwrap_or(MoveRefused::NoRoom)));
    };
    let held = Held {
        rotation,
        ..held(id)
    };
    let aim = Target::Cells {
        holder,
        x: x as i32,
        y: y as i32,
        allowed: true,
    };
    Ok((held, aim))
}

/// Something not carried into the own grid, the same way.
fn into_new(
    inventory: &Inventory,
    item: &Item,
    limits: StackLimits,
) -> Result<(Held, Target), QuickRefused> {
    let id = item.id;
    let holder = Holder::Inventory;
    if let Some(onto) = inventory.first_stack_for(item, holder, limits) {
        let aim = Target::Stack {
            onto,
            allowed: true,
        };
        return Ok((held(id), aim));
    }
    let Some((x, y, rotation)) = inventory.first_place_for_new(item, holder, Rotation::None) else {
        let why = inventory
            .check_add(item, holder, 0, 0, Rotation::None)
            .err();
        return Err(QuickRefused::Move(why.unwrap_or(MoveRefused::NoRoom)));
    };
    let held = Held {
        rotation,
        ..held(id)
    };
    let aim = Target::Cells {
        holder,
        x: x as i32,
        y: y as i32,
        allowed: true,
    };
    Ok((held, aim))
}

#[cfg(test)]
mod tests;
