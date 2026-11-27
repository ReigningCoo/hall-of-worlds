//! The gates' water material: the standard one with its picture sliding down
//! by the shader's own clock (`assets/shaders/gate_water.wesl`, Bevy's globals),
//! so the material changes only when the sliding starts or stops. Changing a
//! material every frame made the web build make new buffers every frame.

use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

/// How fast the water runs down, in pictures a second: a whole number of
/// pictures in the hour Bevy's clock wraps at, so it never jumps.
const FLOW: f32 = 0.6;

/// A gate's water.
pub type GateWaterMaterial = ExtendedMaterial<StandardMaterial, Sliding>;

/// Order matches `struct Sliding` in the shader; a whole `vec4`, as WebGL 2
/// wants uniforms in multiples of 16 bytes.
#[derive(ShaderType, Clone, Copy, Debug, Default, PartialEq)]
struct SlideParams {
    /// `x`: pictures a second (0 still); the rest unused.
    params: Vec4,
}

/// The picture sliding down, or still.
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct Sliding {
    #[uniform(100)]
    params: SlideParams,
}

impl Sliding {
    pub(super) fn running(&self) -> bool {
        self.params.params.x != 0.0
    }

    pub(super) fn set_running(&mut self, running: bool) {
        self.params.params.x = if running { FLOW } else { 0.0 };
    }
}

impl MaterialExtension for Sliding {
    fn fragment_shader() -> ShaderRef {
        "shaders/gate_water.wesl".into()
    }
}
