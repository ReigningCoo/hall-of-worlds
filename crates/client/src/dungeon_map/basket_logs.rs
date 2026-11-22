//! Kindling in fireplaces' log baskets (tavern furniture plan PR 4): each
//! basket place the server last told holds kindling (`common::Benches`,
//! `Held::Fire`) drawn as up to three logs of its wood's colour, lying front
//! to back in its place (`furniture::basket_spot`), with black outlines.
//! Drawn once the fireplace has been seen, as hearths' fires are.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use shared::furniture::Held;

use super::{CELL_SIZE, DungeonScene};
use crate::common::{Benches, CurrentDungeon, DrawnGround};

/// The most logs drawn in one place, however many pieces it holds.
const MOST_LOGS: u32 = 3;
/// How much larger a log's outline is than the log.
const OUTLINE_GROW: f32 = 1.12;
/// A log's radius and length, in cells.
const RADIUS: f32 = 0.045;
const LENGTH: f32 = 0.2;
/// Kindling of no tree's wood: plain brown.
const PLAIN: [u8; 3] = [140, 95, 60];

/// What one place shows, to tell when it must be drawn again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Drawn {
    logs: u32,
    colour: [u8; 3],
    /// Where it is, in tenths of a unit.
    at: IVec3,
}

/// The places drawn now, by fireplace and place, and what logs share.
#[derive(Resource, Default)]
pub(super) struct DrawnBasketLogs {
    places: HashMap<((i64, i64), usize), (Entity, Drawn)>,
    log: Option<(Handle<Mesh>, Handle<StandardMaterial>)>,
    woods: HashMap<[u8; 3], Handle<StandardMaterial>>,
}

/// Draws the baskets' kindling again when the benches, the level or the
/// ground change.
pub(super) fn update_basket_logs(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, heights, benches): (Res<CurrentDungeon>, Res<DrawnGround>, Res<Benches>),
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
    mut drawn: ResMut<DrawnBasketLogs>,
) {
    if !dungeon.is_changed() && !heights.is_changed() && !benches.is_changed() {
        return;
    }
    let (mesh, outline) = (drawn.log)
        .get_or_insert_with(|| {
            let mesh = meshes.add(Cylinder::new(RADIUS * CELL_SIZE, LENGTH * CELL_SIZE));
            let outline = materials.add(StandardMaterial {
                base_color: Color::BLACK,
                unlit: true,
                cull_mode: Some(Face::Front),
                ..default()
            });
            (mesh, outline)
        })
        .clone();
    let mut keep = HashMap::new();
    for (&at, held) in &benches.0 {
        let Held::Fire(fireside) = held else {
            continue;
        };
        let level = &dungeon.level;
        let Some(piece) = level.furniture_at(at.0, at.1).filter(|p| p.at == at) else {
            continue;
        };
        if !piece.cells().any(|(x, y)| dungeon.remembered.is_seen(x, y)) {
            continue;
        }
        // Level at the highest ground under it, as furniture's tops are.
        let ground = (piece.cells())
            .map(|(x, y)| heights.drawn_between(x as f32, y as f32) * CELL_SIZE)
            .fold(f32::MIN, f32::max);
        for (index, kindling) in fireside.places().iter().enumerate() {
            let Some(kindling) = kindling else {
                continue;
            };
            let Some((spot, way)) = super::furniture::basket_spot(piece, index) else {
                continue;
            };
            let colour = (kindling.kindling())
                .and_then(|form| form.wood.wood())
                .map_or(PLAIN, |wood| wood.heartwood);
            let place = spot + Vec3::Y * ground;
            let now = Drawn {
                logs: kindling.count.clamp(1, MOST_LOGS),
                colour,
                at: (place * 10.0).round().as_ivec3(),
            };
            let key = (at, index);
            if let Some((entity, was)) = drawn.places.remove(&key) {
                if was == now {
                    keep.insert(key, (entity, was));
                    continue;
                }
                commands.entity(entity).despawn();
            }
            let wood = (drawn.woods.entry(colour)).or_insert_with(|| {
                let [r, g, b] = colour;
                materials.add(StandardMaterial {
                    base_color: Color::srgb_u8(r, g, b),
                    perceptual_roughness: 0.95,
                    ..default()
                })
            });
            let entity = spawn_logs(&mut commands, place, way, now.logs, (&mesh, wood, &outline));
            commands.entity(*scene).add_child(entity);
            keep.insert(key, (entity, now));
        }
    }
    for (_, (entity, _)) in drawn.places.drain() {
        commands.entity(entity).despawn();
    }
    drawn.places = keep;
}

/// `logs` logs at `place` lying along `way`: two side by side, a third on
/// top.
fn spawn_logs(
    commands: &mut Commands,
    place: Vec3,
    way: Vec3,
    logs: u32,
    (mesh, wood, outline): (
        &Handle<Mesh>,
        &Handle<StandardMaterial>,
        &Handle<StandardMaterial>,
    ),
) -> Entity {
    let lying = Quat::from_rotation_arc(Vec3::Y, way.normalize_or(Vec3::X));
    let side = way.cross(Vec3::Y).normalize_or(Vec3::Z) * RADIUS * CELL_SIZE;
    let spots = [-side, side, Vec3::Y * RADIUS * CELL_SIZE * 1.7];
    let parent = commands
        .spawn((Transform::from_translation(place), Visibility::default()))
        .id();
    commands.entity(parent).with_children(|parent| {
        for &offset in spots.iter().take(logs as usize) {
            let transform = Transform::from_translation(offset).with_rotation(lying);
            parent
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(wood.clone()),
                    transform,
                    NotShadowCaster,
                ))
                .with_child((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(outline.clone()),
                    Transform::from_scale(Vec3::splat(OUTLINE_GROW)),
                    NotShadowCaster,
                ));
        }
    });
    parent
}
