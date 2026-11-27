//! Putting a newly painted map into its image, only when it changed: the
//! minimap is painted again on the clock (`REDRAW_SECONDS`), and changing the
//! image each time made the web build make a new texture each time, even when
//! nothing on it had moved.

use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;

/// Swaps `painted` (`size`, four bytes a pixel) into the image unless it already
/// holds the same; `painted` is left with whatever it held before.
pub(super) fn store(
    images: &mut Assets<Image>,
    handle: &Handle<Image>,
    size: (u32, u32),
    painted: &mut Vec<u8>,
) {
    let same = images.get(handle).is_some_and(|image| {
        image.width() == size.0
            && image.height() == size.1
            && image.data.as_deref() == Some(painted.as_slice())
    });
    if same {
        return;
    }
    let Some(mut image) = images.get_mut(handle) else {
        return;
    };
    if image.width() != size.0 || image.height() != size.1 {
        image.resize(Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        });
    }
    if let Some(data) = image.data.as_mut() {
        std::mem::swap(data, painted);
    }
}
