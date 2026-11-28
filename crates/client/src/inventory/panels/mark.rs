//! Making the cells `dom`'s grids and slots draw ours.

use bevy::prelude::*;

use super::super::bag_slots::BagSlotNode;
use super::super::body::BodyNode;
use super::{BinNode, GridOwner, ItemCell};
use crate::dom::{SlotShown, ThingCell};

/// What a slot is part of: the clothes, the bin or a bag slot.
type SlotMarks = (Has<BodyNode>, Has<BinNode>, Option<&'static BagSlotNode>);

/// Marks each cell of an item in a grid or a slot, for finding what the
/// pointer is over and picking things up.
pub(in crate::inventory) fn mark_grids(
    cells: Query<(Entity, &ThingCell), Added<ThingCell>>,
    owners: Query<(), With<GridOwner>>,
    slots: Query<SlotMarks, With<SlotShown>>,
    mut commands: Commands,
) {
    for (entity, cell) in &cells {
        let ours = ItemCell {
            id: cell.item,
            rotation: cell.rotation,
            grip: cell.grip,
        };
        let mut entity = commands.entity(entity);
        // What is in a slot is part of the clothes, the bin or a bag slot, as
        // the slot.
        match slots.get(cell.grid) {
            Ok((true, _, _)) => entity.try_insert((ours, BodyNode)),
            Ok((_, true, _)) => entity.try_insert((ours, BinNode)),
            Ok((_, _, Some(&slot))) => entity.try_insert((ours, slot)),
            Ok(_) => continue,
            Err(_) if owners.contains(cell.grid) => entity.try_insert(ours),
            Err(_) => continue,
        };
    }
}
