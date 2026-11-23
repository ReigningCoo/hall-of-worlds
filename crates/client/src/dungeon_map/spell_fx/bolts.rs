//! Missiles in flight as sparkling bolts: a bright core in the first colour
//! of what they carry, a streak behind it, and a trail of sparkles in all its
//! colours drifting off as it flies.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::protocol::MissileId;

use super::super::CELL_SIZE;
use super::colours::look_colours;
use super::news::FxQueue;
use super::paint::Paint;
use super::place::{CHEST, spot_drawn, way};
use crate::common::{DrawnGround, Missiles};

/// A bolt's core's radius, and its streak's length, in cells.
const CORE: f32 = 0.13;
const STREAK: f32 = 0.6;

/// Sparkles left behind a cell flown, and how big.
const SPARKS_PER_CELL: f32 = 5.0;
const SPARK: f32 = 0.035;

/// The most sparkles left in one frame.
const MOST_SPARKS: usize = 40;

/// One bolt drawn.
struct Bolt {
    core: Entity,
    streak: Entity,
    /// Where its last sparkle was left.
    left_at: Vec3,
}

/// The bolts drawn, by missile.
#[derive(Resource, Default)]
pub(crate) struct DrawnBolts {
    bolts: HashMap<MissileId, Bolt>,
}

/// Puts a bolt where each missile is now, leaving sparkles behind, spawning
/// and taking them away as missiles come and go.
pub(crate) fn draw_bolts(
    mut paint: Paint,
    missiles: Res<Missiles>,
    ground: Res<DrawnGround>,
    queue: Res<FxQueue>,
    mut drawn: ResMut<DrawnBolts>,
    mut places: Query<&mut Transform>,
) {
    if missiles.is_empty() && drawn.bolts.is_empty() {
        return;
    }
    let now = paint.now();
    let mut keep = HashMap::with_capacity(drawn.bolts.len());
    for (id, at, path) in missiles.at(now) {
        let colours = look_colours(&queue.looks.get(&id).copied().unwrap_or_default());
        let place = spot_drawn(&ground, at, CHEST);
        let ahead = way(path.heading);
        let turned = Quat::from_rotation_arc(Vec3::Z, ahead);
        let mut bolt = drawn.bolts.remove(&id).unwrap_or_else(|| {
            let ball = paint.shared().ball;
            let core = paint.lasting(ball.clone(), colours[0] * 1.6, Transform::default());
            let tail = colours.get(1).copied().unwrap_or(colours[0]) * 0.8;
            let streak = paint.lasting(ball, tail, Transform::default());
            Bolt {
                core,
                streak,
                left_at: spot_drawn(&ground, path.from, CHEST),
            }
        });
        if let Ok(mut transform) = places.get_mut(bolt.core) {
            *transform =
                Transform::from_translation(place).with_scale(Vec3::splat(CORE * CELL_SIZE));
        }
        if let Ok(mut transform) = places.get_mut(bolt.streak) {
            let behind = place - ahead * STREAK * 0.5 * CELL_SIZE;
            let size = Vec3::new(CORE * 0.6, CORE * 0.6, STREAK * 0.5) * CELL_SIZE;
            *transform = Transform::from_translation(behind)
                .with_rotation(turned)
                .with_scale(size);
        }
        // Spread along the stretch flown since the last ones, so the trail
        // stays whole however few frames there are.
        let from = bolt.left_at;
        let due = (from.distance(place) / CELL_SIZE * SPARKS_PER_CELL) as usize;
        let due = due.min(MOST_SPARKS);
        for n in 0..due {
            let along = (n + 1) as f32 / due as f32;
            let drift = paint.random_way() * 0.4 * CELL_SIZE - ahead * 0.5 * CELL_SIZE;
            let jitter = paint.random_way() * CORE * CELL_SIZE;
            let colour = colours[n % colours.len()];
            let at = from.lerp(place, along) + jitter;
            paint.sparkle(at, drift, colour, SPARK * CELL_SIZE);
        }
        if due > 0 {
            bolt.left_at = place;
        }
        keep.insert(id, bolt);
    }
    for (_, gone) in drawn.bolts.drain() {
        paint.commands.entity(gone.core).despawn();
        paint.commands.entity(gone.streak).despawn();
    }
    drawn.bolts = keep;
}
