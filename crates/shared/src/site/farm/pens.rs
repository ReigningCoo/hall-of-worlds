//! A farm's pens (`site::pen`): a yard for poultry and a sty, by the house.

use super::super::pen::{PenKind, PenSize, add_pen};
use super::{FieldLook, Plot};
use crate::dungeon::Level;
use crate::rng::Seed;

/// Percentage chance a farm has a yard, and a sty (first guesses).
const YARD_PERCENT: u64 = 85;
const STY_PERCENT: u64 = 60;

const YARD: PenSize = PenSize {
    kind: PenKind::Yard,
    inside: (5, 4),
    margin: 2,
    dovecote_percent: 40,
};
const STY: PenSize = PenSize {
    kind: PenKind::Sty,
    inside: (3, 3),
    margin: 2,
    dovecote_percent: 0,
};

/// Fences a farm's pens on bare ground as near its door (`entrance`) as
/// they fit, from their own seed, so nothing else rolled changes. A ruined
/// farm has none: nobody keeps animals there.
pub(super) fn add_pens(
    level: &mut Level,
    seed: Seed,
    entrance: (i64, i64),
    ruin: u8,
) -> Vec<FieldLook> {
    if ruin > 0 {
        return Vec::new();
    }
    let seed = seed.derive("farm pens");
    let (x0, y0, width, height) = level.bounds();
    let area = Plot {
        x0,
        y0,
        x1: x0 + width,
        y1: y0 + height,
    };
    let mut pens = Vec::new();
    for (index, (size, percent)) in [(YARD, YARD_PERCENT), (STY, STY_PERCENT)]
        .into_iter()
        .enumerate()
    {
        let own = Seed::new(seed.at(&[index as i64]));
        if own.at(&[-1]) % 100 >= percent {
            continue;
        }
        pens.extend(add_pen(level, area, size, entrance, &[entrance], own));
    }
    pens
}
