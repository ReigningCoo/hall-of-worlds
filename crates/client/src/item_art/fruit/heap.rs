//! An item of fruit as it lies: one fruit, a little heap of a handful, or a
//! bunch hanging from its stalk.

use bevy::prelude::*;

use super::super::herb::kit::Kit;
use super::body::{Placed, body};
use super::colour::{Colours, colour};
use super::group::{Body, Style, Surface, Top};
use super::surface::dress;
use super::tops::top;
use super::variety::Variety;

/// Where each of `count` fruits lies, for fruits reaching 1: a pair, three in a
/// triangle, or more spiralling out (a sunflower's pattern).
pub(super) fn places(count: usize) -> Vec<Vec2> {
    match count {
        1 => vec![Vec2::ZERO],
        2 => vec![Vec2::new(-0.85, 0.15), Vec2::new(0.85, -0.15)],
        3 => [0.0f32, 2.1, 4.2]
            .map(|a| Vec2::new(a.sin(), -a.cos()) * 1.0)
            .to_vec(),
        _ => (0..count)
            .map(|n| {
                let a = n as f32 * 2.399_963;
                Vec2::new(a.cos(), a.sin()) * (n as f32 + 0.5).sqrt() * 1.3
            })
            .collect(),
    }
}

/// One fruit, or a handful of them heaped, filling `room`.
pub(super) fn fruit(
    style: &Style,
    colours: &Colours,
    variety: &Variety,
    room: Vec2,
    kit: &mut Kit,
) {
    let places = places(style.count);
    let spread = places.iter().map(|at| at.length()).fold(0.0, f32::max);
    // Room for what sticks out: a stalk and leaf, or a pineapple's crown.
    let extra = match style.top {
        Top::Leaves => 0.65,
        Top::Stalk if style.count == 1 => 0.25,
        _ => 0.12,
    };
    let reach = if style.count == 1 {
        // One fruit lies along the picture's longer side.
        room.x.min(room.y * style.length).min(room.max_element()) / (1.0 + extra)
    } else {
        room.min_element() / (spread + 1.0 + extra)
    } * variety.size;
    let many = style.count >= 8;
    for (n, at) in places.iter().enumerate() {
        let own = 1.0 - 0.08 * (variety.wobble(n, 20) + 1.0) / 2.0;
        let upright = style.count == 1 && style.top == Top::Stalk && room.x == room.y;
        let yaw = if upright && style.body != Body::Round {
            // Its stalk end (+x) towards the top of the picture (-z).
            std::f32::consts::FRAC_PI_2 + variety.wobble(n, 21) * 0.2
        } else if style.count == 1 && style.body != Body::Round {
            variety.wobble(n, 21) * 0.15
        } else {
            variety.wobble(n, 21) * std::f32::consts::PI
        };
        // The middle of a heap lies on the others.
        let lift = if n * 4 < style.count && style.count > 3 {
            reach * 0.75
        } else {
            0.0
        };
        let shift = if style.top == Top::Leaves {
            Vec3::X * -reach * extra * 0.5
        } else {
            Vec3::ZERO
        };
        let centre = Vec3::new(at.x, lift / reach, at.y) * reach + shift;
        let placed = Placed::new(style, centre, reach * own, yaw);
        let darker;
        let colours = if style.surface == Surface::Scales {
            darker = Colours {
                skin: colours.skin * 0.7,
                ..colours.clone()
            };
            &darker
        } else {
            colours
        };
        body(&placed, colours, variety, kit);
        dress(style.surface, &placed, colours, variety, n, kit);
        let shown = match style.top {
            Top::Stalk if many => Top::Bare,
            // Big tropical fruits are picked without leaves.
            Top::Stalk if !matches!(style.surface, Surface::Plain | Surface::Speckled) => Top::Bare,
            Top::Stalk if n > 0 && variety.wobble(n, 22) < 0.0 => Top::Bare,
            top => top,
        };
        top(shown, &placed, colours, variety, n, kit);
    }
}

/// Berries hanging from a branching stalk, the bunch narrowing to its tip.
pub(super) fn bunch(colours: &Colours, variety: &Variety, room: Vec2, kit: &mut Kit) {
    let berry = kit.material(colour(colours.skin), !colours.matte);
    let stalk = kit.material(colour(colours.stalk), false);
    let size = room.min_element();
    let r = size * 0.17 * variety.size;
    let top_at = Vec3::new(0.0, r * 2.0, -size * 0.9);
    let rows = [4usize, 4, 3, 3, 2, 1];
    for (row, &across) in rows.iter().enumerate() {
        let z = -size * 0.55 + row as f32 * r * 1.6;
        for k in 0..across {
            let n = row * 4 + k;
            let x = (k as f32 - (across as f32 - 1.0) / 2.0) * r * 1.75;
            let wobble = Vec3::new(variety.wobble(n, 23), 0.0, variety.wobble(n, 24)) * r * 0.25;
            let at = Vec3::new(x, r * (1.0 + (k % 2) as f32 * 0.6), z) + wobble;
            kit.rod(&stalk, top_at, at, size * 0.012);
            let own = r * (0.92 + variety.wobble(n, 25) * 0.08);
            kit.blob(&berry, at, Vec3::splat(own), (0.0, 0.0));
        }
    }
    kit.rod(
        &stalk,
        top_at,
        top_at + Vec3::new(0.05, 0.0, -size * 0.08),
        size * 0.03,
    );
}
