//! A plain glove of cloth, leather, knitting or fur on one hand, for clothes' gloves:
//! palm, fingers (one bag for mittens, stubs for fingerless gloves) and thumb.

use shared::armour::hands::Hand;
use shared::body::{Frame, Side};

use super::super::{Model, Stuff};
use super::{Axes, Shape, add_glove};

/// How much of each finger a fingerless glove covers.
const STUB: f32 = 0.3;

/// What a glove is like, for drawing it.
pub(in crate::item_art) struct GloveCut {
    pub mitten: bool,
    pub fingerless: bool,
    /// How far it stands off the skin.
    pub swell: f32,
    pub stuff: Stuff,
    /// Shaded rows that many millimetres apart, as knitting shows.
    pub rows: Option<f32>,
}

/// One glove on one hand of `frame`.
pub(in crate::item_art) fn cloth_glove(
    model: &mut Model,
    frame: &Frame,
    side: Side,
    digits: u8,
    cut: &GloveCut,
) {
    let hand = Hand::of(frame, side, digits);
    let axes = Axes::of(&hand);
    let mut shape = Shape::of(&hand, &axes);
    if cut.fingerless {
        let stub =
            |(root, tip): (bevy::prelude::Vec3, bevy::prelude::Vec3)| (root, root.lerp(tip, STUB));
        shape.fingers = shape.fingers.iter().copied().map(stub).collect();
        shape.thumb_line = stub(shape.thumb_line);
    }
    add_glove(
        model, &shape, &axes, cut.mitten, cut.swell, cut.stuff, cut.rows,
    );
}
