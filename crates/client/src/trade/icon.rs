//! The picture on the trade button: two arrows passing each other, the top one
//! pointing right and the bottom one left, drawn here in the Character
//! button's ink so there is no file to ship for it.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const SIZE: usize = 32;
const INK_BYTES: [u8; 4] = [60, 40, 20, 255];

pub(super) fn trade_icon() -> Image {
    let mut bytes = vec![0; SIZE * SIZE * 4];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let (across, down) = (x as f32 + 0.5, y as f32 + 0.5);
            // The top arrow, mirrored for the bottom one.
            if inked(across, down - 10.0) || inked(SIZE as f32 - across, down - 22.0) {
                bytes[(y * SIZE + x) * 4..][..4].copy_from_slice(&INK_BYTES);
            }
        }
    }
    Image::new(
        Extent3d {
            width: SIZE as u32,
            height: SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

/// Whether a point is on an arrow pointing right along `0`, from 4 to 28
/// across: a shaft, then a head from 20.
fn inked(across: f32, off: f32) -> bool {
    let shaft = (4.0..21.0).contains(&across) && off.abs() < 2.0;
    let head = (20.0..28.0).contains(&across) && off.abs() < (28.0 - across) * 0.75;
    shaft || head
}
