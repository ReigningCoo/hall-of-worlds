//! Finding room for a pen and building it.

use super::super::farm::{FieldLook, Plot, fence_field};
use super::super::town::Rect;
use super::{PenKind, PenLook};
use crate::dungeon::{Cell, Level};
use crate::rng::Seed;

/// How many of the nearest places that fit a pen may be chosen from.
const NEAREST: u64 = 6;

/// What pen to make and how much room to leave round it.
#[derive(Clone, Copy, Debug)]
pub(in crate::site) struct PenSize {
    pub kind: PenKind,
    /// Its cells inside the fence, across and down.
    pub inside: (i64, i64),
    /// How much walkable ground there must be all round its fence (at least
    /// 1, so nothing is cut off: whatever went across its place goes round).
    pub margin: i64,
    /// Percentage chance of a dovecote in a yard.
    pub dovecote_percent: u64,
}

/// Puts a pen on bare ground within `area` (its far edges excluded), at one
/// of the places nearest `near` that fit, chosen by `seed`, its gate on the
/// side nearest `near`; never within 2 cells of anything in `avoid`. Gives
/// its field, or `None` if there is no room.
pub(in crate::site) fn add_pen(
    level: &mut Level,
    area: Plot,
    size: PenSize,
    near: (i64, i64),
    avoid: &[(i64, i64)],
    seed: Seed,
) -> Option<FieldLook> {
    let (w, h) = (size.inside.0 + 2, size.inside.1 + 2);
    let mut places: Vec<Plot> = (area.y0..area.y1 - h + 1)
        .flat_map(|y0| (area.x0..area.x1 - w + 1).map(move |x0| (x0, y0)))
        .map(|(x0, y0)| Plot {
            x0,
            y0,
            x1: x0 + w,
            y1: y0 + h,
        })
        .filter(|&plot| fits(level, plot, size.margin, avoid))
        .collect();
    let middle = |plot: &Plot| (plot.x0 * 2 + w, plot.y0 * 2 + h);
    let distance = |plot: &Plot| {
        let (x, y) = middle(plot);
        (x - near.0 * 2).pow(2) + (y - near.1 * 2).pow(2)
    };
    places.sort_by_key(|plot| (distance(plot), plot.y0, plot.x0));
    let choices = (places.len() as u64).min(NEAREST);
    if choices == 0 {
        return None;
    }
    let plot = places[(seed.at(&[0]) % choices) as usize];
    let outside = fence_field(level, &mut seed.rng(), plot, near, 0);
    Some(field_of(plot, outside, size, seed))
}

/// Whether a pen's plot fits: bare ground, with `margin` of walkable ground
/// (no gate, which needs its own cell before it) all round inside the level,
/// a cell more before its edge, and clear of `avoid`.
fn fits(level: &Level, plot: Plot, margin: i64, avoid: &[(i64, i64)]) -> bool {
    let (min_x, min_y, width, height) = level.bounds();
    let reach = margin + 1;
    if plot.x0 - reach < min_x
        || plot.y0 - reach < min_y
        || plot.x1 + reach > min_x + width
        || plot.y1 + reach > min_y + height
    {
        return false;
    }
    let near = |x: i64, y: i64, by: i64| {
        avoid
            .iter()
            .any(|&(ax, ay)| (ax - x).abs() <= by && (ay - y).abs() <= by)
    };
    for y in plot.y0 - margin..plot.y1 + margin {
        for x in plot.x0 - margin..plot.x1 + margin {
            let inside = (plot.x0..plot.x1).contains(&x) && (plot.y0..plot.y1).contains(&y);
            let ok = if inside {
                level.cell(x, y) == Cell::Ground && level.is_walkable(x, y) && !near(x, y, 2)
            } else {
                level.is_walkable(x, y) && level.cell(x, y) != Cell::FieldGate
            };
            if !ok {
                return false;
            }
        }
    }
    true
}

/// The field of a pen fenced on `plot`, its gate opening onto `outside`:
/// the shelter against the far fence at one end, a yard's dovecote perhaps
/// at the other.
fn field_of(plot: Plot, outside: (i64, i64), size: PenSize, seed: Seed) -> FieldLook {
    let rect = Rect {
        x: plot.x0 + 1,
        y: plot.y0 + 1,
        width: plot.x1 - plot.x0 - 2,
        height: plot.y1 - plot.y0 - 2,
    };
    // The gate's side, and the back opposite it.
    let gate = if outside.1 < plot.y0 {
        0
    } else if outside.0 >= plot.x1 {
        1
    } else if outside.1 >= plot.y1 {
        2
    } else {
        3
    };
    let back = (gate + 2) % 4;
    let flip = seed.at(&[1]) % 2 == 1;
    let (shelter, far_end) = shelter_at(rect, back, flip);
    let dovecote = (size.kind == PenKind::Yard && seed.at(&[2]) % 100 < size.dovecote_percent)
        .then_some(far_end);
    FieldLook {
        rect,
        livestock: true,
        variety: seed.at(&[3]) as u32,
        orchard: false,
        pen: Some(PenLook {
            kind: size.kind,
            shelter,
            back,
            dovecote,
        }),
    }
}

/// A shelter of 2 cells along the `back` side of `rect`, 1 deep, at one end
/// (the other if `flip`), and the cell at the far end of that side.
fn shelter_at(rect: Rect, back: u8, flip: bool) -> (Rect, (i64, i64)) {
    let Rect {
        x,
        y,
        width,
        height,
    } = rect;
    let along_x = back.is_multiple_of(2);
    let length = if along_x { width } else { height };
    let (start, end) = if flip {
        (length - 2, 0)
    } else {
        (0, length - 1)
    };
    let row = match back {
        0 => y,
        1 => x + width - 1,
        2 => y + height - 1,
        _ => x,
    };
    if along_x {
        let shelter = Rect {
            x: x + start,
            y: row,
            width: 2,
            height: 1,
        };
        (shelter, (x + end, row))
    } else {
        let shelter = Rect {
            x: row,
            y: y + start,
            width: 1,
            height: 2,
        };
        (shelter, (row, y + end))
    }
}
