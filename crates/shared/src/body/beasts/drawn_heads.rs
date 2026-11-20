//! Heads drawn by hand for beasts no mesh has, in the measured heads' terms: rings
//! in thousandths of the head's length from the back of the skull to the nose, a
//! ring's `belly` and `back` how far up from the head's line its underside and top
//! are (an underside below the line is negative).

use super::super::measured::MeasuredRing;
use super::super::measured_heads::MeasuredHead;

const fn ring(along: i32, half: i32, belly: i32, back: i32) -> MeasuredRing {
    MeasuredRing {
        along,
        half,
        belly,
        back,
    }
}

/// A sheep's or a goat's: a narrow skull, a long face with a gently rounded
/// (Roman) nose, and a narrow muzzle.
pub const SHEEP_HEAD: MeasuredHead = MeasuredHead {
    source: "drawn by hand",
    length: 0,
    pitch: 0,
    rings: [
        ring(-120, 10, -40, 40),
        ring(20, 250, -220, 290),
        ring(160, 235, -230, 295),
        ring(300, 195, -230, 250),
        ring(440, 155, -220, 205),
        ring(580, 120, -200, 160),
        ring(720, 95, -170, 110),
        ring(850, 60, -110, 55),
    ],
};

/// A pig's: broad jowls low at the back, a straight top tapering along the snout
/// to a flat round disc for a nose.
pub const PIG_HEAD: MeasuredHead = MeasuredHead {
    source: "drawn by hand",
    length: 0,
    pitch: 0,
    rings: [
        ring(-140, 20, -60, 60),
        ring(0, 330, -360, 320),
        ring(150, 320, -340, 290),
        ring(300, 260, -270, 235),
        ring(450, 200, -205, 180),
        ring(600, 160, -160, 140),
        ring(750, 138, -135, 122),
        ring(860, 132, -128, 116),
    ],
};
