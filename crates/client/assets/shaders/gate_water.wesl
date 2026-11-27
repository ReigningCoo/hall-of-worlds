// A World Gate's water: the standard material with its picture sliding down,
// by Bevy's clock (`globals.time`), so the material needn't change every frame.
// See `hall_view/water.rs`.

import bevy_pbr::render::{
    forward_io::{FragmentOutput, VertexOutput},
    mesh_view_bindings::globals,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, main_pass_post_lighting_processing},
};

struct Sliding {
    // x: pictures a second (0: still); the rest unused.
    params: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> sliding: Sliding;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var input = in;
    input.uv = in.uv - vec2<f32>(0.0, fract(globals.time * sliding.params.x));
    var pbr_input = pbr_input_from_standard_material(input, is_front);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color,
    );
    var out: FragmentOutput;
    // Unlit, as the standard material it extends.
    out.color = pbr_input.material.base_color;
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
