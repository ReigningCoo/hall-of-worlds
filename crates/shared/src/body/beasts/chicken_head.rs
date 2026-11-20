//! A hen's head, drawn by hand, as no mesh has one.

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

/// A hen's head, which no mesh has: drawn by hand in the measured heads' terms (in
/// thousandths of its length, belly and back up from its line): a round skull high on
/// the neck, and a short beak pointing a little down.
pub const CHICKEN_HEAD: MeasuredHead = MeasuredHead {
    source: "drawn by hand",
    length: 0,
    pitch: 0,
    rings: [
        ring(-250, 80, 200, 450),
        ring(-100, 330, 60, 650),
        ring(50, 380, 20, 700),
        ring(200, 360, 40, 650),
        ring(350, 260, 120, 520),
        ring(500, 120, 200, 380),
        ring(650, 80, 220, 320),
        ring(780, 20, 230, 265),
    ],
};
