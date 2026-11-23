//! What spells are drawn with: the meshes every effect shares, glowing
//! materials kept by colour for sparkles and bolts (which shrink rather than
//! fade), and fresh ones for shapes that fade on their own.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::render_resource::Face;

/// The shared meshes, made the first time a spell is drawn.
#[derive(Clone)]
pub(super) struct Meshes {
    /// A ball of radius 1: bolts, flashes, domes.
    pub ball: Handle<Mesh>,
    /// A rough ball of radius 1: sparkles.
    pub spark: Handle<Mesh>,
    /// A rod of radius 1 and length 1 along `y`: beams.
    pub rod: Handle<Mesh>,
}

/// The meshes, and glowing materials by colour.
#[derive(Resource, Default)]
pub(crate) struct FxKit {
    meshes: Option<Meshes>,
    glows: HashMap<[u32; 3], Handle<StandardMaterial>>,
}

impl FxKit {
    /// The shared meshes.
    pub(super) fn meshes(&mut self, meshes: &mut Assets<Mesh>) -> Meshes {
        self.meshes
            .get_or_insert_with(|| Meshes {
                ball: meshes.add(Sphere::new(1.0).mesh().uv(16, 10)),
                spark: meshes.add(Sphere::new(1.0).mesh().ico(0).expect("a small ico sphere")),
                rod: meshes.add(Cylinder::new(1.0, 1.0)),
            })
            .clone()
    }

    /// A glowing material of `colour` that never fades, shared.
    pub(super) fn glow(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        colour: LinearRgba,
    ) -> Handle<StandardMaterial> {
        let key = [colour.red, colour.green, colour.blue].map(f32::to_bits);
        self.glows
            .entry(key)
            .or_insert_with(|| materials.add(glow_material(colour)))
            .clone()
    }
}

/// A glowing material of `colour`, added onto what is behind it and seen from
/// both sides: its own, for one shape to fade.
pub(super) fn glow_material(colour: LinearRgba) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::LinearRgba(colour),
        unlit: true,
        alpha_mode: AlphaMode::Add,
        cull_mode: None::<Face>,
        double_sided: true,
        ..default()
    }
}
