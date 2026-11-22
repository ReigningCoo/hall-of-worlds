use bevy::mesh::VertexAttributeValues;

use super::*;
use crate::item_art::{CAMERA_AT, CAMERA_UP, OUTLINE_PIXELS, VIEW_WIDTH};

/// Every kind drawn as a tool (`look_of`).
const TOOLS: [ItemKind; 13] = [
    ItemKind::Cauldron,
    ItemKind::MortarAndPestle,
    ItemKind::Retort,
    ItemKind::Alembic,
    ItemKind::Crucible,
    ItemKind::Filter,
    ItemKind::Pelican,
    ItemKind::OilPress,
    ItemKind::Tap,
    ItemKind::Pan,
    ItemKind::Bowl,
    ItemKind::Sieve,
    ItemKind::Lantern,
];

/// A picture's pixels a cell, as the armour pieces' tests measure their edge by.
const PIXELS_PER_CELL: f32 = 64.0;

/// How far past its picture a tool reaches, across and down, leaving room for its
/// edge: at most 0 each way when it fits.
fn overreach(kind: ItemKind) -> (f32, f32) {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let parts = tool_parts(kind, &mut meshes, &mut materials);
    assert!(!parts.is_empty(), "{kind:?}");
    // Seen as the pictures' camera sees it.
    let view = Transform::from_translation(CAMERA_AT).looking_at(Vec3::ZERO, CAMERA_UP);
    let (wide, long) = cells(kind);
    let (wide, long) = (wide as f32, long as f32);
    let per_cell = VIEW_WIDTH / wide.max(long);
    let edge = OUTLINE_PIXELS / PIXELS_PER_CELL * per_cell;
    let (mut across, mut down) = (f32::MIN, f32::MIN);
    for part in &parts {
        let mesh = meshes.get(&part.mesh).expect("its mesh");
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("{kind:?}: a mesh without positions");
        };
        for &point in points {
            let at = part.transform.transform_point(Vec3::from(point));
            across = across.max(at.dot(*view.right()).abs() + edge - wide * per_cell / 2.0);
            down = down.max(at.dot(*view.up()).abs() + edge - long * per_cell / 2.0);
        }
    }
    (across, down)
}

#[test]
fn every_tool_stays_inside_its_picture_with_room_for_its_edge() {
    let outside: Vec<String> = TOOLS
        .into_iter()
        .map(|kind| (kind, overreach(kind)))
        .filter(|(_, (across, down))| *across > 0.001 || *down > 0.001)
        .map(|(kind, (across, down))| format!("{kind:?}: {across:.3} across, {down:.3} down"))
        .collect();
    assert!(outside.is_empty(), "past their pictures: {outside:?}");
}
