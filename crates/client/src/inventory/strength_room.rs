//! A player's own grid sized by Strength (`shared::item::cells_for_strength`):
//! the server sizes it and says so with the inventory. Here, only what taking
//! off each worn thing would leave, so letting go of one where it would cut
//! cells holding something is refused as the server will refuse it
//! (`MoveRefused::StrengthInUse`), with words under the clothes.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Holder, Inventory, MoveRefused, cells_for_strength};

use super::Carried;
use super::hold::Target;
use crate::character::{CharacterSheet, WithBonuses};
use crate::common::GodMode;

/// The own cells the player would have without each thing they wear, by their
/// Strength with every bonus as the server last said, less what it gives.
#[derive(Resource, Default)]
pub(super) struct StrengthRoom(Vec<(Id, u32)>);

impl StrengthRoom {
    /// Why letting go of `id` with this aim would be refused for the room its
    /// Strength gives: only something worn, leaving the body.
    pub(super) fn refused(&self, carried: &Inventory, id: Id, aim: Target) -> Option<MoveRefused> {
        let &(_, cells) = self.0.iter().find(|(worn, _)| *worn == id)?;
        let mut after = carried.clone();
        match aim {
            Target::Nowhere | Target::Body { .. } | Target::BagSlot { .. } => return None,
            Target::Cells {
                holder: holder @ Holder::Inventory,
                x,
                y,
                ..
            } => {
                let at = u8::try_from(x).ok().zip(u8::try_from(y).ok())?;
                let turned = shared::item::Rotation::None;
                after.move_item(id, holder, at.0, at.1, turned).ok()?;
            }
            _ => {
                after.hand_over(id)?;
            }
        }
        after.check_own_cells(cells).err()
    }
}

/// Works out again what taking off each worn thing would leave, as what is
/// worn, the sheet or the attributes change.
pub(super) fn keep_strength_room(
    carried: Res<Carried>,
    sheet: Res<CharacterSheet>,
    with: Res<WithBonuses>,
    god: Res<GodMode>,
    mut room: ResMut<StrengthRoom>,
) {
    if !(carried.is_changed() || sheet.is_changed() || with.is_changed() || god.is_changed()) {
        return;
    }
    let worn = &carried.0.worn;
    // In god mode nothing is refused, as on the server.
    let Some(with) = with.0.filter(|_| !god.0 && carried.0.sized_by_strength()) else {
        room.0.clear();
        return;
    };
    // The effects count the same either way, so only the difference is taken.
    let now = sheet.0.attributes_with(worn, 0).strength as i32;
    room.0 = worn
        .iter()
        .map(|item| {
            let left: Vec<_> = worn
                .iter()
                .filter(|other| other.id != item.id)
                .cloned()
                .collect();
            let without = sheet.0.attributes_with(&left, 0).strength as i32;
            let strength = (with.strength as i32 + without - now).max(0) as u16;
            (item.id, cells_for_strength(strength))
        })
        .collect();
}

#[cfg(test)]
mod tests;
