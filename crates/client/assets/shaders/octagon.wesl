// A regular octagon filling its node, flat sides along the edges: `fill`
// inside, `edge` in a band `edge_width` pixels wide round it, soft by a pixel.
// The spell design panel's filters (`spell_design/octagon.rs`).

import bevy_ui_render::ui_vertex_output::UiVertexOutput;

struct Octagon {
    fill: vec4<f32>,
    edge: vec4<f32>,
    edge_width: f32,
    unused_a: f32,
    unused_b: f32,
    unused_c: f32,
}

@group(1) @binding(0) var<uniform> octagon: Octagon;

// Signed distance from a regular octagon of inner radius `r` round the origin.
fn octagon_distance(point: vec2<f32>, r: f32) -> f32 {
    let k = vec3(-0.9238795, 0.38268343, 0.41421357);
    var q = abs(point);
    q -= 2.0 * min(dot(vec2(k.x, k.y), q), 0.0) * vec2(k.x, k.y);
    q -= 2.0 * min(dot(vec2(-k.x, k.y), q), 0.0) * vec2(-k.x, k.y);
    q -= vec2(clamp(q.x, -k.z * r, k.z * r), r);
    return length(q) * sign(q.y);
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let half = min(in.size.x, in.size.y) / 2.0;
    let from_middle = in.uv * in.size - in.size / 2.0;
    let distance = octagon_distance(from_middle, half - 0.5);
    let cover = clamp(0.5 - distance, 0.0, 1.0);
    if cover <= 0.0 {
        discard;
    }
    let on_edge = clamp(distance + octagon.edge_width + 0.5, 0.0, 1.0);
    let colour = mix(octagon.fill, octagon.edge, on_edge);
    return vec4(colour.rgb, colour.a * cover);
}
