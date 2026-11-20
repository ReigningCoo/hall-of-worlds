//! Create Crafting Bench's ghost (dev tools): while Bench Mode is on, the bench
//! its panel has set up follows the pointer (or a finger dragging), snapped
//! to the cell under it as its north-west cell, with its tools on it, drawn
//! see-through: the bench's wood where the server would take it, red where it
//! wouldn't (a wall, stairs, other furniture, off the level).

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::furniture::free_for_bench;

use super::bench_tools::ToolModels;
use super::ground_hit::Ground;
use super::{CELL_SIZE, DungeonScene};
use crate::common::{BenchGhost, ClickMode, CurrentDungeon, DrawnGround, PointerPosition};
use crate::dom::Hit;
use crate::view3d::ViewCamera;

/// How see-through the ghost is: its colours' alpha.
const ALPHA: f32 = 0.5;

/// Moves the ghost's bench to the cell under the pointer and says whether it
/// fits there; no place while the pointer is on the interface or off the
/// level.
pub(super) fn follow_pointer(
    mode: Res<ClickMode>,
    pointer: Res<PointerPosition>,
    hit: Res<Hit>,
    camera: Single<(&Camera, &GlobalTransform), With<ViewCamera>>,
    (dungeon, heights): (Res<CurrentDungeon>, Res<DrawnGround>),
    mut ghost: ResMut<BenchGhost>,
) {
    let (camera, camera_transform) = *camera;
    let cell = pointer
        .0
        .filter(|_| *mode == ClickMode::PlaceBench && hit.target.is_none())
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| super::picking::floor_cell(ray, Ground(Some(&heights))));
    let mut next = ghost.clone();
    next.fits = None;
    if let (Some(at), Some(order)) = (cell, next.order.as_mut()) {
        order.piece.at = at;
        let off_stairs = |(x, y): (i64, i64)| !dungeon.stairs.iter().any(|s| (s.x, s.y) == (x, y));
        let free = |cell| free_for_bench(&dungeon.level, cell) && off_stairs(cell);
        next.fits = Some(order.piece.cells().all(free));
    }
    // Only a real change redraws it.
    ghost.set_if_neq(next);
}

/// The ghost as drawn, and its two looks.
#[derive(Default)]
pub(super) struct Drawn {
    entity: Option<Entity>,
    looks: Option<[Handle<StandardMaterial>; 2]>,
}

/// Draws the ghost again whenever it changes.
pub(super) fn draw_ghost(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    ghost: Res<BenchGhost>,
    heights: Res<DrawnGround>,
    mut models: ResMut<ToolModels>,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
    mut drawn: Local<Drawn>,
) {
    if !ghost.is_changed() && !heights.is_changed() {
        return;
    }
    if let Some(entity) = drawn.entity.take() {
        commands.entity(entity).despawn();
    }
    let (Some(order), Some(fits)) = (&ghost.order, ghost.fits) else {
        return;
    };
    let [wood, refused] = drawn
        .looks
        .get_or_insert_with(|| {
            let look = |red, green, blue| StandardMaterial {
                base_color: Color::srgba(red, green, blue, ALPHA),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            };
            [
                materials.add(look(0.85, 0.65, 0.4)),
                materials.add(look(0.95, 0.15, 0.1)),
            ]
        })
        .clone();
    let look = if fits { wood } else { refused };
    let piece = &order.piece;
    // Level at the highest ground under it, as a bench's top is.
    let ground = (piece.cells())
        .map(|(x, y)| heights.drawn_between(x as f32, y as f32) * CELL_SIZE)
        .fold(f32::MIN, f32::max);
    let mut root = commands.spawn((
        Transform::from_translation(Vec3::Y * ground),
        Visibility::default(),
    ));
    root.with_children(|parent| {
        for (low, high) in super::furniture::ghost_boxes(piece) {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::from_corners(low, high))),
                MeshMaterial3d(look.clone()),
                Transform::from_translation((low + high) / 2.0),
                NotShadowCaster,
            ));
        }
        let facing = piece.facing.beside((0, 0));
        let turn = Quat::from_rotation_y((facing.0 as f32).atan2(facing.1 as f32));
        for &(kind, _) in &order.tools {
            let spot = super::furniture::tool_spot(piece, kind);
            let model = models.get(kind, &mut meshes, &mut materials);
            if let (Some((spot, _)), Some(model)) = (spot, model) {
                parent.spawn((
                    Mesh3d(model.fill),
                    MeshMaterial3d(look.clone()),
                    Transform::from_translation(spot).with_rotation(turn),
                    NotShadowCaster,
                ));
            }
        }
    });
    let entity = root.id();
    commands.entity(*scene).add_child(entity);
    drawn.entity = Some(entity);
}
