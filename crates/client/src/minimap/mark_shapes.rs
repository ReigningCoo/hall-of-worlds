//! The marks' icons as distance functions (negative inside), in the icon's own
//! measure: -1 to 1 across and down, down being +y. Drawn by `paint`, for the
//! maps and the popup alike, as the maps have no icon files.

use shared::map_mark::MarkIcon;

/// How far `(x, y)` is from `icon`'s shape, negative inside.
pub fn icon_distance(icon: MarkIcon, x: f32, y: f32) -> f32 {
    let at = (x, y);
    match icon {
        MarkIcon::Flag => {
            let pole = line(at, (-0.5, -0.85), (-0.5, 0.9), 0.1);
            let cloth = polygon(at, &[(-0.5, -0.85), (0.75, -0.45), (-0.5, -0.05)]);
            pole.min(cloth)
        }
        MarkIcon::House => polygon(
            at,
            &[
                (-0.85, 0.0),
                (0.0, -0.8),
                (0.85, 0.0),
                (0.6, 0.0),
                (0.6, 0.8),
                (0.15, 0.8),
                (0.15, 0.35),
                (-0.15, 0.35),
                (-0.15, 0.8),
                (-0.6, 0.8),
                (-0.6, 0.0),
            ],
        ),
        MarkIcon::Cave => {
            // A mound standing on y = 0.65, with a round way in.
            let hill = circle(at, (0.0, 0.65), 0.9).max(y - 0.65);
            hill.max(-circle(at, (0.0, 0.65), 0.42))
        }
        MarkIcon::Mine => {
            let head = line(at, (-0.75, -0.2), (0.0, -0.65), 0.12).min(line(
                at,
                (0.0, -0.65),
                (0.75, -0.2),
                0.12,
            ));
            head.min(line(at, (0.0, -0.65), (0.0, 0.85), 0.11))
        }
        MarkIcon::Farm => {
            let mut shape = line(at, (0.0, 0.9), (0.0, -0.85), 0.09);
            for top in [-0.55, -0.15, 0.25] {
                shape = shape
                    .min(line(at, (0.0, top + 0.3), (-0.42, top), 0.1))
                    .min(line(at, (0.0, top + 0.3), (0.42, top), 0.1));
            }
            shape
        }
        MarkIcon::Orchard => {
            let crown = circle(at, (0.0, -0.25), 0.62);
            crown.min(polygon(
                at,
                &[(-0.14, 0.2), (0.14, 0.2), (0.14, 0.9), (-0.14, 0.9)],
            ))
        }
        MarkIcon::Bag => {
            let body = circle(at, (0.0, 0.3), 0.6);
            let neck = polygon(
                at,
                &[(-0.2, -0.25), (0.2, -0.25), (0.42, -0.75), (-0.42, -0.75)],
            );
            body.min(neck)
        }
        MarkIcon::Logs => {
            let ends = [(-0.42, 0.38), (0.42, 0.38), (0.0, -0.36)];
            let wood = (ends.iter())
                .map(|&end| circle(at, end, 0.4))
                .fold(f32::MAX, f32::min);
            let rings = (ends.iter())
                .map(|&end| circle(at, end, 0.1))
                .fold(f32::MAX, f32::min);
            wood.max(-rings)
        }
        MarkIcon::Camp => {
            let tent = polygon(at, &[(-0.9, 0.8), (0.0, -0.85), (0.9, 0.8)]);
            let door = polygon(at, &[(-0.28, 0.85), (0.0, 0.15), (0.28, 0.85)]);
            tent.max(-door)
        }
        MarkIcon::Danger => {
            let head = circle(at, (0.0, -0.2), 0.62);
            let jaw = polygon(at, &[(-0.32, 0.2), (0.32, 0.2), (0.32, 0.8), (-0.32, 0.8)]);
            let eyes = circle(at, (-0.25, -0.2), 0.17).min(circle(at, (0.25, -0.2), 0.17));
            let teeth = line(at, (0.0, 0.45), (0.0, 0.85), 0.05);
            head.min(jaw).max(-eyes).max(-teeth)
        }
    }
}

/// A round-ended stroke from `a` to `b`, `half` as thick either side.
fn line(at: (f32, f32), a: (f32, f32), b: (f32, f32), half: f32) -> f32 {
    let (along_x, along_y) = (b.0 - a.0, b.1 - a.1);
    let (to_x, to_y) = (at.0 - a.0, at.1 - a.1);
    let length = along_x * along_x + along_y * along_y;
    let share = ((to_x * along_x + to_y * along_y) / length).clamp(0.0, 1.0);
    (to_x - along_x * share).hypot(to_y - along_y * share) - half
}

fn circle(at: (f32, f32), centre: (f32, f32), radius: f32) -> f32 {
    (at.0 - centre.0).hypot(at.1 - centre.1) - radius
}

/// Distance to a polygon, negative inside.
fn polygon(at: (f32, f32), corners: &[(f32, f32)]) -> f32 {
    let mut nearest = f32::MAX;
    let mut inside = false;
    let mut previous = corners[corners.len() - 1];
    for &corner in corners {
        nearest = nearest.min(line(at, previous, corner, 0.0));
        if (corner.1 > at.1) != (previous.1 > at.1)
            && at.0
                < corner.0 + (at.1 - corner.1) / (previous.1 - corner.1) * (previous.0 - corner.0)
        {
            inside = !inside;
        }
        previous = corner;
    }
    if inside { -nearest } else { nearest }
}
