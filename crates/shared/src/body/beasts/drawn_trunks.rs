//! Trunks drawn by hand for beasts no mesh has, in the measured trunks' terms
//! (`shared::body::Measured`): rings in thousandths of the height at the withers,
//! along from the hip joints, with the belly's and the back's heights from the
//! ground.

use super::super::measured::{Measured, MeasuredRing};

const fn ring(along: i32, half: i32, belly: i32, back: i32) -> MeasuredRing {
    MeasuredRing {
        along,
        half,
        belly,
        back,
    }
}

/// A rabbit's, crouched: a big round rump rising well above the shoulders, the
/// back arching down to a short chest close to the ground.
pub const RABBIT: Measured = Measured {
    source: "drawn by hand",
    hips: 900,
    shoulders: 1000,
    rings: [
        ring(-200, 1, 500, 540),
        ring(-80, 340, 130, 1080),
        ring(60, 400, 90, 1300),
        ring(220, 400, 100, 1350),
        ring(380, 360, 140, 1280),
        ring(540, 310, 190, 1170),
        ring(700, 280, 230, 1070),
        ring(860, 270, 250, 1000),
        ring(980, 240, 300, 990),
        ring(1080, 120, 520, 960),
    ],
};
