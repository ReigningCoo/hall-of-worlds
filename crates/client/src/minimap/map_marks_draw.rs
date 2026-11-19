//! The player's personal marks on the world map's maps (`common::MapMarks`),
//! over the land and pois' pictures, the near way round the planet.

use super::canvas::Canvas;
use super::mark_paint::paint_mark;
use super::projection::Projection;
use crate::common::{MapMarks, in_points, near_way_round};

/// Draws every mark `size` pixels across, on a planet `width` points round.
pub(super) fn draw(
    canvas: &mut Canvas,
    projection: &Projection,
    marks: &MapMarks,
    width: i64,
    size: f32,
) {
    let (right, bottom) = (canvas.width as f32 + size, canvas.height as f32 + size);
    for mark in &marks.0 {
        let (x, y) = in_points(mark.at);
        let x = projection.centre.0 + near_way_round(x - projection.centre.0, width);
        let at = projection.to_picture(x, y);
        if at.0 < -size || at.1 < -size || at.0 > right || at.1 > bottom {
            continue;
        }
        paint_mark(
            (mark.icon, mark.colour),
            at,
            size,
            |x, y, colour, amount| {
                if x >= 0 && y >= 0 {
                    canvas.blend(x as u32, y as u32, colour, amount);
                }
            },
        );
    }
}
