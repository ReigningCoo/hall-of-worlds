//! The filters' octagonal frame (owner, 2026-11-23: wheres round, filters
//! octagons): a UI material (`assets/shaders/octagon.wesl`) drawing an
//! octagon filled in one colour with an edge in another, under the icon.

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

/// How wide the edge is, in pixels.
const EDGE_WIDTH: f32 = 2.0;

/// An octagon behind a node's children: its fill and its edge.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub(super) struct Octagon {
    pub(super) fill: Color,
    pub(super) edge: Color,
}

/// Order matches `struct Octagon` in the shader; 48 bytes, as WebGL 2 wants
/// uniforms in sixteens.
#[derive(ShaderType, Clone, Copy, Debug, Default)]
struct OctagonParams {
    fill: Vec4,
    edge: Vec4,
    edge_width: f32,
    unused_a: f32,
    unused_b: f32,
    unused_c: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct OctagonMaterial {
    #[uniform(0)]
    params: OctagonParams,
}

impl UiMaterial for OctagonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/octagon.wesl".into()
    }
}

/// Gives each new octagon its material. `try_insert`: the rows may be drawn
/// again before it lands.
pub(super) fn give_octagons(
    new: Query<(Entity, &Octagon), Added<Octagon>>,
    mut materials: ResMut<Assets<OctagonMaterial>>,
    mut commands: Commands,
) {
    for (entity, octagon) in &new {
        let colour = |colour: Color| LinearRgba::from(colour).to_vec4();
        let params = OctagonParams {
            fill: colour(octagon.fill),
            edge: colour(octagon.edge),
            edge_width: EDGE_WIDTH,
            ..default()
        };
        let material = materials.add(OctagonMaterial { params });
        commands.entity(entity).try_insert(MaterialNode(material));
    }
}
