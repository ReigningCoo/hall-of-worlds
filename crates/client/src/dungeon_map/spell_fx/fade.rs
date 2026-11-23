//! Each drawn piece of a spell lasts a moment: growing, shrinking or fading
//! as it goes, then gone.

use bevy::prelude::*;

/// A piece of a spell drawn for a while.
#[derive(Component)]
pub(crate) struct Fx {
    /// When it was drawn and how long it lasts, in `Time`'s seconds.
    pub(super) born: f64,
    pub(super) life: f32,
    /// Its full size (the transform's scale).
    pub(super) size: Vec3,
    pub(super) grow: Grow,
    /// Its own material's colour, fading to nothing; `None` for a shared
    /// material, which shrinks instead.
    pub(super) fades: Option<LinearRgba>,
}

/// How its size changes over its life.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Grow {
    /// The same all along.
    Steady,
    /// Quickly from a fifth to full in its first third.
    Spread,
    /// From full to nothing.
    Shrink,
}

/// Sizes, fades and at the end takes away every piece.
pub(crate) fn fade_fx(
    mut commands: Commands,
    time: Res<Time>,
    mut pieces: Query<(
        Entity,
        &Fx,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let now = time.elapsed_secs_f64();
    for (entity, fx, mut transform, material) in &mut pieces {
        let age = ((now - fx.born) as f32 / fx.life.max(0.01)).max(0.0);
        if age >= 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let scale = match fx.grow {
            Grow::Steady => 1.0,
            Grow::Spread => 0.2 + 0.8 * (age * 3.0).min(1.0),
            Grow::Shrink => 1.0 - age,
        };
        transform.scale = fx.size * scale;
        if let Some(colour) = fx.fades
            && let Some(mut material) = materials.get_mut(&material.0)
        {
            let left = (1.0 - age) * (1.0 - age);
            material.base_color = Color::LinearRgba(colour * left);
        }
    }
}
