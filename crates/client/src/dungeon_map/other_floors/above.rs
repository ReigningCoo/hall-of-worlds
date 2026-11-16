//! What stands on a building's floor: the floor above it, if any, which
//! decides where its walls' tops get battlements (`walls::Above`).

use shared::dungeon::Level;
use shared::protocol::LevelInfo;

use super::super::walls::Above;
use super::which::has_floors_below;
use crate::common::OtherLevels;

/// What stands on floor `index` of the site `info` describes: the floor above
/// as `level` finds it (the player's own too), nothing on the top storey, in
/// a basement or anywhere but a building.
pub(in crate::dungeon_map) fn above_floor<'a>(
    info: &LevelInfo,
    index: u32,
    level: impl Fn(u32) -> Option<&'a Level>,
) -> Above<'a> {
    let storeys = info.levels - u32::from(info.basement);
    if !info.building || info.storey(index) < 0 || index + 1 >= storeys {
        return Above::Nothing;
    }
    level(index + 1).map_or(Above::Unknown, Above::Floor)
}

/// For the player's own floor: whether the floors below are drawn under it,
/// and what stands on it.
pub(in crate::dungeon_map) fn own_stack<'a>(
    info: Option<&LevelInfo>,
    others: &'a OtherLevels,
) -> (bool, Above<'a>) {
    let above = info.map_or(Above::Nothing, |info| {
        above_floor(info, info.index, |index| {
            others.level(index).map(|level| &level.level)
        })
    });
    (has_floors_below(info), above)
}
