//! `GroundShown`: what the ground section shows, while it is on screen, for
//! Cut Kindling to split a log lying there (woodcutting plan: only while the
//! inventory is open with the ground's storage shown).

use bevy::prelude::*;
use shared::item::Item;

use super::ground::{Ground, GroundHolder};

/// What is in the bags in reach, in the order the ground's grid shows them,
/// while that grid is on screen (the Character panel open on its Inventory
/// tab, with something in reach); empty otherwise.
#[derive(Resource, Default, Debug, PartialEq)]
pub struct GroundShown(pub Vec<Item>);

/// Keeps `GroundShown`: the grid counts as on screen while it has a size, as
/// a hidden page, a closed panel or a hidden section leaves it none.
pub(super) fn keep_ground_shown(
    ground: Res<Ground>,
    holders: Query<&ComputedNode, With<GroundHolder>>,
    mut shown: ResMut<GroundShown>,
) {
    let on_screen = holders.iter().any(|node| node.size().min_element() > 0.0);
    let items = match on_screen {
        true => ground.grid.items().cloned().collect(),
        false => Vec::new(),
    };
    shown.set_if_neq(GroundShown(items));
}
