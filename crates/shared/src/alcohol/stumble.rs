//! Stumbling (owner, 2026-11-18): drunk or worse, the odd move veers off
//! to one side, and travel stops there. Whether a move stumbles, and which
//! way, comes from a hash of the character's id, the map and the move's
//! number, so the client predicts it exactly and the server applies it to
//! whatever move it is sent.

use super::Stage;
use crate::creature::Size;
use crate::dungeon::Level;
use crate::id::Id;
use crate::movement::{Gait, Step, move_on_level};
use crate::spot::Spot;

/// Which way a stumbling move veers: an eighth or a quarter of a turn, to
/// one side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Veer {
    /// Turning the way the eight steps go round (`Step` order: east, then
    /// south-east and on), else the other way.
    pub clockwise: bool,
    /// 1 (45°) or 2 (90°).
    pub eighths: u8,
}

/// Whether `character`'s move `number` on map `map` stumbles at `stage`, and
/// which way.
pub fn stumble(character: Id, map: u32, number: u32, stage: Option<Stage>) -> Option<Veer> {
    let one_in = stage?.stumble_one_in()?;
    let hash = character
        .seed()
        .derive("stumble")
        .at(&[i64::from(map), i64::from(number)]);
    hash.is_multiple_of(one_in).then_some(Veer {
        clockwise: (hash >> 32) & 1 == 0,
        eighths: 1 + ((hash >> 33) & 1) as u8,
    })
}

/// The eight steps in turning order.
const RING: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

impl Veer {
    /// `step` turned this way.
    pub fn step(self, step: Step) -> Step {
        let at = RING
            .iter()
            .position(|&d| d == (step.dx(), step.dy()))
            .unwrap_or(0);
        let turn = usize::from(self.eighths);
        let to = match self.clockwise {
            true => (at + turn) % 8,
            false => (at + 8 - turn) % 8,
        };
        let (dx, dy) = RING[to];
        Step::new(dx, dy).unwrap_or(step)
    }

    /// The move from `from` towards `to`, turned this way, its length kept
    /// (whole units, rounded towards zero).
    pub fn turned(self, from: Spot, to: Spot) -> Spot {
        let (dx, dy) = (to.x - from.x, to.y - from.y);
        // Turning by a quarter is exact; by an eighth, about √½ of each part.
        const HALF_ROOT_TWO: i64 = 7071;
        let (x, y) = match (self.eighths, self.clockwise) {
            (2, true) => (-dy, dx),
            (2, false) => (dy, -dx),
            (_, true) => (
                (dx - dy) * HALF_ROOT_TWO / 10_000,
                (dx + dy) * HALF_ROOT_TWO / 10_000,
            ),
            (_, false) => (
                (dx + dy) * HALF_ROOT_TWO / 10_000,
                (dy - dx) * HALF_ROOT_TWO / 10_000,
            ),
        };
        from.offset(x, y)
    }
}

/// Where a stumbling move from `from` towards `to` on `level` ends: turned,
/// or turned and half as long if that is in the way, or nowhere (staying at
/// `from`) if both are; by the one rule moves on levels go by.
pub fn veered_move(
    level: &Level,
    size: Size,
    gait: Gait,
    from: Spot,
    to: Spot,
    veer: Veer,
) -> Spot {
    let turned = veer.turned(from, to);
    let half = Spot {
        x: from.x + (turned.x - from.x) / 2,
        y: from.y + (turned.y - from.y) / 2,
    };
    [turned, half]
        .into_iter()
        .find(|&spot| move_on_level(level, size, gait, from, spot).is_some())
        .unwrap_or(from)
}

/// A stumbling world map step: `step` turned (whoever takes it checks it
/// can be, staying put if not).
pub fn veered_step(step: Step, veer: Veer) -> Step {
    veer.step(step)
}
