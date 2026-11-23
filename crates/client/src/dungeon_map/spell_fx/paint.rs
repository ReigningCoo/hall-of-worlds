//! `Paint`: what every system drawing spells needs, in one system parameter:
//! spawning pieces that fade (`Fx`), sparkles that drift, and a little
//! randomness for how they scatter (looks only, so never the game's own).

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::super::parts::DungeonScene;
use super::fade::{Fx, Grow};
use super::kit::{FxKit, Meshes, glow_material};
use super::sparkles::Drift;

/// How long a sparkle lasts, in seconds.
const SPARKLE_LIFE: f32 = 0.6;

#[derive(SystemParam)]
pub(crate) struct Paint<'w, 's> {
    pub(super) commands: Commands<'w, 's>,
    kit: ResMut<'w, FxKit>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    time: Res<'w, Time>,
    seed: Local<'s, u32>,
}

impl Paint<'_, '_> {
    /// Now, in `Time`'s seconds.
    pub(super) fn now(&self) -> f64 {
        self.time.elapsed_secs_f64()
    }

    /// The shared meshes.
    pub(super) fn shared(&mut self) -> Meshes {
        self.kit.meshes(&mut self.meshes)
    }

    /// A mesh of its own (an arc, a ring).
    pub(super) fn add_mesh(&mut self, mesh: impl Into<Mesh>) -> Handle<Mesh> {
        self.meshes.add(mesh)
    }

    /// A number from 0 to 1, different each time.
    pub(super) fn random(&mut self) -> f32 {
        *self.seed = self
            .seed
            .wrapping_mul(747_796_405)
            .wrapping_add(2_891_336_453);
        let mixed = (*self.seed ^ (*self.seed >> 15)).wrapping_mul(277_803_737);
        (mixed >> 8) as f32 / (1u32 << 24) as f32
    }

    /// A way at random, of length 1.
    pub(super) fn random_way(&mut self) -> Vec3 {
        let (a, b) = (self.random(), self.random());
        let (around, up) = (a * std::f32::consts::TAU, b * 2.0 - 1.0);
        let flat = (1.0 - up * up).sqrt();
        Vec3::new(flat * around.cos(), up, flat * around.sin())
    }

    /// A piece of `mesh` placed by `transform` (its scale the full size),
    /// fading in its own material of `colour` over `life` seconds.
    pub(super) fn fading(
        &mut self,
        mesh: Handle<Mesh>,
        transform: Transform,
        colour: LinearRgba,
        (life, grow): (f32, Grow),
    ) {
        let material = self.materials.add(glow_material(colour));
        let fx = Fx {
            born: self.now(),
            life,
            size: transform.scale,
            grow,
            fades: Some(colour),
        };
        self.commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            transform,
            fx,
            DungeonScene,
        ));
    }

    /// A piece of `mesh` in a shared glow of `colour`, kept until its caller
    /// takes it away (a missile's bolt).
    pub(super) fn lasting(
        &mut self,
        mesh: Handle<Mesh>,
        colour: LinearRgba,
        transform: Transform,
    ) -> Entity {
        let material = self.kit.glow(&mut self.materials, colour);
        self.commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                transform,
                DungeonScene,
            ))
            .id()
    }

    /// One sparkle at `at` of `size` (on screen), drifting at `velocity` and
    /// shrinking away.
    pub(super) fn sparkle(&mut self, at: Vec3, velocity: Vec3, colour: LinearRgba, size: f32) {
        let spark = self.shared().spark;
        let material = self.kit.glow(&mut self.materials, colour);
        let life = SPARKLE_LIFE * (0.6 + 0.8 * self.random());
        let fx = Fx {
            born: self.now(),
            life,
            size: Vec3::splat(size),
            grow: Grow::Shrink,
            fades: None,
        };
        let transform = Transform::from_translation(at).with_scale(Vec3::splat(size));
        self.commands.spawn((
            Mesh3d(spark),
            MeshMaterial3d(material),
            transform,
            fx,
            Drift(velocity),
            DungeonScene,
        ));
    }

    /// `count` sparkles bursting from `at` at about `speed`, lifted by `up`,
    /// in turn through `colours`.
    pub(super) fn burst(
        &mut self,
        at: Vec3,
        colours: &[LinearRgba],
        count: usize,
        (speed, up): (f32, f32),
        size: f32,
    ) {
        for n in 0..count {
            let way = self.random_way();
            let velocity = way * speed * (0.4 + self.random()) + Vec3::Y * up;
            let colour = colours[n % colours.len()];
            let size = size * (0.6 + 0.8 * self.random());
            self.sparkle(at + way * size, velocity, colour, size);
        }
    }
}
