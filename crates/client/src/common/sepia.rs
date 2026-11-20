//! Memory's colour: what was seen before but is out of sight now is drawn in
//! sepia, like an old photograph, keeping each surface's lightness so shapes
//! read the same. One place to tune it for pictures and models alike.

use bevy::color::{Color, ColorToComponents, LinearRgba, Srgba};

/// How far each channel bends from plain grey: red lifted, blue held down, so
/// the middle tones turn brown while black stays black.
const CURVE: [f32; 3] = [0.85, 1.0, 1.3];
/// The palest colour memory reaches: a cream, not white.
const PAPER: [f32; 3] = [1.0, 0.95, 0.84];

/// The sepia for a lightness from 0 (black) to 1 (white), both as seen on
/// screen (sRGB), as sRGB channels from 0 to 1.
pub fn sepia(lightness: f32) -> [f32; 3] {
    let lightness = lightness.clamp(0.0, 1.0);
    [0, 1, 2].map(|i| lightness.powf(CURVE[i]) * PAPER[i])
}

/// The same for a lightness and channels in linear light, as vertex colours
/// and material colours are.
pub fn sepia_linear(lightness: f32) -> [f32; 3] {
    let on_screen = Srgba::gamma_function(lightness.clamp(0.0, 1.0));
    let [r, g, b] = sepia(on_screen);
    LinearRgba::from(Color::srgb(r, g, b)).to_f32_array_no_alpha()
}

/// A colour's lightness (Rec. 601 weights), in whatever space it is given.
pub fn lightness([r, g, b]: [f32; 3]) -> f32 {
    0.299 * r + 0.587 * g + 0.114 * b
}

#[cfg(test)]
mod tests;
