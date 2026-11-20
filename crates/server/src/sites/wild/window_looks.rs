//! A cutout's looks in a wilderness window: towns' buildings, farms' fields and
//! forts' walls, turned into the window's frame; and the kinds of its own
//! site's trees (an oasis's), rolled as its site rolls them.

use shared::dungeon::Cell;
use shared::site::{BuildingLook, Rect};
use shared::wilderness::Wilderness;

use super::window::Window;
use crate::wilderness::WildChunk;

/// Adds a chunk's cutout's looks to the window, `turn` cells across the seam.
pub(super) fn add_looks(window: &mut Window, wild: &Wilderness, wild_chunk: &WildChunk, turn: i64) {
    let turned = |rect: Rect| Rect {
        x: rect.x + turn,
        ..rect
    };
    for building in &wild_chunk.buildings {
        window.buildings.push(BuildingLook {
            rects: building.rects.iter().copied().map(turned).collect(),
            door: (building.door.0 + turn, building.door.1),
            ..building.clone()
        });
    }
    for field in &wild_chunk.fields {
        window.fields.push(field.moved((turn, 0)));
    }
    for wall in &wild_chunk.walls {
        window.walls.push(wall.moved((turn, 0)));
    }
    add_tree_kinds(window, wild, wild_chunk, turn);
}

/// The kinds of the trees of a cutout that is its own site's (with an
/// offset): `Wilderness::site_tree_kind` by the site's own cells, as felling
/// there finds them (`sites::trees::tree_here`).
fn add_tree_kinds(window: &mut Window, wild: &Wilderness, wild_chunk: &WildChunk, turn: i64) {
    let Some(cutout) = wild_chunk.plan.cutout() else {
        return;
    };
    let Some((ox, oy)) = cutout.offset() else {
        return;
    };
    let level = cutout.level();
    let (min_x, min_y, width, height) = level.bounds();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.cell(x, y) == Cell::Tree {
                let kind = wild.site_tree_kind(cutout.poi(), x - ox, y - oy);
                window.level.level.set_tree_kind(x + turn, y, Some(kind));
            }
        }
    }
}
