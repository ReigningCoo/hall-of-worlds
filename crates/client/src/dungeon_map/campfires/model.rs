//! A campfire's model, from simple shapes: six flattened stones in a ring,
//! three sticks crossed over the middle, and up to three flames (cones) as
//! tall as the kindling left. Each part has a black outline: the same mesh a
//! little larger, drawn from inside (as bags' are).

use std::f32::consts::TAU;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use shared::campfire::MAX_KINDLING;

use super::Flame;
use crate::dungeon_map::CELL_SIZE;

/// How far the ring of stones reaches from the middle, in cells.
const RING: f32 = 0.3;

/// How much larger a part's outline is than the part.
const OUTLINE_GROW: f32 = 1.12;

/// Where the flames stand, over the sticks, in cells.
const FLAME_BASE: f32 = 0.06;

/// How large a fire in a hearth's firebox is drawn, against a campfire.
const HEARTH_SCALE: f32 = 0.55;

/// What every fire is built from.
#[derive(Clone)]
pub(in crate::dungeon_map) struct FireParts {
    stone: Handle<Mesh>,
    stick: Handle<Mesh>,
    flame: Handle<Mesh>,
    stone_colour: Handle<StandardMaterial>,
    wood: Handle<StandardMaterial>,
    fire: Handle<StandardMaterial>,
    embers: Handle<StandardMaterial>,
    outline: Handle<StandardMaterial>,
}

impl FireParts {
    pub(in crate::dungeon_map) fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let unlit = |colour: Color| StandardMaterial {
            base_color: colour,
            unlit: true,
            ..default()
        };
        let rough = |colour: Color| StandardMaterial {
            base_color: colour,
            perceptual_roughness: 0.95,
            ..default()
        };
        Self {
            stone: meshes.add(Sphere::new(0.07 * CELL_SIZE)),
            stick: meshes.add(Cylinder::new(0.035 * CELL_SIZE, 0.5 * CELL_SIZE)),
            // A unit tall, scaled to each flame's height (`Flame`).
            flame: meshes.add(Cone::new(0.09 * CELL_SIZE, 1.0)),
            stone_colour: materials.add(rough(Color::srgb(0.48, 0.46, 0.44))),
            wood: materials.add(rough(Color::srgb(0.35, 0.22, 0.12))),
            fire: materials.add(unlit(Color::srgb(1.0, 0.6, 0.15))),
            embers: materials.add(unlit(Color::srgb(0.75, 0.2, 0.05))),
            outline: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                unlit: true,
                cull_mode: Some(Face::Front),
                ..default()
            }),
        }
    }
}

/// Spawns one part and its outline under `parent`: the part's entity.
fn part(
    parent: &mut ChildSpawnerCommands,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    outline: &Handle<StandardMaterial>,
    transform: Transform,
) -> Entity {
    parent
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            transform,
            NotShadowCaster,
        ))
        .with_child((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(outline.clone()),
            Transform::from_scale(Vec3::splat(OUTLINE_GROW)),
            NotShadowCaster,
        ))
        .id()
}

/// Builds the fire at `cell` with `pieces` of kindling left under `fire`;
/// in a hearth's firebox (`in_hearth`), without the ring of stones and
/// smaller, to fit under its slab.
pub(in crate::dungeon_map) fn spawn_fire(
    commands: &mut Commands,
    fire: Entity,
    parts: &FireParts,
    (cell, pieces): ((i64, i64), u8),
    in_hearth: bool,
) {
    let small = if in_hearth { HEARTH_SCALE } else { 1.0 };
    // Turned by its cell, so fires side by side differ.
    let turn = (cell.0 * 7 + cell.1 * 13).rem_euclid(12) as f32 / 12.0 * TAU;
    commands.entity(fire).with_children(|parent| {
        for stone in 0..if in_hearth { 0 } else { 6 } {
            let angle = turn + stone as f32 / 6.0 * TAU;
            let at = Vec3::new(angle.cos(), 0.0, angle.sin()) * RING * CELL_SIZE;
            let transform = Transform::from_translation(at.with_y(0.03 * CELL_SIZE))
                .with_scale(Vec3::new(1.2, 0.6, 1.0))
                .with_rotation(Quat::from_rotation_y(-angle));
            part(
                parent,
                &parts.stone,
                &parts.stone_colour,
                &parts.outline,
                transform,
            );
        }
        let sticks = if pieces == 0 {
            &parts.embers
        } else {
            &parts.wood
        };
        for stick in 0..3 {
            let angle = turn + stick as f32 / 3.0 * TAU / 2.0;
            let lean = Quat::from_rotation_y(angle) * Quat::from_rotation_z(TAU / 4.0 - 0.25);
            let transform = Transform::from_xyz(0.0, 0.05 * CELL_SIZE, 0.0)
                .with_rotation(lean)
                .with_scale(Vec3::splat(small));
            part(parent, &parts.stick, sticks, &parts.outline, transform);
        }
        if pieces == 0 {
            return;
        }
        // Taller the more is left: under half a cell for one piece, a cell
        // for a full fire; one flame, and two more round it from three pieces.
        let full = f32::from(pieces).min(MAX_KINDLING as f32) / MAX_KINDLING as f32;
        let height = (0.4 + 0.6 * full) * CELL_SIZE * small;
        let flames: &[(f32, f32, f32)] = match pieces {
            1 | 2 => &[(0.0, 0.0, 1.0)],
            _ => &[(0.0, 0.0, 1.0), (0.08, 0.0, 0.7), (-0.06, 0.07, 0.6)],
        };
        let base = FLAME_BASE * CELL_SIZE;
        for (index, &(dx, dz, tall)) in flames.iter().enumerate() {
            let tall = height * tall;
            let transform = Transform::from_xyz(dx * CELL_SIZE, base + tall / 2.0, dz * CELL_SIZE)
                .with_scale(Vec3::new(1.0, tall, 1.0));
            let id = part(parent, &parts.flame, &parts.fire, &parts.outline, transform);
            parent.commands().entity(id).insert(Flame {
                height: tall,
                base,
                phase: index as f32 * 2.1,
            });
        }
    });
}
