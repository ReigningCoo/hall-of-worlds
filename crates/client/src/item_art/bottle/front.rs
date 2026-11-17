//! `Front`: the side of the liquid facing the camera, where its bubbles, fizz
//! and swirls are put so they show whatever the bottle's shape.

use bevy::prelude::*;

use super::surface::Surface;

/// How squarely a point must face the camera to count as the front.
const FACING: f32 = 0.55;

pub(super) struct Front {
    /// Points on the front and the way out of the liquid there.
    points: Vec<(Vec3, Vec3)>,
    /// The picture's right and up, in the bottle's space.
    right: Vec3,
    up: Vec3,
    /// The front's corners across the picture.
    low: Vec2,
    high: Vec2,
}

impl Front {
    /// The front of these liquid surfaces, as seen from `view` (the way to the
    /// camera in the bottle's space), leaving out the top and a margin at the
    /// bottom and top.
    pub(super) fn of(surfaces: &[Surface], view: Vec3, bottom: f32, top: f32) -> Option<Self> {
        let up = (Vec3::Y - view * Vec3::Y.dot(view)).normalize();
        let right = up.cross(view);
        let margin = (top - bottom) * 0.08;
        let mut points = Vec::new();
        for surface in surfaces {
            let (positions, normals, _) = surface.triangles();
            points.extend(positions.into_iter().zip(normals).filter(|(p, n)| {
                n.dot(view) > FACING
                    && n.y.abs() < 0.7
                    && p.y > bottom + margin
                    && p.y < top - margin
            }));
        }
        let (mut low, mut high) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
        for (p, _) in &points {
            let on = Vec2::new(p.dot(right), p.dot(up));
            low = low.min(on);
            high = high.max(on);
        }
        (!points.is_empty()).then_some(Self {
            points,
            right,
            up,
            low,
            high,
        })
    }

    /// The front's width and height across the picture.
    pub(super) fn span(&self) -> Vec2 {
        self.high - self.low
    }

    /// The picture's right and up, in the bottle's space.
    pub(super) fn axes(&self) -> (Vec3, Vec3) {
        (self.right, self.up)
    }

    /// A point `depth` in from the front, at `share` across and up it (0 to 1),
    /// or none if the front does not reach there (the shape narrows).
    pub(super) fn at(&self, share: Vec2, depth: f32) -> Option<Vec3> {
        let target = self.low + (self.high - self.low) * share;
        let on = |p: &Vec3| Vec2::new(p.dot(self.right), p.dot(self.up));
        let (point, normal) = self.points.iter().min_by(|a, b| {
            on(&a.0)
                .distance(target)
                .total_cmp(&on(&b.0).distance(target))
        })?;
        let gap = target - on(point);
        // Too far from any point of the front: off its edge.
        if gap.length() > self.span().max_element() * 0.08 {
            return None;
        }
        Some(*point + self.right * gap.x + self.up * gap.y - *normal * depth)
    }
}
