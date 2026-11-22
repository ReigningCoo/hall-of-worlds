//! Chairs by style: stools (three or four legs), back-stools (a low back),
//! ladder-backs (crossbars), panel-backs (a solid back), X-frames (crossed
//! legs), stone blocks (perhaps with a back) and tall bar stools with a foot
//! ring; wooden ones but bar stools may have arms.

use shared::furniture::ChairStyle;

use super::super::CELL_SIZE;
use super::seats::{SEAT_TOP, Seat};
use super::{Grain, Part};

/// How deep a chair is, from back to front, and how wide.
pub(super) const DEPTH: f32 = CELL_SIZE * 0.32;
const WIDTH: f32 = CELL_SIZE * 0.34;
/// How thick its seat is, and its legs.
const BOARD: f32 = CELL_SIZE * 0.035;
const LEG: f32 = CELL_SIZE * 0.045;
/// How high arms are, and a tall back.
const ARM_TOP: f32 = CELL_SIZE * 0.35;
const BACK_TOP: f32 = CELL_SIZE * 0.56;
/// How high a bar stool's seat is: a little under the counter's top
/// (`bar_counter::BAR_TOP`, 0.55 of a cell).
const BAR_STOOL_TOP: f32 = CELL_SIZE * 0.38;

/// A chair's parts in the seat's terms.
pub(super) fn parts(seat: &Seat, style: ChairStyle, arms: bool) -> Vec<Part> {
    let chair = Chair::of(seat);
    let mut parts = match style {
        ChairStyle::Stool => chair.stool(),
        ChairStyle::BackStool => chair.with_back(CELL_SIZE * 0.4, Back::Board),
        ChairStyle::LadderBack => chair.with_back(BACK_TOP, Back::Ladder),
        ChairStyle::PanelBack => chair.with_back(BACK_TOP, Back::Panel),
        ChairStyle::XFrame => chair.x_frame(),
        ChairStyle::StoneBlock => return chair.stone_block(),
        ChairStyle::BarStool => return chair.bar_stool(),
    };
    if arms {
        parts.extend(chair.arms());
    }
    parts
}

/// What a back is between its posts.
enum Back {
    /// One board near the top.
    Board,
    /// Three crossbars.
    Ladder,
    /// A solid panel from the seat up.
    Panel,
}

struct Chair<'a> {
    seat: &'a Seat,
    /// Its sides along the seat's length.
    left: f32,
    right: f32,
}

impl<'a> Chair<'a> {
    fn of(seat: &'a Seat) -> Self {
        let left = (seat.length - WIDTH) / 2.0;
        Self {
            seat,
            left,
            right: left + WIDTH,
        }
    }

    fn part(&self, low: (f32, f32, f32), high: (f32, f32, f32), grain: Grain, n: i64) -> Part {
        self.seat.part(low, high, grain, n)
    }

    /// The seat board.
    fn board(&self) -> Part {
        self.part(
            (0.0, SEAT_TOP - BOARD, self.left),
            (DEPTH, SEAT_TOP, self.right),
            Grain::Across,
            1,
        )
    }

    /// A leg at a corner, `back` and `left` choosing which, up to `top`.
    fn leg(&self, back: bool, left: bool, top: f32) -> Part {
        let inset = CELL_SIZE * 0.015;
        let f = if back { inset } else { DEPTH - inset - LEG };
        let u = if left {
            self.left + inset
        } else {
            self.right - inset - LEG
        };
        self.part((f, 0.0, u), (f + LEG, top, u + LEG), Grain::Up, 2)
    }

    /// Low rails between the legs, front to back on each side.
    fn rails(&self) -> Vec<Part> {
        let (low, rail) = (CELL_SIZE * 0.07, CELL_SIZE * 0.025);
        [
            self.left + 0.02 * CELL_SIZE,
            self.right - 0.02 * CELL_SIZE - rail,
        ]
        .into_iter()
        .map(|u| {
            self.part(
                (LEG, low, u),
                (DEPTH - LEG, low + rail, u + rail),
                Grain::Across,
                3,
            )
        })
        .collect()
    }

    /// A seat on three legs (two behind, one in front) or four, by a hash.
    fn stool(&self) -> Vec<Part> {
        let mut parts = vec![self.board()];
        if self.seat.hash(200).is_multiple_of(2) {
            let middle = self.seat.length / 2.0 - LEG / 2.0;
            let front = DEPTH - LEG - CELL_SIZE * 0.03;
            parts.push(self.part(
                (front, 0.0, middle),
                (front + LEG, SEAT_TOP - BOARD, middle + LEG),
                Grain::Up,
                2,
            ));
            parts.push(self.leg(true, true, SEAT_TOP - BOARD));
            parts.push(self.leg(true, false, SEAT_TOP - BOARD));
        } else {
            for back in [true, false] {
                for left in [true, false] {
                    parts.push(self.leg(back, left, SEAT_TOP - BOARD));
                }
            }
            parts.extend(self.rails());
        }
        parts
    }

    /// Four legs, the back two going on up as posts to `top`, with a back
    /// between them.
    fn with_back(&self, top: f32, back: Back) -> Vec<Part> {
        let mut parts = vec![self.board()];
        for left in [true, false] {
            parts.push(self.leg(true, left, top));
            parts.push(self.leg(false, left, SEAT_TOP - BOARD));
        }
        parts.extend(self.rails());
        let (from, to) = (self.left + LEG, self.right - LEG);
        let (f, thin) = (CELL_SIZE * 0.025, CELL_SIZE * 0.025);
        let bar = |low: f32, high: f32, n: i64| {
            self.part((f, low, from), (f + thin, high, to), Grain::Along, n)
        };
        match back {
            Back::Board => parts.push(bar(top - CELL_SIZE * 0.08, top - 0.01 * CELL_SIZE, 4)),
            Back::Ladder => {
                let step = (top - SEAT_TOP) / 4.0;
                for i in 1..=3 {
                    let y = SEAT_TOP + step * i as f32;
                    parts.push(bar(y - CELL_SIZE * 0.022, y + CELL_SIZE * 0.022, 4 + i));
                }
            }
            Back::Panel => {
                parts.push(bar(SEAT_TOP, top - 0.02 * CELL_SIZE, 4));
                // A rail across the top of the posts.
                parts.push(self.part(
                    (0.0, top, self.left),
                    (
                        f + thin + 0.01 * CELL_SIZE,
                        top + 0.03 * CELL_SIZE,
                        self.right,
                    ),
                    Grain::Along,
                    8,
                ));
            }
        }
        parts
    }

    /// A tall stool for a bar counter: a seat on four legs, with a ring of rails low down for the feet and another under the seat.
    fn bar_stool(&self) -> Vec<Part> {
        let top = BAR_STOOL_TOP - BOARD;
        let mut parts = vec![self.part(
            (0.0, top, self.left),
            (DEPTH, BAR_STOOL_TOP, self.right),
            Grain::Across,
            1,
        )];
        for back in [true, false] {
            for left in [true, false] {
                parts.push(self.leg(back, left, top));
            }
        }
        let rail = CELL_SIZE * 0.02;
        for (n, y) in [(3, CELL_SIZE * 0.12), (5, top - CELL_SIZE * 0.06)] {
            let (a, b) = (LEG, DEPTH - LEG);
            let (l, r) = (self.left + LEG, self.right - LEG);
            for u in [l - rail, r] {
                parts.push(self.part((a, y, u), (b, y + rail, u + rail), Grain::Across, n));
            }
            for f in [a - rail, b] {
                parts.push(self.part((f, y, l), (f + rail, y + rail, r), Grain::Along, n + 1));
            }
        }
        parts
    }

    /// A seat on two pairs of crossed legs, one pair at each side, with a
    /// rail joining their feet.
    fn x_frame(&self) -> Vec<Part> {
        let mut parts = vec![self.board()];
        let reach = DEPTH - LEG;
        for u in [
            self.left + 0.01 * CELL_SIZE,
            self.right - 0.01 * CELL_SIZE - LEG,
        ] {
            for front_foot in [true, false] {
                let f = if front_foot { reach } else { 0.0 };
                let lean = if front_foot { -reach } else { reach };
                parts.push(self.seat.leaning(
                    (f, 0.0, u),
                    (f + LEG, SEAT_TOP - BOARD, u + LEG),
                    Grain::Up,
                    (lean, 0.0),
                ));
            }
        }
        for f in [0.0, reach] {
            parts.push(self.part(
                (f, 0.0, self.left + LEG),
                (f + LEG, CELL_SIZE * 0.03, self.right - LEG),
                Grain::Along,
                9,
            ));
        }
        parts
    }

    /// Arms on both sides: a rail on a post at the front and one at the back.
    fn arms(&self) -> Vec<Part> {
        let mut parts = Vec::new();
        let thin = CELL_SIZE * 0.03;
        for u in [self.left, self.right - thin] {
            for f in [CELL_SIZE * 0.015, DEPTH - CELL_SIZE * 0.05] {
                parts.push(self.part(
                    (f, SEAT_TOP, u),
                    (f + thin, ARM_TOP, u + thin),
                    Grain::Up,
                    10,
                ));
            }
            parts.push(self.part(
                (0.0, ARM_TOP, u - 0.005 * CELL_SIZE),
                (
                    DEPTH,
                    ARM_TOP + CELL_SIZE * 0.025,
                    u + thin + 0.005 * CELL_SIZE,
                ),
                Grain::Across,
                11,
            ));
        }
        parts
    }

    /// A block of stone, and by a hash a slab standing behind it as a back.
    fn stone_block(&self) -> Vec<Part> {
        let mut parts = vec![self.part(
            (0.0, 0.0, self.left),
            (DEPTH, SEAT_TOP, self.right),
            Grain::Up,
            1,
        )];
        if self.seat.hash(201).is_multiple_of(2) {
            parts.push(self.part(
                (0.0, SEAT_TOP, self.left),
                (CELL_SIZE * 0.07, BACK_TOP - 0.04 * CELL_SIZE, self.right),
                Grain::Up,
                2,
            ));
        }
        parts
    }
}
