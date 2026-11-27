// A dungeon surface: the standard material, shown only on the side of the sight
// mask it belongs to, so the edge of sight falls part way across a cell. See
// docs/context/client/dungeon_map.md.

import bevy_pbr::render::{
    forward_io::{FragmentOutput, VertexOutput},
    mesh_view_bindings::globals,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, main_pass_post_lighting_processing},
};

struct Mask {
    origin: vec2<f32>,
    per_unit: vec2<f32>,
    // 1 only in sight, -1 only out of sight.
    side: f32,
    // 1 when water flows (0: still), by Bevy's clock (`globals.time`, wrapping
    // every hour: a whole number of `PERIOD`s, so it never jumps).
    flowing: f32,
    // The wedge hiding what is above the player towards the camera (`Wedge`):
    // the cosine of half its angle, 1 when on, its point and which way it
    // opens. 48 bytes in all: WebGL 2 needs a multiple of 16.
    wedge_cos: f32,
    wedge_on: f32,
    wedge_at: vec2<f32>,
    wedge_towards: vec2<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> mask: Mask;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var mask_picture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var mask_sampler: sampler;

// How far in front of an upright surface the mask is read, in world units (a
// tenth of a cell): a wall's side stands right on the edge of what is in sight.
const IN_FRONT: f32 = 2.4;

// Tops, facing up over the floor: sight stops at the wall below them, so they go
// by the cell, as they always did. Match `RAISED` in mesh.rs. Measured from the
// ground under them, which raised wilderness cells give as `uv_b.x`.
const RAISED: f32 = 1.0;
const RAISED_NORMAL: f32 = 0.5;

// Round the wedge's point, everything is in it (world units, a fifth of a
// cell), so what stands right over the player is hidden whichever way it opens.
const WEDGE_NEAR: f32 = 4.8;

// How fast water flows in each band (`FLOW_BANDS` in mesh/flow.rs), in pictures
// (cells) a second: each a whole number of pictures in the 8 s `PERIOD` the
// clock is wrapped at, so it never jumps.
const SLOWEST_FLOW: f32 = 0.125;
const PERIOD: f32 = 8.0;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    // Sampled before anything can return, as derivatives need every fragment.
    let normal = normalize(in.world_normal);
    let at = in.world_position.xz + normal.xz * IN_FRONT;
    let place = (at - mask.origin) * mask.per_unit;
    let in_sight = textureSample(mask_picture, mask_sampler, place).r > 0.5;
    var ground = 0.0;
    // Water flowing down raised ground: `band × 512 + degrees` (0: still).
    var flow = 0.0;
    @if(VERTEX_UVS_B) {
        ground = in.uv_b.x;
        flow = in.uv_b.y;
    }
    let raised = in.world_position.y - ground > RAISED && normal.y > RAISED_NORMAL;
    if !raised && ((mask.side > 0.0 && !in_sight) || (mask.side < 0.0 && in_sight)) {
        discard;
    }
    if mask.wedge_on > 0.5 {
        let offset = in.world_position.xz - mask.wedge_at;
        let along = dot(offset, mask.wedge_towards);
        if along >= mask.wedge_cos * length(offset) - WEDGE_NEAR {
            discard;
        }
    }

    var input = in;
    if mask.flowing > 0.5 && flow > 0.5 {
        let seconds = globals.time % PERIOD;
        let band = floor(flow / 512.0);
        let angle = radians(flow - band * 512.0);
        let speed = SLOWEST_FLOW * exp2(band - 1.0);
        // The picture moves along the flow: east is `u`, south `v`.
        let moved = vec2<f32>(cos(angle), sin(angle)) * speed * seconds;
        input.uv = fract(in.uv - moved);
    }
    var pbr_input = pbr_input_from_standard_material(input, is_front);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color,
    );
    var out: FragmentOutput;
    // Unlit: the light is baked into the vertex colours, so real lights would
    // only add a shine.
    out.color = pbr_input.material.base_color;
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
