//! Sparkles drifting: each moves on at its own speed, slowing as it goes, and
//! shrinks away (`fade_fx`).

use bevy::prelude::*;

/// A sparkle's speed, on screen a second.
#[derive(Component)]
pub(crate) struct Drift(pub(super) Vec3);

/// How much of its speed a sparkle keeps each second.
const KEPT: f32 = 0.15;

/// Moves every sparkle on.
pub(crate) fn drift_sparkles(time: Res<Time>, mut sparkles: Query<(&mut Drift, &mut Transform)>) {
    let step = time.delta_secs();
    let kept = KEPT.powf(step);
    for (mut drift, mut transform) in &mut sparkles {
        transform.translation += drift.0 * step;
        drift.0 *= kept;
    }
}
