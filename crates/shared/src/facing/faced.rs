//! The cell someone faces: what Collect Water fills from and Fell Tree fells.

use super::Heading;
use crate::dungeon::Level;
use crate::spot::{HALF_SPAN, Spot};

/// The nearest cell `wanted` (by its x and y) that someone at `at` facing
/// `heading` faces: its middle within `reach` units and within `half` of
/// straight ahead, or underfoot.
pub fn faced_cell(
    level: &Level,
    at: Spot,
    heading: Heading,
    reach: u64,
    half: u16,
    wanted: impl Fn(&Level, i64, i64) -> bool,
) -> Option<(i64, i64)> {
    let (x, y) = at.cell();
    let mut best: Option<(u64, (i64, i64))> = None;
    for cy in y - 2..=y + 2 {
        for cx in x - 2..=x + 2 {
            if !wanted(level, cx, cy) {
                continue;
            }
            let middle = Spot::middle_of((cx, cy));
            let distance = at.distance(middle);
            let underfoot = distance <= HALF_SPAN as u64;
            let ahead =
                Heading::towards(at, middle).is_some_and(|towards| towards.apart(heading) <= half);
            let faced = distance <= reach && (underfoot || ahead);
            if faced && best.is_none_or(|(nearest, _)| distance < nearest) {
                best = Some((distance, (cx, cy)));
            }
        }
    }
    best.map(|(_, cell)| cell)
}
