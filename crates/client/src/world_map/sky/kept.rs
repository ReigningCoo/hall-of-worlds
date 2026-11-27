//! Whether the sky's materials need writing again. The shaders move the fade
//! between the sky's two moments and anything moving by Bevy's own clock
//! (`globals.time`), so the materials change only when something else does:
//! changing them every frame made the web build make new buffers every frame.

use super::SkyParams;

/// Light and other shades that differ by less than this look the same.
const STEP: f32 = 1.0 / 512.0;

/// How far between the sky's moments `params` puts the shader at Bevy's clock
/// `now` (wrapped, as `globals.time`), as `blend_now` in the shaders.
pub(super) fn blend_at(params: &SkyParams, now: f32) -> f32 {
    let mut since = now - params.written;
    if since < 0.0 {
        since += params.still.w;
    }
    (params.blend_from + since * params.still.z).clamp(0.0, 1.0)
}

/// Whether the shaders, given `stored`, draw what `new` would at `now`.
pub(super) fn looks_like(stored: &SkyParams, new: &SkyParams, now: f32) -> bool {
    let near = |a: f32, b: f32| (a - b).abs() < STEP;
    stored.origin_a == new.origin_a
        && stored.origin_b == new.origin_b
        && stored.extent == new.extent
        && near(blend_at(stored, now), new.blend_from)
        && stored.cloud_height == new.cloud_height
        && stored.half == new.half
        && stored.right == new.right
        && stored.towards == new.towards
        && stored.centre == new.centre
        && stored.light.abs_diff_eq(new.light, STEP)
        && stored.shown.abs_diff_eq(new.shown, STEP)
        && stored.still.x == new.still.x
        && stored.still.y == new.still.y
        && (stored.still.z - new.still.z).abs() <= new.still.z.abs() * 0.01
        && stored.still.w == new.still.w
}

#[cfg(test)]
mod tests;
