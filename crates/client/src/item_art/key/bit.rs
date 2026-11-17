//! The bit: a plate standing out from the shank's end, its wards cut as notches
//! from its edge and slots through its middle, built as blocks column by column.

use bevy::prelude::*;

use super::look::{COLUMNS, KeyLook};
use super::piece::Piece;
use super::shank::Shank;

/// How wide the bit is along the shank.
pub(super) fn bit_width(look: &KeyLook) -> f32 {
    0.3 + 0.06 * look.bit_width as f32
}

/// How far the bit reaches out from the shank's middle.
pub(super) fn bit_depth(look: &KeyLook) -> f32 {
    0.24 + 0.06 * look.bit_depth as f32
}

/// The bit's blocks, on one side or both.
pub(super) fn bit(look: &KeyLook, shank: Shank) -> Vec<Piece> {
    let depth = bit_depth(look);
    let column = (shank.bit_to - shank.bit_from) / COLUMNS as f32;
    let thick = shank.radius * 1.6;
    let inner = shank.radius * 0.5;
    let sides: &[f32] = if look.double { &[1.0, -1.0] } else { &[1.0] };
    let mut pieces = Vec::new();
    for &side in sides {
        for (index, &cut) in look.cuts.iter().enumerate() {
            let x0 = shank.bit_from + column * index as f32;
            let x1 = x0 + column;
            let spans = match cut {
                0 => vec![(inner, depth)],
                1..=3 => vec![(inner, depth * (1.0 - 0.18 * cut as f32))],
                // A slot: the edge kept, the middle cut through.
                _ => vec![(inner, depth * 0.38), (depth * 0.66, depth)],
            };
            for (z0, z1) in spans {
                let from = Vec2::new(x0, z0 * side);
                let to = Vec2::new(x1, z1 * side);
                pieces.push(Piece::block(from, to, thick));
            }
        }
    }
    pieces
}
