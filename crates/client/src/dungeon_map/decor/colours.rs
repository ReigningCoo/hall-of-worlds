//! Decor's colours (linear), and the painted and woven ones it varies by.

use bevy::prelude::*;
use shared::decor::RugColour;

use super::model::srgb;

pub(super) fn dark_wood() -> Vec3 {
    srgb(0.38, 0.25, 0.15)
}

pub(super) fn light_wood() -> Vec3 {
    srgb(0.66, 0.5, 0.32)
}

pub(super) fn iron() -> Vec3 {
    srgb(0.2, 0.2, 0.22)
}

pub(super) fn steel() -> Vec3 {
    srgb(0.66, 0.68, 0.72)
}

pub(super) fn brass() -> Vec3 {
    srgb(0.8, 0.63, 0.28)
}

pub(super) fn wax() -> Vec3 {
    srgb(0.95, 0.92, 0.8)
}

pub(super) fn bone() -> Vec3 {
    srgb(0.88, 0.82, 0.68)
}

pub(super) fn paper() -> Vec3 {
    srgb(0.92, 0.88, 0.74)
}

/// A painted or woven colour and one that stands out on it, by variant.
pub(super) fn painted(variant: u8) -> (Vec3, Vec3) {
    let (main, other) = match variant % 5 {
        0 => ((0.62, 0.14, 0.12), (0.9, 0.75, 0.3)),
        1 => ((0.14, 0.22, 0.5), (0.88, 0.85, 0.75)),
        2 => ((0.2, 0.42, 0.22), (0.9, 0.78, 0.35)),
        3 => ((0.85, 0.7, 0.25), (0.15, 0.12, 0.1)),
        _ => ((0.92, 0.9, 0.84), (0.6, 0.12, 0.12)),
    };
    (
        srgb(main.0, main.1, main.2),
        srgb(other.0, other.1, other.2),
    )
}

/// A rug's main colour and the one its pattern is picked out in.
pub(super) fn rug(colour: RugColour) -> (Vec3, Vec3) {
    let (main, other) = match colour {
        RugColour::Madder => ((0.62, 0.16, 0.12), (0.85, 0.7, 0.35)),
        RugColour::Indigo => ((0.15, 0.2, 0.45), (0.8, 0.75, 0.6)),
        RugColour::Weld => ((0.82, 0.68, 0.25), (0.45, 0.2, 0.12)),
        RugColour::Green => ((0.22, 0.4, 0.22), (0.75, 0.62, 0.3)),
        RugColour::Walnut => ((0.4, 0.26, 0.15), (0.75, 0.6, 0.4)),
        RugColour::Undyed => ((0.85, 0.8, 0.68), (0.42, 0.32, 0.22)),
    };
    (
        srgb(main.0, main.1, main.2),
        srgb(other.0, other.1, other.2),
    )
}
