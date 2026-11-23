//! Missiles in flight (magic plan PR 6): `common::Missiles` kept from the
//! server's `MissileNews`, and each drawn as a small glowing ball following
//! its way, a little above the ground. A stand-in: PR 7 draws them as
//! sparkling bolts coloured by their effects.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::protocol::{MissileId, MissileNews, ServerMessage};
use shared::spot::CELL_SPAN;

use super::CELL_SIZE;
use super::parts::DungeonScene;
use crate::common::{DrawnGround, Missiles};
use crate::connection::FromServer;

/// How high a missile flies over the ground, and how big it is, in cells.
const FLIES_AT: f32 = 0.6;
const RADIUS: f32 = 0.12;

/// The balls drawn, by missile, and the mesh and material they share.
#[derive(Resource, Default)]
pub(super) struct DrawnMissiles {
    balls: HashMap<MissileId, Entity>,
    looks: Option<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

/// Keeps `Missiles` as the server tells: launched, landed, and forgotten
/// when the player goes to another map.
pub(super) fn take_missiles(
    mut from_server: MessageReader<FromServer>,
    mut missiles: ResMut<Missiles>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::Missile(MissileNews::Flying { id, path }) => {
                missiles.launch(*id, *path, now);
            }
            ServerMessage::Missile(MissileNews::Landed { id, .. }) => missiles.land(*id),
            ServerMessage::Welcome { .. }
            | ServerMessage::EnterLevel { .. }
            | ServerMessage::EnterHall { .. }
            | ServerMessage::OnWorldMap { .. }
            | ServerMessage::Teleported { .. } => missiles.clear(),
            _ => {}
        }
    }
    if !missiles.is_empty() {
        missiles.forget_old(now);
    }
}

/// Puts a ball where each missile is now, spawning and taking them away as
/// missiles come and go.
pub(super) fn draw_missiles(
    mut commands: Commands,
    missiles: Res<Missiles>,
    heights: Res<DrawnGround>,
    time: Res<Time>,
    mut drawn: ResMut<DrawnMissiles>,
    mut balls: Query<&mut Transform>,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
) {
    if missiles.is_empty() && drawn.balls.is_empty() {
        return;
    }
    let (mesh, material) = (drawn.looks)
        .get_or_insert_with(|| {
            let mesh = meshes.add(Sphere::new(RADIUS * CELL_SIZE));
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.85, 0.95, 1.0),
                emissive: LinearRgba::rgb(3.0, 4.0, 6.0),
                unlit: true,
                ..default()
            });
            (mesh, material)
        })
        .clone();
    let mut keep = HashMap::with_capacity(drawn.balls.len());
    for (id, at, _) in missiles.at(time.elapsed_secs_f64()) {
        let span = CELL_SPAN as f32;
        let (x, y) = (at.x as f32 / span, at.y as f32 / span);
        let ground = heights.drawn_between(x, y);
        let place = Vec3::new(x, ground + FLIES_AT, y) * CELL_SIZE;
        let ball = match drawn.balls.remove(&id) {
            Some(ball) => ball,
            None => commands
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(place),
                    DungeonScene,
                ))
                .id(),
        };
        if let Ok(mut transform) = balls.get_mut(ball) {
            transform.translation = place;
        }
        keep.insert(id, ball);
    }
    for (_, gone) in drawn.balls.drain() {
        commands.entity(gone).despawn();
    }
    drawn.balls = keep;
}
