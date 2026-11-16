//! Building one other floor: what is drawn of it, built as the player's own
//! floor is (`mesh::build_posed`), under a parent lowered or raised by whole
//! storeys and stretched so its walls meet the floor above.

use bevy::prelude::*;
use shared::dungeon::{Level, Sight};
use shared::protocol::LevelInfo;
use std::collections::{BTreeMap, BTreeSet};

use super::super::WALL_HEIGHT;
use super::super::mesh::{DungeonMeshes, LightGrid, Surface, View, build_posed};
use super::super::theme::Shape;
use super::floors::Floor;
use super::roof::{RoofAt, roof_mesh, roofed, roofing};
use super::which::drawn;
use crate::common::{CurrentDungeon, OtherFloorLooks};

/// How far below the floor above a storey's walls stop, so their tops don't
/// fight it for depth.
const TOP_GAP: f32 = WALL_HEIGHT * 0.01;

/// Where another floor is, from the player's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Placed {
    /// This many storeys down.
    Below(u32),
    /// This many storeys up.
    Above(u32),
}

impl Placed {
    /// How high the floor stands (negative below) and how much its walls are
    /// stretched, for a storey `storey` walls tall. The player's own walls are
    /// a wall tall, so the floor above stands on them, and each further one a
    /// storey higher.
    pub(super) fn height_and_stretch(self, storey: f32) -> (f32, f32) {
        let storey = storey * WALL_HEIGHT;
        let stretch = (storey - TOP_GAP) / WALL_HEIGHT;
        let height = match self {
            Self::Below(down) => -(down as f32) * storey,
            Self::Above(up) => WALL_HEIGHT + TOP_GAP + (up - 1) as f32 * storey,
        };
        (height, stretch)
    }
}

/// What every other floor is built from: the building, what the player sees
/// of it now, where they and the camera are, and how it looks.
pub(super) struct Building<'a> {
    pub(super) info: &'a LevelInfo,
    /// Each floor from the ground up, if the client has it.
    pub(super) known: &'a [Option<&'a Level>],
    /// Which cells of each floor the client knows (the player's own too).
    pub(super) cells: &'a [Option<&'a Sight>],
    /// The player's own floor.
    pub(super) own: &'a Level,
    /// What the player sees now of each floor.
    pub(super) now: &'a [Sight],
    pub(super) at: (i64, i64),
    pub(super) camera: Vec3,
    /// The outside light, all over.
    pub(super) light: Vec3,
    pub(super) shape: Shape,
    pub(super) looks: &'a OtherFloorLooks,
}

/// Builds floor `index` (`level`, as the client has it), placed `place`, into
/// `floor`'s meshes.
pub(super) fn build_floor(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    floor: &Floor,
    building: &Building,
    level: &CurrentDungeon,
    (index, place): (u32, Placed),
) {
    let Building { info, looks, .. } = *building;
    // Seen straight down: the floors between it and the player's, theirs
    // included. Nothing is seen straight up.
    let between: Vec<&Level> = match place {
        Placed::Below(_) => (index + 1..info.index)
            .filter_map(|i| building.known[i as usize])
            .chain([building.own])
            .collect(),
        Placed::Above(_) => Vec::new(),
    };
    let range = (index == 0).then_some(looks.ground_range);
    let now = &building.now[index as usize];
    let draw = drawn(
        &level.level,
        &level.remembered,
        now,
        &between,
        (building.at, range),
        looks.remembered,
    );
    let (height, stretch) = place.height_and_stretch(looks.storey);
    let camera = building.camera;
    let local = Vec3::new(camera.x, (camera.y - height) / stretch, camera.z);
    let view = View {
        cell: building.at,
        sight: now.clone(),
    };
    let light = LightGrid::filled(&level.level, building.light);
    let empty = Sight::new(&level.level);
    let mut made = build_posed(
        &level.level,
        (&[view], now, &draw, &empty),
        &light,
        &level.stairs,
        (&[], &BTreeSet::new(), &[], &level.walls, info.ruined),
        // Never cut down near the player: other floors aren't in the way
        // (the wedge leaves out the floors above that would be).
        (local, building.shape, false, false),
        None,
        &BTreeMap::new(),
    );
    let roof = match place {
        Placed::Below(_) => between.first().map(|over| {
            let known = (&level.level, &level.remembered);
            (roofed(known, over, &draw), RoofAt::Top)
        }),
        Placed::Above(up) => {
            let under = index as usize - 1;
            let level_under = if up == 1 {
                Some(building.own)
            } else {
                building.known[under]
            };
            level_under
                .zip(building.cells[under])
                .map(|known| (roofing(&level.level, known), RoofAt::Foot))
        }
    };
    if let Some((cells, at)) = roof {
        add_roof(&mut made, &cells, at, building.light);
    }
    commands.entity(floor.root).insert(
        Transform::from_translation(Vec3::Y * height).with_scale(Vec3::new(1.0, stretch, 1.0)),
    );
    put(commands, meshes, floor, made);
}

/// Adds a roof over `cells` to a floor's walls (their tops look the same).
fn add_roof(made: &mut DungeonMeshes, cells: &[(i64, i64)], at: RoofAt, light: Vec3) {
    if cells.is_empty() {
        return;
    }
    let wall = Surface::Wall as usize;
    let roof = roof_mesh(cells, at, light);
    if made.has_solid[wall] {
        if made.solid[wall].merge(&roof).is_err() {
            return;
        }
    } else {
        made.solid[wall] = roof;
    }
    made.has_solid[wall] = true;
}

/// Puts a floor's built meshes in place, showing those with faces.
fn put(commands: &mut Commands, meshes: &mut Assets<Mesh>, floor: &Floor, made: DungeonMeshes) {
    let solid = made
        .solid
        .into_iter()
        .zip(&made.has_solid)
        .zip(&floor.solid);
    let memory = made
        .memory
        .into_iter()
        .zip(&made.has_memory)
        .zip(&floor.memory);
    for ((mesh, &has), (entity, handle, _)) in solid.chain(memory) {
        commands.entity(*entity).insert(if has {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
        // An empty mesh is left as it was, hidden: Bevy warns about meshes
        // with no vertices.
        if has && let Some(mut stored) = meshes.get_mut(handle) {
            *stored = mesh;
        }
    }
}

#[cfg(test)]
mod tests;
