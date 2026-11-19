//! Painting a mark (owner's design): a black disc with a thin edge and the icon
//! inside, both in the mark's colour, so it is plainly the player's own and
//! unlike pois' pictures (black-edged) and quests' "!" and "?".

use shared::map_mark::{MarkColour, MarkIcon};

use super::mark_shapes::icon_distance;

/// How far across a mark is on the maps, in the screen's pixels at the maps' own
/// zoom (times the full map's zoom, as pois' pictures are).
pub const MARK_PIXELS: f32 = 16.0;
/// The edge's thickness, against the mark's size.
const EDGE: f32 = 0.08;
/// The icon's half size, against the mark's.
const ICON: f32 = 0.3;

/// Paints a mark `size` pixels across centred on `centre`, through `blend(x,
/// y, colour, amount)` for each pixel it touches (`amount` 0 to 1).
pub fn paint_mark(
    (icon, colour): (MarkIcon, MarkColour),
    centre: (f32, f32),
    size: f32,
    mut blend: impl FnMut(i64, i64, [u8; 3], f32),
) {
    let radius = size / 2.0;
    let edge = (size * EDGE).max(1.0);
    let scale = size * ICON;
    let rgb = colour.rgb();
    let low = |at: f32| (at - radius - 1.0).floor() as i64;
    let high = |at: f32| (at + radius + 1.0).ceil() as i64;
    for y in low(centre.1)..high(centre.1) {
        for x in low(centre.0)..high(centre.0) {
            let (dx, dy) = (x as f32 + 0.5 - centre.0, y as f32 + 0.5 - centre.1);
            let from_middle = dx.hypot(dy);
            let disc = (0.5 - (from_middle - radius)).clamp(0.0, 1.0);
            if disc <= 0.0 {
                continue;
            }
            blend(x, y, [0, 0, 0], disc);
            let ring =
                (0.5 - ((from_middle - radius + edge / 2.0).abs() - edge / 2.0)).clamp(0.0, 1.0);
            let inside = 0.5 - icon_distance(icon, dx / scale, dy / scale) * scale;
            let amount = ring.max(inside.clamp(0.0, 1.0)).min(disc);
            if amount > 0.0 {
                blend(x, y, rgb, amount);
            }
        }
    }
}

/// A mark alone, `size` pixels square, as sRGB red, green, blue, alpha bytes:
/// for the popup's buttons.
pub fn mark_pixels(look: (MarkIcon, MarkColour), size: u32) -> Vec<u8> {
    let mut pixels = vec![0; (size * size * 4) as usize];
    let middle = size as f32 / 2.0;
    paint_mark(
        look,
        (middle, middle),
        size as f32 - 2.0,
        |x, y, rgb, amount| {
            blend_into(&mut pixels, size, (x, y), rgb, amount);
        },
    );
    pixels
}

/// A plain disc of a colour, edged in black, for the popup's colour buttons.
pub fn swatch_pixels(colour: MarkColour, size: u32) -> Vec<u8> {
    let mut pixels = vec![0; (size * size * 4) as usize];
    let middle = size as f32 / 2.0;
    let radius = middle - 1.0;
    for y in 0..size as i64 {
        for x in 0..size as i64 {
            let from = (x as f32 + 0.5 - middle).hypot(y as f32 + 0.5 - middle);
            let disc = (0.5 - (from - radius)).clamp(0.0, 1.0);
            blend_into(&mut pixels, size, (x, y), [0, 0, 0], disc);
            let inner = (0.5 - (from - radius + 2.0)).clamp(0.0, 1.0);
            blend_into(&mut pixels, size, (x, y), colour.rgb(), inner);
        }
    }
    pixels
}

/// Mixes `rgb` over one pixel of a square picture `size` across.
fn blend_into(pixels: &mut [u8], size: u32, (x, y): (i64, i64), rgb: [u8; 3], amount: f32) {
    if x < 0 || y < 0 || x >= i64::from(size) || y >= i64::from(size) || amount <= 0.0 {
        return;
    }
    let at = ((y as u32 * size + x as u32) * 4) as usize;
    for (pixel, new) in pixels[at..at + 3].iter_mut().zip(rgb) {
        let old = f32::from(*pixel);
        *pixel = (old + (f32::from(new) - old) * amount).round() as u8;
    }
    let alpha = f32::from(pixels[at + 3]) / 255.0;
    pixels[at + 3] = ((alpha + (1.0 - alpha) * amount) * 255.0).round() as u8;
}
