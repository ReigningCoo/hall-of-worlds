//! A journey clicked into someone squeezed past ends at the edge of their
//! circle on the side it comes from, not at their middle.

use shared::clearance::{Crowding, Other, crowded_at};
use shared::creature::Size;
use shared::spot::Spot;

/// How far outside the circles touching the route's new end is, in units.
const MARGIN: f64 = 2.0;

/// Shortens `spots` (a route, the player's spot first) whose end is inside
/// someone squeezed past to end where it first comes to them, as long as
/// `legal` allows the walk there. Left as it is if the player starts inside.
pub(super) fn stop_at_edge(
    spots: &mut Vec<Spot>,
    size: Size,
    others: &[Other],
    legal: impl Fn(Spot, Spot) -> bool,
) {
    let Some(&goal) = spots.last() else {
        return;
    };
    let inside: Vec<Other> = others
        .iter()
        .filter(|other| other.crowding == Crowding::Squeeze)
        .filter(|other| crowded_at(size, goal, std::slice::from_ref(*other)))
        .copied()
        .collect();
    if inside.is_empty() {
        return;
    }
    for end in (1..spots.len()).rev() {
        let (a, b) = (spots[end - 1], spots[end]);
        if crowded_at(size, a, &inside) {
            continue;
        }
        let entry = inside
            .iter()
            .filter_map(|other| entering(a, b, other.at, reach(size, other)))
            .min_by(|x, y| x.total_cmp(y));
        let Some(t) = entry else {
            continue;
        };
        let stop = a.offset(
            ((b.x - a.x) as f64 * t).round() as i64,
            ((b.y - a.y) as f64 * t).round() as i64,
        );
        if !crowded_at(size, stop, &inside) && legal(a, stop) {
            spots.truncate(end);
            if stop != a {
                spots.push(stop);
            }
        }
        return;
    }
}

/// How close the player's middle may come to `other`'s without overlapping,
/// with a margin.
fn reach(size: Size, other: &Other) -> f64 {
    (size.radius() + other.size.radius()) as f64 + MARGIN
}

/// How far along from `a` to `b` (0 to 1) the line first comes within
/// `radius` of `centre`, `a` being further than that.
fn entering(a: Spot, b: Spot, centre: Spot, radius: f64) -> Option<f64> {
    let (dx, dy) = ((b.x - a.x) as f64, (b.y - a.y) as f64);
    let (fx, fy) = ((a.x - centre.x) as f64, (a.y - centre.y) as f64);
    let square = dx * dx + dy * dy;
    if square == 0.0 {
        return None;
    }
    let half = fx * dx + fy * dy;
    let rest = fx * fx + fy * fy - radius * radius;
    let found = half * half - square * rest;
    if found < 0.0 {
        return None;
    }
    // Rounded back towards `a`, so the stop is never inside.
    let t = (-half - found.sqrt()) / square;
    (0.0..=1.0).contains(&t).then_some(t)
}

#[cfg(test)]
mod tests;
